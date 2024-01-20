use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(_args: TokenStream, input: TokenStream) -> TokenStream {
    bitfield_lib::derive_bitfield(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn generate_specifier_impls(_input: TokenStream) -> TokenStream {
    bitfield_lib::generate_specifier_impls()
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
