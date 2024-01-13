use proc_macro::TokenStream;

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    debug_lib::derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
