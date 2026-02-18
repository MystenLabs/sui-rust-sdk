//! This module contains the code to generate Serialize and Deserialize
//! implementations for enumeration type
//!
//! An enumeration should be decode-able from the full string variant name
//! or its integer tag number, and should encode to the string representation

use proc_macro2::TokenStream;
use quote::quote;

use crate::descriptor::{EnumDescriptor, TypePath};
use crate::resolver::Resolver;
use std::collections::HashSet;

pub fn generate_enum(
    resolver: &Resolver<'_>,
    path: &TypePath,
    descriptor: &EnumDescriptor,
    use_integers_for_enums: bool,
) -> TokenStream {
    let rust_type = resolver.rust_type_token(path);

    let mut seen_numbers = HashSet::new();
    let variants: Vec<_> = descriptor
        .values
        .iter()
        .filter(|variant| seen_numbers.insert(variant.number()))
        .map(|variant| {
            let variant_name = variant.name.clone().unwrap();
            let variant_number = variant.number();
            let rust_variant = resolver.rust_variant_ident(path, &variant_name);
            (variant_name, variant_number, rust_variant)
        })
        .collect();

    let serialize_impl = generate_serialize(&rust_type, &variants, use_integers_for_enums);
    let deserialize_impl = generate_deserialize(&rust_type, &variants);

    quote! {
        #serialize_impl
        #deserialize_impl
    }
}

fn generate_serialize(
    rust_type: &TokenStream,
    variants: &[(String, i32, proc_macro2::Ident)],
    use_integers_for_enums: bool,
) -> TokenStream {
    let body = if use_integers_for_enums {
        let arms = variants.iter().map(|(_, number, rust_variant)| {
            quote! { Self::#rust_variant => #number, }
        });
        quote! {
            let variant = match self {
                #(#arms)*
            };
            serializer.serialize_i32(variant)
        }
    } else {
        let arms = variants.iter().map(|(name, _, rust_variant)| {
            quote! { Self::#rust_variant => #name, }
        });
        quote! {
            let variant = match self {
                #(#arms)*
            };
            serializer.serialize_str(variant)
        }
    };

    super::serialize_impl(rust_type, body)
}

fn generate_deserialize(
    rust_type: &TokenStream,
    variants: &[(String, i32, proc_macro2::Ident)],
) -> TokenStream {
    let field_names: Vec<&str> = variants.iter().map(|(name, _, _)| name.as_str()).collect();
    let fields_array = super::fields_array(&field_names);

    let str_match_arms = variants.iter().map(|(name, _, rust_variant)| {
        quote! { #name => Ok(#rust_type::#rust_variant), }
    });

    let visitor = quote! {
        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = #rust_type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    #(#str_match_arms)*
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
    };

    let body = quote! {
        #fields_array
        #visitor
        deserializer.deserialize_any(GeneratedVisitor)
    };

    super::deserialize_impl(rust_type, body)
}
