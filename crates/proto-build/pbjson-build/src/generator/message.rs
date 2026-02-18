//! This module contains the code to generate Serialize and Deserialize
//! implementations for message types
//!
//! The implementation follows the proto3 [JSON mapping][1] with the default options
//!
//! Importantly:
//! - numeric types can be decoded from either a string or number
//! - 32-bit integers and floats are encoded as numbers
//! - 64-bit integers are encoded as strings
//! - repeated fields are encoded as arrays
//! - bytes are base64 encoded
//! - messages and maps are encoded as objects
//! - fields are lowerCamelCase except where overridden by the proto definition
//! - default values are not emitted on encode
//! - unrecognised fields error on decode
//!
//! Note: This will not generate code to correctly serialize/deserialize well-known-types
//! such as google.protobuf.Any, google.protobuf.Duration, etc... conversions for these
//! special-cased messages will need to be manually implemented. Once done so, however,
//! any messages containing these types will serialize/deserialize correctly
//!
//! [1]: https://developers.google.com/protocol-buffers/docs/proto3#json

use proc_macro2::TokenStream;
use quote::quote;

use crate::descriptor::TypePath;
use crate::escape::escape_type;
use crate::escape::ident_from_escaped;
use crate::message::Field;
use crate::message::FieldModifier;
use crate::message::FieldType;
use crate::message::Message;
use crate::message::OneOf;
use crate::message::ScalarType;
use crate::resolver::Resolver;

// ---------------------------------------------------------------------------
// Ident helpers
// ---------------------------------------------------------------------------

/// Convert a field's rust_field_name (which may have r# prefix) to an Ident.
fn field_ident(field: &Field) -> proc_macro2::Ident {
    ident_from_escaped(&field.rust_field_name())
}

/// Convert a field's rust_type_name (CamelCase, possibly with _ suffix) to an Ident.
fn type_name_ident(field: &Field) -> proc_macro2::Ident {
    let name = field.rust_type_name();
    proc_macro2::Ident::new(&name, proc_macro2::Span::call_site())
}

/// Create a `field_name__` variable ident from an escaped field name.
fn var_ident(escaped_name: &str) -> proc_macro2::Ident {
    let raw = escaped_name.strip_prefix("r#").unwrap_or(escaped_name);
    proc_macro2::Ident::new(&format!("{raw}__"), proc_macro2::Span::call_site())
}

// ---------------------------------------------------------------------------
// Variable – abstracts over self.field vs v/*v access patterns
// ---------------------------------------------------------------------------

struct Variable {
    /// A reference to the field's value (e.g. `&self.field` or `v`)
    as_ref: TokenStream,
    /// The field's value without `&` (e.g. `self.field` or `*v`)
    as_unref: TokenStream,
    /// The raw expression (e.g. `self.field` or `v`)
    raw: TokenStream,
}

fn self_variable(field: &Field) -> Variable {
    let ident = field_ident(field);
    Variable {
        as_ref: quote!(&self.#ident),
        as_unref: quote!(self.#ident),
        raw: quote!(self.#ident),
    }
}

fn v_variable() -> Variable {
    Variable {
        as_ref: quote!(v),
        as_unref: quote!(*v),
        raw: quote!(v),
    }
}

// ---------------------------------------------------------------------------
// Well-known type helpers
// ---------------------------------------------------------------------------

fn well_known_type_serializer(type_path: &TypePath) -> Option<TokenStream> {
    let type_name = type_path.to_string();
    match type_name.as_str() {
        "google.protobuf.FieldMask" => Some(quote!(crate::_serde::FieldMaskSerializer)),
        "google.protobuf.Timestamp" => Some(quote!(crate::_serde::TimestampSerializer)),
        "google.protobuf.Duration" => Some(quote!(crate::_serde::DurationSerializer)),
        "google.protobuf.Value" => Some(quote!(crate::_serde::ValueSerializer)),
        "google.protobuf.Any" => Some(quote!(crate::_serde::AnySerializer)),
        "google.protobuf.Empty" => Some(quote!(crate::_serde::EmptySerializer)),
        _ => None,
    }
}

fn well_known_type_deserializer(type_path: &TypePath) -> Option<TokenStream> {
    let type_name = type_path.to_string();
    match type_name.as_str() {
        "google.protobuf.FieldMask" => Some(quote!(crate::_serde::FieldMaskDeserializer)),
        "google.protobuf.Timestamp" => Some(quote!(crate::_serde::TimestampDeserializer)),
        "google.protobuf.Duration" => Some(quote!(crate::_serde::DurationDeserializer)),
        "google.protobuf.Value" => Some(quote!(crate::_serde::ValueDeserializer)),
        "google.protobuf.Any" => Some(quote!(crate::_serde::AnyDeserializer)),
        "google.protobuf.Empty" => Some(quote!(crate::_serde::EmptyDeserializer)),
        _ => None,
    }
}

fn override_deserializer(scalar: ScalarType) -> Option<TokenStream> {
    match scalar {
        ScalarType::Bytes => Some(quote!(crate::_serde::BytesDeserialize<_>)),
        _ if scalar.is_numeric() => Some(quote!(crate::_serde::NumberDeserialize<_>)),
        _ => None,
    }
}

// ===========================================================================
// Serialization
// ===========================================================================

fn field_empty_predicate(member: &Field, emit_fields: bool) -> TokenStream {
    if emit_fields {
        return quote!(true);
    }

    let ident = field_ident(member);

    match (&member.field_type, &member.field_modifier) {
        (_, FieldModifier::Required) => unreachable!(),
        (_, FieldModifier::Repeated)
        | (FieldType::Map(_, _), _)
        | (FieldType::Scalar(ScalarType::String), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::Bytes), FieldModifier::UseDefault) => {
            quote!(!self.#ident.is_empty())
        }
        (_, FieldModifier::Optional) | (FieldType::Message(_), _) => {
            quote!(self.#ident.is_some())
        }
        (FieldType::Scalar(ScalarType::F64), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::F32), FieldModifier::UseDefault) => {
            quote!(self.#ident != 0.)
        }
        (FieldType::Scalar(ScalarType::Bool), FieldModifier::UseDefault) => {
            quote!(self.#ident)
        }
        (FieldType::Enum(_), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::I64), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::I32), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::U32), FieldModifier::UseDefault)
        | (FieldType::Scalar(ScalarType::U64), FieldModifier::UseDefault) => {
            quote!(self.#ident != 0)
        }
    }
}

fn decode_variant(resolver: &Resolver<'_>, value: &TokenStream, path: &TypePath) -> TokenStream {
    let enum_type = resolver.rust_type_token(path);
    quote! {
        #enum_type::try_from(#value)
            .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", #value)))
    }
}

fn serialize_scalar_variable(
    scalar: ScalarType,
    field_modifier: FieldModifier,
    variable: &Variable,
    field_name: &str,
) -> TokenStream {
    let as_ref = &variable.as_ref;
    let raw = &variable.raw;

    match scalar {
        ScalarType::I64 | ScalarType::U64 => match field_modifier {
            FieldModifier::Repeated => quote! {
                struct_ser.serialize_field(#field_name, &#raw.iter().map(ToString::to_string).collect::<Vec<_>>())?;
            },
            _ => quote! {
                #[allow(clippy::needless_borrow)]
                #[allow(clippy::needless_borrows_for_generic_args)]
                struct_ser.serialize_field(#field_name, ToString::to_string(&#raw).as_str())?;
            },
        },
        ScalarType::Bytes => match field_modifier {
            FieldModifier::Repeated => quote! {
                struct_ser.serialize_field(#field_name, &#raw.iter().map(crate::_serde::base64::encode).collect::<Vec<_>>())?;
            },
            _ => quote! {
                #[allow(clippy::needless_borrow)]
                #[allow(clippy::needless_borrows_for_generic_args)]
                struct_ser.serialize_field(#field_name, crate::_serde::base64::encode(&#raw).as_str())?;
            },
        },
        _ => quote! {
            struct_ser.serialize_field(#field_name, #as_ref)?;
        },
    }
}

fn serialize_variable(
    resolver: &Resolver<'_>,
    field: &Field,
    variable: &Variable,
    preserve_proto_field_names: bool,
) -> TokenStream {
    let json_name = field.json_name();
    let field_name = if preserve_proto_field_names {
        field.name.clone()
    } else {
        json_name
    };

    match &field.field_type {
        FieldType::Scalar(scalar) => {
            serialize_scalar_variable(*scalar, field.field_modifier, variable, &field_name)
        }
        FieldType::Enum(path) => {
            let as_unref = &variable.as_unref;
            let raw = &variable.raw;

            let decode = match field.field_modifier {
                FieldModifier::Repeated => {
                    let dv = decode_variant(resolver, &quote!(v), path);
                    quote! {
                        let v = #raw.iter().cloned().map(|v| {
                            #dv
                        }).collect::<std::result::Result<Vec<_>, _>>()?;
                    }
                }
                _ => {
                    let dv = decode_variant(resolver, as_unref, path);
                    quote! {
                        let v = #dv?;
                    }
                }
            };

            quote! {
                #decode
                struct_ser.serialize_field(#field_name, &v)?;
            }
        }
        FieldType::Map(_, value_type)
            if matches!(
                value_type.as_ref(),
                FieldType::Scalar(ScalarType::I64)
                    | FieldType::Scalar(ScalarType::U64)
                    | FieldType::Scalar(ScalarType::Bytes)
                    | FieldType::Enum(_)
            ) =>
        {
            let raw = &variable.raw;
            let map_transform = match value_type.as_ref() {
                FieldType::Scalar(ScalarType::I64) | FieldType::Scalar(ScalarType::U64) => {
                    quote! {
                        let v: std::collections::BTreeMap<_, _> = #raw.iter()
                            .map(|(k, v)| (k, v.to_string())).collect();
                    }
                }
                FieldType::Scalar(ScalarType::Bytes) => {
                    quote! {
                        let v: std::collections::BTreeMap<_, _> = #raw.iter()
                            .map(|(k, v)| (k, crate::_serde::base64::encode(v))).collect();
                    }
                }
                FieldType::Enum(path) => {
                    let enum_type = resolver.rust_type_token(path);
                    quote! {
                        let v: std::collections::BTreeMap<_, _> = #raw.iter()
                            .map(|(k, v)| {
                                let v = #enum_type::try_from(*v)
                                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                                Ok((k, v))
                            }).collect::<std::result::Result<_, _>>()?;
                    }
                }
                _ => unreachable!(),
            };

            quote! {
                #map_transform
                struct_ser.serialize_field(#field_name, &v)?;
            }
        }
        FieldType::Message(type_name) if well_known_type_serializer(type_name).is_some() => {
            let wkt_ser = well_known_type_serializer(type_name).unwrap();
            let as_ref = &variable.as_ref;
            let raw = &variable.raw;

            match field.field_modifier {
                FieldModifier::Repeated => quote! {
                    struct_ser.serialize_field(#field_name, &#raw.iter().map(#wkt_ser).collect::<Vec<_>>())?;
                },
                _ => quote! {
                    struct_ser.serialize_field(#field_name, &#wkt_ser(#as_ref))?;
                },
            }
        }
        _ => {
            let as_ref = &variable.as_ref;
            quote! {
                struct_ser.serialize_field(#field_name, #as_ref)?;
            }
        }
    }
}

fn serialize_field(
    resolver: &Resolver<'_>,
    field: &Field,
    emit_fields: bool,
    preserve_proto_field_names: bool,
) -> TokenStream {
    let self_var = self_variable(field);

    match &field.field_modifier {
        FieldModifier::Required => {
            serialize_variable(resolver, field, &self_var, preserve_proto_field_names)
        }
        FieldModifier::Optional => {
            let v_var = v_variable();
            let inner = serialize_variable(resolver, field, &v_var, preserve_proto_field_names);
            let unref = &self_var.as_unref;
            quote! {
                if let Some(v) = #unref.as_ref() {
                    #inner
                }
            }
        }
        FieldModifier::Repeated | FieldModifier::UseDefault => {
            let predicate = field_empty_predicate(field, emit_fields);
            let inner = serialize_variable(resolver, field, &self_var, preserve_proto_field_names);
            quote! {
                if #predicate {
                    #inner
                }
            }
        }
    }
}

fn serialize_one_of(
    resolver: &Resolver<'_>,
    one_of: &OneOf,
    preserve_proto_field_names: bool,
) -> TokenStream {
    let one_of_field = ident_from_escaped(&one_of.rust_field_name());
    let one_of_type = resolver.rust_type_token(&one_of.path);

    let arms: Vec<TokenStream> = one_of
        .fields
        .iter()
        .map(|field| {
            let variant = type_name_ident(field);
            let v_var = v_variable();
            let inner = serialize_variable(resolver, field, &v_var, preserve_proto_field_names);
            quote! {
                #one_of_type::#variant(v) => {
                    #inner
                }
            }
        })
        .collect();

    quote! {
        if let Some(v) = self.#one_of_field.as_ref() {
            match v {
                #(#arms)*
            }
        }
    }
}

fn message_serialize_body(
    resolver: &Resolver<'_>,
    message: &Message,
    emit_fields: bool,
    preserve_proto_field_names: bool,
) -> TokenStream {
    let message_name = message.path.to_string();

    let required_len = message
        .fields
        .iter()
        .filter(|m| m.field_modifier.is_required())
        .count();

    let field_len_checks: Vec<TokenStream> = message
        .fields
        .iter()
        .filter(|f| !f.field_modifier.is_required())
        .map(|f| {
            let pred = field_empty_predicate(f, emit_fields);
            quote! { if #pred { len += 1; } }
        })
        .collect();

    let one_of_len_checks: Vec<TokenStream> = message
        .one_ofs
        .iter()
        .map(|o| {
            let ident = ident_from_escaped(&o.rust_field_name());
            quote! { if self.#ident.is_some() { len += 1; } }
        })
        .collect();

    let has_optional = required_len != message.fields.len() || !message.one_ofs.is_empty();
    let has_fields = !message.fields.is_empty() || !message.one_ofs.is_empty();

    let len_decl = if has_optional {
        quote!(let mut len = #required_len;)
    } else {
        quote!(let len = #required_len;)
    };

    let struct_ser_decl = if has_fields {
        quote!(let mut struct_ser = serializer.serialize_struct(#message_name, len)?;)
    } else {
        quote!(let struct_ser = serializer.serialize_struct(#message_name, len)?;)
    };

    let field_sers: Vec<TokenStream> = message
        .fields
        .iter()
        .map(|f| serialize_field(resolver, f, emit_fields, preserve_proto_field_names))
        .collect();

    let one_of_sers: Vec<TokenStream> = message
        .one_ofs
        .iter()
        .map(|o| serialize_one_of(resolver, o, preserve_proto_field_names))
        .collect();

    quote! {
        use serde::ser::SerializeStruct;
        #len_decl
        #(#field_len_checks)*
        #(#one_of_len_checks)*
        #struct_ser_decl
        #(#field_sers)*
        #(#one_of_sers)*
        struct_ser.end()
    }
}

// ===========================================================================
// Deserialization
// ===========================================================================

fn encode_scalar_field(scalar: ScalarType, field_modifier: FieldModifier) -> TokenStream {
    let deser = match override_deserializer(scalar) {
        Some(d) => d,
        None => {
            return match field_modifier {
                FieldModifier::Optional => quote!(map_.next_value()?),
                _ => quote!(Some(map_.next_value()?)),
            };
        }
    };

    match field_modifier {
        FieldModifier::Optional => {
            quote!(map_.next_value::<::std::option::Option<#deser>>()?.map(|x| x.0))
        }
        FieldModifier::Repeated => {
            quote! {
                Some(map_.next_value::<Vec<#deser>>()?
                    .into_iter().map(|x| x.0).collect())
            }
        }
        _ => {
            quote!(Some(map_.next_value::<#deser>()?.0))
        }
    }
}

fn deserialize_one_of_field_value(
    resolver: &Resolver<'_>,
    field: &Field,
    one_of: &OneOf,
) -> TokenStream {
    let one_of_type = resolver.rust_type_token(&one_of.path);
    let variant = type_name_ident(field);

    match &field.field_type {
        FieldType::Scalar(s) => match override_deserializer(*s) {
            Some(deser) => quote! {
                map_.next_value::<::std::option::Option<#deser>>()?.map(|x| #one_of_type::#variant(x.0))
            },
            None => quote! {
                map_.next_value::<::std::option::Option<_>>()?.map(#one_of_type::#variant)
            },
        },
        FieldType::Enum(path) => {
            let enum_type = resolver.rust_type_token(path);
            quote! {
                map_.next_value::<::std::option::Option<#enum_type>>()?.map(|x| #one_of_type::#variant(x as i32))
            }
        }
        FieldType::Message(type_name) => match well_known_type_deserializer(type_name) {
            Some(deser) => quote! {
                map_.next_value::<::std::option::Option<#deser>>()?.map(|x| #one_of_type::#variant(x.0))
            },
            None => quote! {
                map_.next_value::<::std::option::Option<_>>()?.map(#one_of_type::#variant)
            },
        },
        FieldType::Map(_, _) => unreachable!("one of cannot contain map fields"),
    }
}

fn deserialize_regular_field_value(
    resolver: &Resolver<'_>,
    field: &Field,
    btree_map: bool,
) -> TokenStream {
    match &field.field_type {
        FieldType::Scalar(scalar) => encode_scalar_field(*scalar, field.field_modifier),
        FieldType::Enum(path) => {
            let enum_type = resolver.rust_type_token(path);
            match field.field_modifier {
                FieldModifier::Optional => {
                    quote!(map_.next_value::<::std::option::Option<#enum_type>>()?.map(|x| x as i32))
                }
                FieldModifier::Repeated => {
                    quote!(Some(map_.next_value::<Vec<#enum_type>>()?.into_iter().map(|x| x as i32).collect()))
                }
                _ => {
                    quote!(Some(map_.next_value::<#enum_type>()? as i32))
                }
            }
        }
        FieldType::Map(key, value) => deserialize_map_field(resolver, key, value, btree_map),
        FieldType::Message(type_name) => {
            if let Some(wkt_deser) = well_known_type_deserializer(type_name) {
                match field.field_modifier {
                    FieldModifier::Repeated => {
                        quote!(Some(map_.next_value::<Vec<#wkt_deser>>()?.into_iter().map(|x| x.0.into()).collect()))
                    }
                    _ => {
                        quote!(map_.next_value::<::std::option::Option<#wkt_deser>>()?.map(|x| x.0.into()))
                    }
                }
            } else {
                match field.field_modifier {
                    FieldModifier::Repeated => quote!(Some(map_.next_value()?)),
                    _ => quote!(map_.next_value()?),
                }
            }
        }
    }
}

fn deserialize_map_field(
    resolver: &Resolver<'_>,
    key: &ScalarType,
    value: &FieldType,
    btree_map: bool,
) -> TokenStream {
    let map_type = if btree_map {
        quote!(std::collections::BTreeMap)
    } else {
        quote!(std::collections::HashMap)
    };

    let (key_type, key_needs_unwrap) = match key {
        ScalarType::Bytes | ScalarType::F32 | ScalarType::F64 => {
            panic!("protobuf disallows maps with floating point or bytes keys")
        }
        _ if key.is_numeric() => {
            let kt = key.rust_type_token();
            (quote!(crate::_serde::NumberDeserialize<#kt>), true)
        }
        _ => (quote!(_), false),
    };

    let (val_type, val_needs_transform, val_expr) = match value {
        FieldType::Scalar(scalar) if scalar.is_numeric() => {
            let vt = scalar.rust_type_token();
            (
                quote!(crate::_serde::NumberDeserialize<#vt>),
                true,
                quote!(v.0),
            )
        }
        FieldType::Scalar(ScalarType::Bytes) => (
            quote!(crate::_serde::BytesDeserialize<_>),
            true,
            quote!(v.0),
        ),
        FieldType::Enum(path) => {
            let enum_type = resolver.rust_type_token(path);
            (quote!(#enum_type), true, quote!(v as i32))
        }
        FieldType::Map(_, _) => panic!("protobuf disallows nested maps"),
        _ => (quote!(_), false, quote!(v)),
    };

    let needs_transform = key_needs_unwrap || val_needs_transform;
    let key_expr = if key_needs_unwrap {
        quote!(k.0)
    } else {
        quote!(k)
    };

    if needs_transform {
        quote! {
            Some(
                map_.next_value::<#map_type<#key_type, #val_type>>()?
                    .into_iter().map(|(k, v)| (#key_expr, #val_expr)).collect()
            )
        }
    } else {
        quote! {
            Some(
                map_.next_value::<#map_type<#key_type, #val_type>>()?
            )
        }
    }
}

fn deserialize_field(
    resolver: &Resolver<'_>,
    field: &Field,
    one_of: Option<&OneOf>,
    btree_map: bool,
) -> TokenStream {
    let owner_name = match one_of {
        Some(o) => o.rust_field_name(),
        None => field.rust_field_name(),
    };

    let json_name = field.json_name();
    let variant_ident = type_name_ident(field);
    let var = var_ident(&owner_name);

    let value_expr = match one_of {
        Some(o) => deserialize_one_of_field_value(resolver, field, o),
        None => deserialize_regular_field_value(resolver, field, btree_map),
    };

    quote! {
        GeneratedField::#variant_ident => {
            if #var.is_some() {
                return Err(serde::de::Error::duplicate_field(#json_name));
            }
            #var = #value_expr;
        }
    }
}

fn fields_enum(
    fields: &[(String, String, Option<String>)],
    ignore_unknown_fields: bool,
) -> TokenStream {
    let variants: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|(_, type_name, _)| {
            let escaped = escape_type(type_name.clone());
            proc_macro2::Ident::new(&escaped, proc_macro2::Span::call_site())
        })
        .collect();

    let skip = if ignore_unknown_fields {
        quote!(__SkipField__,)
    } else {
        quote!()
    };

    quote! {
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            #(#variants,)*
            #skip
        }
    }
}

fn deserialize_field_name(message: &Message, ignore_unknown_fields: bool) -> TokenStream {
    let fields: Vec<(String, String, Option<String>)> = message
        .all_fields()
        .map(|field| {
            let json_name = field.json_name();
            let proto_name = Some(field.name.clone()).filter(|p| p != &json_name);
            (json_name, field.rust_type_name(), proto_name)
        })
        .collect();

    // Build FIELDS array
    let field_name_strings: Vec<&str> = fields
        .iter()
        .flat_map(|(json_name, _, proto_name)| {
            proto_name
                .as_deref()
                .into_iter()
                .chain(std::iter::once(json_name.as_str()))
        })
        .collect();

    let fields_array = super::fields_array(&field_name_strings);
    let enum_def = fields_enum(&fields, ignore_unknown_fields);

    // Build match arms for visit_str
    let str_match_arms: Vec<TokenStream> = fields
        .iter()
        .map(|(json_name, type_name, proto_name)| {
            let variant = proc_macro2::Ident::new(
                &escape_type(type_name.clone()),
                proc_macro2::Span::call_site(),
            );
            match proto_name {
                Some(proto_name) => quote! {
                    #json_name | #proto_name => Ok(GeneratedField::#variant),
                },
                None => quote! {
                    #json_name => Ok(GeneratedField::#variant),
                },
            }
        })
        .collect();

    let unknown_field_handling = if !fields.is_empty() {
        let fallback = if ignore_unknown_fields {
            quote!(_ => Ok(GeneratedField::__SkipField__),)
        } else {
            quote!(_ => Err(serde::de::Error::unknown_field(value, FIELDS)),)
        };
        quote! {
            match value {
                #(#str_match_arms)*
                #fallback
            }
        }
    } else if ignore_unknown_fields {
        quote!(Ok(GeneratedField::__SkipField__))
    } else {
        quote!(Err(serde::de::Error::unknown_field(value, FIELDS)))
    };

    quote! {
        #fields_array
        #enum_def
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        #unknown_field_handling
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
    }
}

fn message_deserialize_body(
    resolver: &Resolver<'_>,
    message: &Message,
    rust_type: &TokenStream,
    ignore_unknown_fields: bool,
    btree_map_paths: &[String],
) -> TokenStream {
    let message_name = message.path.to_string();
    let struct_name_str = format!("struct {}", message.path);

    let field_name_deser = deserialize_field_name(message, ignore_unknown_fields);

    let field_var_inits: Vec<TokenStream> = message
        .fields
        .iter()
        .map(|f| {
            let v = var_ident(&f.rust_field_name());
            quote!(let mut #v = None;)
        })
        .collect();

    let one_of_var_inits: Vec<TokenStream> = message
        .one_ofs
        .iter()
        .map(|o| {
            let v = var_ident(&o.rust_field_name());
            quote!(let mut #v = None;)
        })
        .collect();

    let btree_map = btree_map_paths
        .iter()
        .any(|prefix| message.path.prefix_match(prefix.as_ref()).is_some());

    let has_fields = !message.fields.is_empty() || !message.one_ofs.is_empty();

    let map_loop = if has_fields {
        let field_arms: Vec<TokenStream> = message
            .fields
            .iter()
            .map(|f| deserialize_field(resolver, f, None, btree_map))
            .collect();

        let one_of_arms: Vec<TokenStream> = message
            .one_ofs
            .iter()
            .flat_map(|one_of| {
                one_of
                    .fields
                    .iter()
                    .map(move |f| deserialize_field(resolver, f, Some(one_of), btree_map))
            })
            .collect();

        let skip_arm = if ignore_unknown_fields {
            quote! {
                GeneratedField::__SkipField__ => {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        } else {
            quote!()
        };

        quote! {
            while let Some(k) = map_.next_key()? {
                match k {
                    #(#field_arms)*
                    #(#one_of_arms)*
                    #skip_arm
                }
            }
        }
    } else {
        quote! {
            while map_.next_key::<GeneratedField>()?.is_some() {
                let _ = map_.next_value::<serde::de::IgnoredAny>()?;
            }
        }
    };

    let field_constructions: Vec<TokenStream> = message
        .fields
        .iter()
        .map(|f| {
            let ident = field_ident(f);
            let v = var_ident(&f.rust_field_name());
            let json_name = f.json_name();

            match f.field_modifier {
                FieldModifier::Required => quote! {
                    #ident: #v.ok_or_else(|| serde::de::Error::missing_field(#json_name))?,
                },
                FieldModifier::UseDefault | FieldModifier::Repeated => quote! {
                    #ident: #v.unwrap_or_default(),
                },
                _ => quote! {
                    #ident: #v,
                },
            }
        })
        .collect();

    let one_of_constructions: Vec<TokenStream> = message
        .one_ofs
        .iter()
        .map(|o| {
            let ident = ident_from_escaped(&o.rust_field_name());
            let v = var_ident(&o.rust_field_name());
            quote!(#ident: #v,)
        })
        .collect();

    quote! {
        #field_name_deser
        struct GeneratedVisitor;
        #[allow(clippy::useless_conversion)]
        #[allow(clippy::unit_arg)]
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = #rust_type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(#struct_name_str)
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<#rust_type, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                #(#field_var_inits)*
                #(#one_of_var_inits)*
                #map_loop
                Ok(#rust_type {
                    #(#field_constructions)*
                    #(#one_of_constructions)*
                })
            }
        }
        deserializer.deserialize_struct(#message_name, FIELDS, GeneratedVisitor)
    }
}

// ===========================================================================
// Top-level entry point
// ===========================================================================

pub fn generate_message(
    resolver: &Resolver<'_>,
    message: &Message,
    ignore_unknown_fields: bool,
    btree_map_paths: &[String],
    emit_fields: bool,
    preserve_proto_field_names: bool,
) -> TokenStream {
    let rust_type = resolver.rust_type_token(&message.path);

    let ser_body =
        message_serialize_body(resolver, message, emit_fields, preserve_proto_field_names);
    let ser = super::serialize_impl(&rust_type, ser_body);

    let de_body = message_deserialize_body(
        resolver,
        message,
        &rust_type,
        ignore_unknown_fields,
        btree_map_paths,
    );
    let de = super::deserialize_impl(&rust_type, de_body);

    quote! {
        #ser
        #de
    }
}
