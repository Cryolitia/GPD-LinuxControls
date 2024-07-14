//! ```rust
//! use serde::Serialize;
//! use serialize_display_adapter_macro_derive::PrettyJsonSerializeDisplayAdapter;
//!
//! #[derive(Serialize, PrettyJsonSerializeDisplayAdapter)]
//! struct Demo<'a> {
//!     name: &'a str,
//!     age: u8,
//! }
//! fn main() {
//!     let name = "root";
//!     let demo = Demo {
//!         name: name.as_ref(),
//!         age: 42,
//!     };
//!     print!("{}", demo)
//! }
//!```
//!
//! Should print:
//! ```json
//! {
//!     "name": "root",
//!     "age": 42
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(JsonSerializeDisplayAdapter)]
pub fn json_serialize_display_adapter_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics std::fmt::Display for #name #type_generics #where_clause where #name #type_generics : serde::Serialize {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string(self).unwrap_or_else(|e| format!("{}", e)))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(PrettyJsonSerializeDisplayAdapter)]
pub fn pretty_json_serialize_display_adapter_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics std::fmt::Display for #name #type_generics #where_clause where #name #type_generics : serde::Serialize {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{}", e)))
            }
        }
    };
    gen.into()
}
