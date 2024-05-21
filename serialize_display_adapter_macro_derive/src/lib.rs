use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(SerializeDisplayAdapter)]
pub fn serialize_display_adapter_macro_derive(input: TokenStream) -> TokenStream {
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
