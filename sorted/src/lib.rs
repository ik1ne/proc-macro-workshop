use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    sorted_lib::sorted::derive(args.into(), input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    sorted_lib::check::derive(args.into(), input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}