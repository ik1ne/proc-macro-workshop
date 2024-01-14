use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    sorted_lib::derive(args.into(), input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
