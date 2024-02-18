use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(SerializeDisplayAdapter)]
pub fn serialize_display_adapter_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{}", e)))
            }
        }
    };
    gen.into()
}