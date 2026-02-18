//! This module contains the actual code generation logic

use proc_macro2::TokenStream;
use quote::quote;

mod enumeration;
mod message;

pub use enumeration::generate_enum;
pub use message::generate_message;

pub fn serialize_impl(rust_type: &TokenStream, body: TokenStream) -> TokenStream {
    quote! {
        impl serde::Serialize for #rust_type {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                #body
            }
        }
    }
}

pub fn deserialize_impl(rust_type: &TokenStream, body: TokenStream) -> TokenStream {
    quote! {
        impl<'de> serde::Deserialize<'de> for #rust_type {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #body
            }
        }
    }
}

pub fn fields_array(field_names: &[&str]) -> TokenStream {
    quote! {
        const FIELDS: &[&str] = &[
            #(#field_names,)*
        ];
    }
}
