use proc_macro::TokenStream;

use syn::parse_macro_input;

use seq_lib::Seq;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let seq = parse_macro_input!(input as Seq);

    seq.expand()
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
