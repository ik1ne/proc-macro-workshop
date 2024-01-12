use proc_macro::TokenStream;
use quote::quote;

use syn::parse_macro_input;

use crate::seq::Seq;

mod seq;

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let _seq = parse_macro_input!(input as Seq);

    TokenStream::from(quote! {})
}
