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
                #![allow(clippy::wrong_self_convention)]

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
    let field_path_builders = generate_field_path_builders_impl(message, &map_types);

    quote! {
        #constants
        #message_fields_impl
        #field_path_builders
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

fn generate_field_path_builders_impl(
    message: &DescriptorProto,
    map_types: &HashSet<String>,
) -> TokenStream {
    let message_ident = quote::format_ident!("{}", message.name());
    let builder_ident = quote::format_ident!("{}FieldPathBuilder", message.name());

    let mut field_chain_methods = TokenStream::new();

    for field in &message.field {
        field_chain_methods.extend(generate_field_chain_methods(
            message.name(),
            field,
            map_types,
        ));
    }

    quote! {
        impl #message_ident {
            pub fn path_builder() -> #builder_ident {
                #builder_ident::new()
            }
        }

        pub struct #builder_ident {
            path: Vec<&'static str>,
        }

        impl #builder_ident {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self {
                    path: Default::default(),
                }
            }

            #[doc(hidden)]
            pub fn new_with_base(base: Vec<&'static str>) -> Self {
                Self { path: base }
            }

            pub fn finish(self) -> String {
                self.path.join(".")
            }

            #field_chain_methods
        }
    }
}

fn generate_field_chain_methods(
    message_name: &str,
    field: &FieldDescriptorProto,
    map_types: &HashSet<String>,
) -> TokenStream {
    let message_ident = quote::format_ident!("{message_name}");
    let field_const = quote::format_ident!("{}_FIELD", field.name().to_ascii_uppercase());
    let name = if field.name() == "type" {
        quote::format_ident!("r#{}", field.name())
    } else {
        quote::format_ident!("{}", field.name())
    };

    if matches!(field.r#type(), Type::Message) && !field.type_name().contains("google") {
        let field_message_name = field.type_name().split('.').next_back().unwrap();

        if field_message_name == message_name || map_types.contains(field_message_name) {
            quote! {
                pub fn #name(mut self) -> String {
                    self.path.push(#message_ident::#field_const.name);
                    self.finish()
                }
            }
        } else {
            let return_type = quote::format_ident!("{}FieldPathBuilder", field_message_name);
            quote! {
                pub fn #name(mut self) -> #return_type {
                    self.path.push(#message_ident::#field_const.name);
                    #return_type::new_with_base(self.path)
                }
            }
        }
    } else {
        quote! {
            pub fn #name(mut self) -> String {
                self.path.push(#message_ident::#field_const.name);
                self.finish()
            }
        }
    }
}
