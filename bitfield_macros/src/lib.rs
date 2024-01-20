use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(_args: TokenStream, input: TokenStream) -> TokenStream {
    bitfield_lib::derive_bitfield(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
