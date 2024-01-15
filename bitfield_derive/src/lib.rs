use proc_macro::TokenStream;
use syn::Error;

#[proc_macro_attribute]
pub fn bitfield(_args: TokenStream, input: TokenStream) -> TokenStream {
    bitfield_lib::bitfield_inner(input.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn generate_bitfield_types(_input: TokenStream) -> TokenStream {
    bitfield_lib::generate_token_stream_for_bitfield_types().into()
}
