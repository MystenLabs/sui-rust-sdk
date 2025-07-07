use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

use proc_macro2::TokenStream;
use prost_types::field_descriptor_proto::Type;
use prost_types::DescriptorProto;
use prost_types::FieldDescriptorProto;
use prost_types::FileDescriptorSet;
use quote::quote;

pub(crate) fn generate_getters(packages: &HashMap<String, FileDescriptorSet>, out_dir: &Path) {
    for (package, fds) in packages {
        if package.contains("google") {
            continue;
        }

        let mut buf = String::new();
        let mut stream = TokenStream::new();

        for file in &fds.file {
            for message in &file.message_type {
                stream.extend(generate_getters_for_message(message));
            }
        }

        let code = quote! {
            mod _getter_impls {
                use super::*;

                #stream
            }
        };

        let ast: syn::File = syn::parse2(code).expect("not a valid tokenstream");
        let code = prettyplease::unparse(&ast);
        buf.push_str(&code);

        let file_name = format!("{package}.getters.rs");
        std::fs::write(out_dir.join(file_name), &buf).unwrap();
    }
}

fn generate_getters_for_message(message: &DescriptorProto) -> TokenStream {
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

    let functions = generate_getter_functions(message, &map_types);

    quote! {
        #functions
    }
}

fn generate_getter_functions(
    message: &DescriptorProto,
    map_types: &HashSet<String>,
) -> TokenStream {
    let message_ident = quote::format_ident!("{}", message.name());
    let mut functions = TokenStream::new();

    for field in &message.field {
        if let Some(function) = generate_getter_function(field, map_types) {
            functions.extend(function);
        }
    }

    quote! {
        impl #message_ident {
            #[doc(hidden)]
            pub fn default_ref() -> &'static Self {
                static DEFAULT: std::sync::LazyLock<#message_ident> = std::sync::LazyLock::new(#message_ident::default);
                &DEFAULT
            }

            #functions
        }
    }
}

fn generate_getter_function(
    field: &FieldDescriptorProto,
    map_types: &HashSet<String>,
) -> Option<TokenStream> {
    let name = quote::format_ident!(
        "{}{}",
        if field.name() == "type" { "r#" } else { "" },
        field.name()
    );
    let field_message_name =
        if matches!(field.r#type(), Type::Message) && !field.type_name().contains("google") {
            field.type_name().split('.').next_back().unwrap()
        } else {
            return None;
        };
    let field_message_ty = quote::format_ident!("{}", field_message_name);

    // Skip maps for now
    if map_types.contains(field_message_name)
        || (field.oneof_index.is_some() && !field.proto3_optional())
    {
        return None;
    }

    let function = if matches!(
        field.label(),
        prost_types::field_descriptor_proto::Label::Repeated
    ) {
        quote! {
            pub fn #name(&self) -> &[#field_message_ty] {
                &self.#name
            }
        }
    } else {
        quote! {
            pub fn #name(&self) -> &#field_message_ty {
                if let Some(r) = &self.#name {
                    r as _
                } else {
                    #field_message_ty::default_ref()
                }
                // self.#name.as_ref().as_deref().unwrap_or_else(#field_message_ty::default_ref)
            }
        }
    };

    Some(function)
}
