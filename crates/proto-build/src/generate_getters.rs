use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use heck::ToPascalCase;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use prost_types::DescriptorProto;
use prost_types::FieldDescriptorProto;
use prost_types::FileDescriptorSet;
use prost_types::field_descriptor_proto::Type;
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
                #![allow(clippy::useless_conversion)]
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
    let mut const_default_fields = TokenStream::new();

    let mut oneofs = BTreeSet::new();

    for field in &message.field {
        if let Some(function) = generate_getter_function(message, field, map_types) {
            functions.extend(function);
        }

        if let Some(const_default_field) =
            generate_const_default_field(field, map_types, &mut oneofs)
        {
            const_default_fields.extend(const_default_field);
        }
    }

    for oneof_index in oneofs {
        let oneof_field = quote::format_ident!("{}", message.oneof_decl[oneof_index].name());
        const_default_fields.extend(quote! {
            #oneof_field: None,
        });
    }

    quote! {
        impl #message_ident {
            pub const fn const_default() -> Self {
                Self {
                    #const_default_fields
                }
            }

            #[doc(hidden)]
            pub fn default_instance() -> &'static Self {
                static DEFAULT: #message_ident = #message_ident::const_default();
                &DEFAULT
            }

            #functions
        }
    }
}

fn generate_getter_function(
    message: &DescriptorProto,
    field: &FieldDescriptorProto,
    map_types: &HashSet<String>,
) -> Option<TokenStream> {
    let name = quote::format_ident!(
        "{}{}",
        if field.name() == "type" { "r#" } else { "" },
        field.name()
    );
    let name_opt = quote::format_ident!("{}_opt", field.name());
    let name_mut = quote::format_ident!("{}_mut", field.name());
    let name_opt_mut = quote::format_ident!("{}_opt_mut", field.name());
    let with_name = quote::format_ident!("with_{}", field.name());

    // Skip google messages
    if field.type_name().contains("google") {
        return None;
    }

    let field_message_name = resolve_type(field);
    let field_message_ty = TokenStream::from_str(&field_message_name).unwrap(); // quote::format_ident!("{}", field_message_name);

    // Skip maps for now
    if map_types.contains(&field_message_name) {
        return None;
    }

    // oneofs
    let function = if matches!(field.r#type(), Type::Enum) {
        // skip Enums
        return None;
    } else if field.oneof_index.is_some() && !field.proto3_optional() {
        let default_instance = TokenStream::from_str(&type_default(field)).unwrap();
        let ref_return_type = TokenStream::from_str(&getter_ref_return_type(field)).unwrap();
        let oneof = &message.oneof_decl[field.oneof_index() as usize];
        let oneof_field = quote::format_ident!("{}", oneof.name());
        let oneof_name = quote::format_ident!("{}", oneof.name().to_pascal_case());
        let message_mod = quote::format_ident!("{}", message.name().to_snake_case());
        let variant = quote::format_ident!("{}", field.name().to_pascal_case());
        let field_as = if is_ref_return(field) {
            quote! {field as _}
        } else {
            quote! {*field}
        };

        quote! {
            pub fn #name(&self) -> #ref_return_type {
                if let Some(#message_mod::#oneof_name::#variant(field)) = &self.#oneof_field {
                    #field_as
                    // field as _
                } else {
                    // #field_message_ty::default_instance() as _
                    #default_instance
                }
            }

            pub fn #name_opt(&self) -> Option<#ref_return_type> {
                if let Some(#message_mod::#oneof_name::#variant(field)) = &self.#oneof_field {
                    Some(#field_as)
                } else {
                    None
                }
            }

            pub fn #name_opt_mut(&mut self) -> Option<&mut #field_message_ty> {
                if let Some(#message_mod::#oneof_name::#variant(field)) = &mut self.#oneof_field {
                    Some(field as _)
                } else {
                    None
                }
            }

            pub fn #name_mut(&mut self) -> &mut #field_message_ty {
                if self.#name_opt_mut().is_none() {
                    self.#oneof_field = Some(#message_mod::#oneof_name::#variant(#field_message_ty::default()));
                }
                self.#name_opt_mut().unwrap()
            }

            pub fn #with_name(mut self, field: #field_message_ty) -> Self {
                self.#oneof_field = Some(#message_mod::#oneof_name::#variant(field.into()));
                self
            }
        }
    } else if matches!(
        field.label(),
        prost_types::field_descriptor_proto::Label::Repeated
    ) {
        quote! {
            pub fn #name(&self) -> &[#field_message_ty] {
                &self.#name
            }

            pub fn #with_name(mut self, field: Vec<#field_message_ty>) -> Self {
                self.#name = field;
                self
            }
        }
    } else if !matches!(field.r#type(), Type::Message) {
        //primitives
        quote! {
            // pub fn #name_opt(&self) -> Option<&#field_message_ty> {
            //     self.#name
            //         .as_ref()
            //         .map(|field| field as _)
            // }

            // pub fn #name_opt_mut(&mut self) -> Option<&mut #field_message_ty> {
            //     self.#name
            //         .as_mut()
            //         .map(|field| field as _)
            // }

            // pub fn #name_mut(&mut self) -> &mut #field_message_ty {
            //     self.#name
            //         .get_or_insert_default()
            // }

            pub fn #with_name(mut self, field: #field_message_ty) -> Self {
                self.#name = Some(field.into());
                self
            }
        }
    } else {
        quote! {
            pub fn #name(&self) -> &#field_message_ty {
                self.#name
                    .as_ref()
                    .map(|field| field as _)
                    .unwrap_or_else(|| #field_message_ty::default_instance() as _)
            }

            pub fn #name_opt(&self) -> Option<&#field_message_ty> {
                self.#name
                    .as_ref()
                    .map(|field| field as _)
            }

            pub fn #name_opt_mut(&mut self) -> Option<&mut #field_message_ty> {
                self.#name
                    .as_mut()
                    .map(|field| field as _)
            }

            pub fn #name_mut(&mut self) -> &mut #field_message_ty {
                self.#name
                    .get_or_insert_default()
            }

            pub fn #with_name(mut self, field: #field_message_ty) -> Self {
                self.#name = Some(field.into());
                self
            }
        }
    };

    Some(function)
}

fn generate_const_default_field(
    field: &FieldDescriptorProto,
    map_types: &HashSet<String>,
    oneofs: &mut BTreeSet<usize>,
) -> Option<TokenStream> {
    let name = quote::format_ident!(
        "{}{}",
        if field.name() == "type" { "r#" } else { "" },
        field.name()
    );
    let is_map = if matches!(field.r#type(), Type::Message) && !field.type_name().contains("google")
    {
        let field_message_name = field.type_name().split('.').next_back().unwrap();
        map_types.contains(field_message_name)
    } else {
        false
    };

    // Skip oneofs that are not synthetic optional
    if field.oneof_index.is_some() && !field.proto3_optional() {
        oneofs.insert(field.oneof_index() as usize);
        return None;
    }

    let const_default_field = if is_map {
        // Map fields
        quote! {
            #name: std::collections::BTreeMap::new(),
        }
    } else if matches!(
        field.label(),
        prost_types::field_descriptor_proto::Label::Repeated
    ) {
        // Repeated fields
        quote! {
            #name: Vec::new(),
        }
    } else {
        quote! {
            #name: None,
        }
    };

    Some(const_default_field)
}

fn resolve_type(field: &FieldDescriptorProto) -> String {
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
        Type::Group | Type::Message => field.type_name().split('.').next_back().unwrap().into(),
    }
}

fn type_default(field: &FieldDescriptorProto) -> String {
    match field.r#type() {
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
            let ty = field.type_name().split('.').next_back().unwrap();
            format!("{}::default_instance() as _", ty)
        }
    }
}

fn getter_ref_return_type(field: &FieldDescriptorProto) -> String {
    match field.r#type() {
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
            let ty = field.type_name().split('.').next_back().unwrap();
            format!("&{}", ty)
        }
    }
}

fn is_ref_return(field: &FieldDescriptorProto) -> bool {
    match field.r#type() {
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
