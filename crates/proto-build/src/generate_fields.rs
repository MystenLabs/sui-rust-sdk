use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

use proc_macro2::TokenStream;
use prost_types::DescriptorProto;
use prost_types::FieldDescriptorProto;
use prost_types::FileDescriptorSet;
use prost_types::field_descriptor_proto::Type;
use quote::quote;

pub(crate) fn generate_field_info(packages: &HashMap<String, FileDescriptorSet>, out_dir: &Path) {
    for (package, fds) in packages {
        if package.contains("google") {
            continue;
        }

        let mut buf = String::new();
        let mut stream = TokenStream::new();

        for file in &fds.file {
            for message in &file.message_type {
                stream.extend(generate_field_info_for_message(message));
            }
        }

        let code = quote! {
            mod _field_impls {
                use super::*;

                use crate::field::MessageFields;
                use crate::field::MessageField;

                #stream
            }
        };

        let ast: syn::File = syn::parse2(code).expect("not a valid tokenstream");
        let code = prettyplease::unparse(&ast);
        buf.push_str(&code);

        let file_name = format!("{package}.field_info.rs");
        std::fs::write(out_dir.join(file_name), &buf).unwrap();
    }
}

fn generate_field_info_for_message(message: &DescriptorProto) -> TokenStream {
    let map_types: HashSet<String> = message
        .nested_type
        .iter()
        .filter_map(|m| {
            if m.options.as_ref().is_some_and(|o| o.map_entry()) {
                Some(m.name().to_owned())
            } else {
                None
            }
        })
        .collect();

    let constants = generate_field_constants(message, &map_types);
    let message_fields_impl = generate_message_fields_impl(message);

    quote! {
        #constants
        #message_fields_impl
    }
}

fn generate_field_constants(message: &DescriptorProto, map_types: &HashSet<String>) -> TokenStream {
    let message_ident = quote::format_ident!("{}", message.name());
    let mut field_consts = TokenStream::new();

    for field in &message.field {
        field_consts.extend(generate_field_constant(message.name(), field, map_types));
    }

    quote! {
        impl #message_ident {
            #field_consts
        }
    }
}

fn generate_message_fields_impl(message: &DescriptorProto) -> TokenStream {
    let message_ident = quote::format_ident!("{}", message.name());

    let mut field_refs = TokenStream::new();

    for field in &message.field {
        field_refs.extend(generate_field_reference(field));
    }

    quote! {
        impl MessageFields for #message_ident {
            const FIELDS: &'static [&'static MessageField] = &[
                #field_refs
            ];
        }
    }
}

fn generate_field_constant(
    message_name: &str,
    field: &FieldDescriptorProto,
    map_types: &HashSet<String>,
) -> TokenStream {
    let ident = quote::format_ident!("{}_FIELD", field.name().to_ascii_uppercase());
    let name = field.name();
    let json_name = field.json_name();
    let number = field.number();

    let message_fields =
        if matches!(field.r#type(), Type::Message) && !field.type_name().contains("google") {
            let field_message_name = field.type_name().split('.').next_back().unwrap();

            if field_message_name == message_name || map_types.contains(field_message_name) {
                quote! { None }
            } else {
                let field_message = quote::format_ident!("{}", field_message_name);
                quote! { Some(#field_message::FIELDS) }
            }
        } else {
            quote! { None }
        };

    quote! {
        pub const #ident: &'static MessageField = &MessageField {
            name: #name,
            json_name: #json_name,
            number: #number,
            message_fields: #message_fields,
        };
    }
}

fn generate_field_reference(field: &FieldDescriptorProto) -> TokenStream {
    let ident = quote::format_ident!("{}_FIELD", field.name().to_ascii_uppercase());

    quote! {
        Self::#ident,
    }
}
