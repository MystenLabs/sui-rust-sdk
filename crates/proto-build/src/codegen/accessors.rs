use std::path::Path;
use std::str::FromStr;

use heck::ToPascalCase;
use proc_macro2::TokenStream;
use prost_types::field_descriptor_proto::Type;
use quote::quote;

use crate::context::Context;
use crate::message_graph::Field;
use crate::message_graph::Message;
use crate::message_graph::OneofField;

pub(crate) fn generate_accessors(context: &Context, out_dir: &Path) {
    for package in context.graph().packages.iter() {
        let mut stream = TokenStream::new();

        for message in context
            .graph()
            .messages
            .values()
            .filter(|m| &m.package == package && !context.is_extern(&m.type_name))
        {
            stream.extend(generate_accessors_for_message(context, message));
        }

        // If we didn't generate anything then just skip
        if !stream.is_empty() {
            let code = quote! {
                mod _accessor_impls {
                    #![allow(clippy::useless_conversion)]

                    #stream
                }
            };

            let ast: syn::File = syn::parse2(code).expect("not a valid tokenstream");
            let code = prettyplease::unparse(&ast);

            let file_name = format!("{}.accessors.rs", package.trim_start_matches('.'));
            std::fs::write(out_dir.join(file_name), &code).unwrap();
        }
    }
}

fn generate_accessors_for_message(context: &Context, message: &Message) -> TokenStream {
    let package = format!("{}.__accessors", message.package);
    let message_rust_path =
        TokenStream::from_str(&context.resolve_ident(&package, &message.type_name)).unwrap();

    let mut functions = TokenStream::new();

    functions.extend(generate_const_default_functions(
        context,
        message,
        &message_rust_path,
    ));

    functions.extend(generate_accessors_functions(context, message));

    quote! {
        impl #message_rust_path {

            #functions
        }
    }
}

fn generate_accessors_functions(context: &Context, message: &Message) -> TokenStream {
    let mut accessors = TokenStream::new();

    for field in &message.fields {
        accessors.extend(generate_accessors_functions_for_field(
            context, message, field, None,
        ));
    }

    for oneof_field in &message.oneof_fields {
        for field in &oneof_field.fields {
            accessors.extend(generate_accessors_functions_for_field(
                context,
                message,
                field,
                Some(oneof_field),
            ));
        }
    }

    accessors
}

fn generate_accessors_functions_for_field(
    context: &Context,
    message: &Message,
    field: &Field,
    oneof: Option<&OneofField>,
) -> TokenStream {
    let package = format!("{}.__accessors", message.package);
    let name = quote::format_ident!("{}", field.rust_struct_field_name());
    let name_opt = quote::format_ident!("{}_opt", field.inner.name());
    let set_name = quote::format_ident!("set_{}", field.inner.name());
    let name_mut = quote::format_ident!("{}_mut", field.inner.name());
    let name_opt_mut = quote::format_ident!("{}_opt_mut", field.inner.name());
    let with_name = quote::format_ident!("with_{}", field.inner.name());

    // doc comments

    let name_comments = vec![format!(
        "Returns the value of `{name}`, or the default value if `{name}` is unset."
    )];
    let name_opt_comments = vec![format!(
        "If `{name}` is set, returns [`Some`] with the value; otherwise returns [`None`]."
    )];
    let mut set_name_comments = vec![format!("Sets `{name}` with the provided value.")];
    let mut name_mut_comments = vec![
        format!("Returns a mutable reference to `{name}`."),
        "If the field is unset, it is first initialized with the default value.".to_owned(),
    ];
    let name_opt_mut_comments = vec![format!(
        "If `{name}` is set, returns [`Some`] with a mutable reference to the value; otherwise returns [`None`]."
    )];

    let field_type_path =
        TokenStream::from_str(&field.resolve_rust_type_path(context, &package)).unwrap();

    let default_instance = TokenStream::from_str(&type_default(field, context, &package)).unwrap();
    let ref_return_type =
        TokenStream::from_str(&ref_return_type(field, context, &package)).unwrap();
    let field_as = if is_ref_return(field) {
        quote! {field as _}
    } else {
        quote! {*field}
    };

    if let Some((key, value)) = &field.map {
        // Map Types
        let key_type =
            TokenStream::from_str(&resolve_rust_type_path(key, context, &package)).unwrap();
        let value_type =
            TokenStream::from_str(&resolve_rust_type_path(value, context, &package)).unwrap();

        quote! {
            #( #[doc = #name_comments] )*
            pub fn #name(&self) -> &::std::collections::BTreeMap<#key_type, #value_type> {
                &self.#name
            }

            #( #[doc = #name_mut_comments] )*
            pub fn #name_mut(&mut self) -> &mut ::std::collections::BTreeMap<#key_type, #value_type> {
                &mut self.#name
            }

            #( #[doc = #set_name_comments] )*
            pub fn #set_name(&mut self, field: ::std::collections::BTreeMap<#key_type, #value_type>) {
                self.#name = field;
            }

            #( #[doc = #set_name_comments] )*
            pub fn #with_name(mut self, field: ::std::collections::BTreeMap<#key_type, #value_type>) -> Self {
                self.#set_name(field);
                self
            }
        }
    } else if field.is_repeated() {
        if field.is_enum() {
            return TokenStream::new();
        }

        quote! {
            #( #[doc = #name_comments] )*
            pub fn #name(&self) -> &[#field_type_path] {
                &self.#name
            }

            #( #[doc = #name_mut_comments] )*
            pub fn #name_mut(&mut self) -> &mut Vec<#field_type_path> {
                &mut self.#name
            }

            #( #[doc = #set_name_comments] )*
            pub fn #set_name(&mut self, field: Vec<#field_type_path>) {
                self.#name = field;
            }

            #( #[doc = #set_name_comments] )*
            pub fn #with_name(mut self, field: Vec<#field_type_path>) -> Self {
                self.#set_name(field);
                self
            }
        }
    } else if let Some(oneof) = oneof {
        if field.inner.type_name() == ".google.protobuf.Empty" {
            return TokenStream::new();
        }

        let oneof_field = quote::format_ident!("{}", oneof.rust_struct_field_name());
        let oneof_type_path = TokenStream::from_str(&context.resolve_ident(
            &package,
            &format!(
                "{}.{}",
                message.type_name,
                oneof.descriptor.name().to_pascal_case()
            ),
        ))
        .unwrap();
        let variant = quote::format_ident!("{}", field.inner.name().to_pascal_case());

        name_mut_comments.push(
            "If any other oneof field in the same oneof is set, it will be cleared.".to_owned(),
        );
        set_name_comments.push(
            "If any other oneof field in the same oneof is set, it will be cleared.".to_owned(),
        );

        quote! {
            #( #[doc = #name_comments] )*
            pub fn #name(&self) -> #ref_return_type {
                if let Some(#oneof_type_path::#variant(field)) = &self.#oneof_field {
                    #field_as
                } else {
                    #default_instance
                }
            }

            #( #[doc = #name_opt_comments] )*
            pub fn #name_opt(&self) -> Option<#ref_return_type> {
                if let Some(#oneof_type_path::#variant(field)) = &self.#oneof_field {
                    Some(#field_as)
                } else {
                    None
                }
            }

            #( #[doc = #name_opt_mut_comments] )*
            pub fn #name_opt_mut(&mut self) -> Option<&mut #field_type_path> {
                if let Some(#oneof_type_path::#variant(field)) = &mut self.#oneof_field {
                    Some(field as _)
                } else {
                    None
                }
            }

            #( #[doc = #name_mut_comments] )*
            pub fn #name_mut(&mut self) -> &mut #field_type_path {
                if self.#name_opt_mut().is_none() {
                    self.#oneof_field = Some(#oneof_type_path::#variant(#field_type_path::default()));
                }
                self.#name_opt_mut().unwrap()
            }

            #( #[doc = #set_name_comments] )*
            pub fn #set_name<T: Into<#field_type_path>>(&mut self, field: T) {
                self.#oneof_field = Some(#oneof_type_path::#variant(field.into().into()));
            }

            #( #[doc = #set_name_comments] )*
            pub fn #with_name<T: Into<#field_type_path>>(mut self, field: T) -> Self {
                self.#set_name(field.into());
                self
            }
        }
    } else if field.is_optional() {
        let mut accessors = TokenStream::new();

        // only include "bare getter" for message types
        if field.is_message() && !field.is_well_known_type() {
            accessors.extend(quote! {
                #( #[doc = #name_comments] )*
                pub fn #name(&self) -> #ref_return_type {
                    self.#name
                        .as_ref()
                        .map(|field| field as _)
                        .unwrap_or_else(|| #default_instance)
                }
            });
        }

        // Only include mut getters for non bytes/enum types
        if !matches!(field.inner.r#type(), Type::Bytes | Type::Enum) {
            accessors.extend(quote! {
                #( #[doc = #name_opt_mut_comments] )*
                pub fn #name_opt_mut(&mut self) -> Option<&mut #field_type_path> {
                    self.#name
                        .as_mut()
                        .map(|field| field as _)
                }

                #( #[doc = #name_mut_comments] )*
                pub fn #name_mut(&mut self) -> &mut #field_type_path {
                    self.#name
                        .get_or_insert_default()
                }
            });
        }

        // only include _opt and set for non enums (as this already exists for enums from prost)
        if !matches!(field.inner.r#type(), Type::Enum) {
            accessors.extend(quote! {
                #( #[doc = #name_opt_comments] )*
                pub fn #name_opt(&self) -> Option<#ref_return_type> {
                    self.#name
                        .as_ref()
                        .map(|field| #field_as)
                }

                #( #[doc = #set_name_comments] )*
                pub fn #set_name<T: Into<#field_type_path>>(&mut self, field: T) {
                    self.#name = Some(field.into().into());
                }
            });
        }

        quote! {
            #accessors

            #( #[doc = #set_name_comments] )*
            pub fn #with_name<T: Into<#field_type_path>>(mut self, field: T) -> Self {
                self.#set_name(field.into());
                self
            }
        }
    } else {
        // maybe required or implicit optional

        let mut accessors = TokenStream::new();

        if field.inner.r#type() != Type::Bytes {
            accessors.extend(quote! {
            #( #[doc = #name_mut_comments] )*
                pub fn #name_mut(&mut self) -> &mut #field_type_path {
                    &mut self.#name
                }
            });
        }

        quote! {
            #accessors

            #( #[doc = #set_name_comments] )*
            pub fn #set_name<T: Into<#field_type_path>>(&mut self, field: T) {
                self.#name = field.into().into();
            }

            #( #[doc = #set_name_comments] )*
            pub fn #with_name<T: Into<#field_type_path>>(mut self, field: T) -> Self {
                self.#set_name(field.into());
                self
            }
        }
    }
}

fn generate_const_default_functions(
    _context: &Context,
    message: &Message,
    message_rust_path: &TokenStream,
) -> TokenStream {
    let mut const_default_fields = TokenStream::new();

    for field in &message.fields {
        let field_name = quote::format_ident!("{}", field.rust_struct_field_name());

        let field_default = if field.is_map() {
            quote! {
                #field_name: std::collections::BTreeMap::new(),
            }
        } else if field.is_repeated() {
            quote! {
                #field_name: Vec::new(),
            }
        } else if field.is_optional() {
            quote! {
                #field_name: None,
            }
        } else {
            // maybe required or implicit optional
            match field.inner.r#type() {
                Type::Double
                | Type::Float
                | Type::Int64
                | Type::Uint64
                | Type::Int32
                | Type::Fixed64
                | Type::Fixed32
                | Type::Uint32
                | Type::Enum
                | Type::Sfixed32
                | Type::Sfixed64
                | Type::Sint32
                | Type::Sint64 => {
                    quote! {
                        #field_name: 0,
                    }
                }

                Type::Bool => {
                    quote! {
                        #field_name: false,
                    }
                }
                Type::String => {
                    quote! {
                        #field_name: String::new(),
                    }
                }
                Type::Bytes => {
                    quote! {
                        #field_name: ::prost::bytes::Bytes::new(),
                    }
                }
                Type::Group | Type::Message => {
                    panic!("messages are optional");
                }
            }
        };

        const_default_fields.extend(field_default);
    }

    for oneof in &message.oneof_fields {
        let oneof_field = quote::format_ident!("{}", oneof.rust_struct_field_name());
        const_default_fields.extend(quote! {
            #oneof_field: None,
        });
    }

    quote! {
        pub const fn const_default() -> Self {
            Self {
                #const_default_fields
            }
        }

        #[doc(hidden)]
        pub fn default_instance() -> &'static Self {
            static DEFAULT: #message_rust_path = #message_rust_path::const_default();
            &DEFAULT
        }
    }
}

fn type_default(field: &Field, context: &Context, package: &str) -> String {
    match field.inner.r#type() {
        Type::Float => String::from("0.0f32"),
        Type::Double => String::from("0.0f64"),
        Type::Uint32 | Type::Fixed32 => String::from("0u32"),
        Type::Uint64 | Type::Fixed64 => String::from("0u64"),
        Type::Int32 | Type::Sfixed32 | Type::Sint32 | Type::Enum => String::from("0i32"),
        Type::Int64 | Type::Sfixed64 | Type::Sint64 => String::from("0i64"),
        Type::Bool => String::from("false"),
        Type::String => String::from("\"\""),
        Type::Bytes => String::from("&[]"),
        Type::Group | Type::Message => {
            let ty = context.resolve_ident(package, field.inner.type_name());
            format!("{}::default_instance() as _", ty)
        }
    }
}

fn ref_return_type(field: &Field, context: &Context, package: &str) -> String {
    match field.inner.r#type() {
        Type::Float => String::from("f32"),
        Type::Double => String::from("f64"),
        Type::Uint32 | Type::Fixed32 => String::from("u32"),
        Type::Uint64 | Type::Fixed64 => String::from("u64"),
        Type::Int32 | Type::Sfixed32 | Type::Sint32 | Type::Enum => String::from("i32"),
        Type::Int64 | Type::Sfixed64 | Type::Sint64 => String::from("i64"),
        Type::Bool => String::from("bool"),
        Type::String => String::from("&str"),
        Type::Bytes => String::from("&[u8]"),
        Type::Group | Type::Message => {
            let ty = context.resolve_ident(package, field.inner.type_name());
            format!("&{}", ty)
        }
    }
}

fn is_ref_return(field: &Field) -> bool {
    match field.inner.r#type() {
        Type::Float => false,
        Type::Double => false,
        Type::Uint32 | Type::Fixed32 => false,
        Type::Uint64 | Type::Fixed64 => false,
        Type::Int32 | Type::Sfixed32 | Type::Sint32 | Type::Enum => false,
        Type::Int64 | Type::Sfixed64 | Type::Sint64 => false,
        Type::Bool => false,
        Type::String => true,
        Type::Bytes => true,
        Type::Group | Type::Message => true,
    }
}

pub fn resolve_rust_type_path(
    field: &prost_types::FieldDescriptorProto,
    context: &crate::context::Context,
    package: &str,
) -> String {
    match field.r#type() {
        Type::Float => String::from("f32"),
        Type::Double => String::from("f64"),
        Type::Uint32 | Type::Fixed32 => String::from("u32"),
        Type::Uint64 | Type::Fixed64 => String::from("u64"),
        Type::Int32 | Type::Sfixed32 | Type::Sint32 | Type::Enum => String::from("i32"),
        Type::Int64 | Type::Sfixed64 | Type::Sint64 => String::from("i64"),
        Type::Bool => String::from("bool"),
        Type::String => String::from("String"),
        Type::Bytes => String::from("::prost::bytes::Bytes"),
        Type::Group | Type::Message => context.resolve_ident(package, field.type_name()),
    }
}
