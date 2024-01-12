use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{braced, Ident, Lit, Token};

pub(crate) struct Seq {
    i: Ident,
    _in: Token![in],
    range_begin: Lit,
    _dot_dot: Token![..],
    range_end: Lit,
    brace_token: Brace,
    body: proc_macro2::TokenStream,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Seq {
            i: input.parse()?,
            _in: input.parse()?,
            range_begin: input.parse()?,
            _dot_dot: input.parse()?,
            range_end: input.parse()?,
            brace_token: braced!(content in input),
            body: content.parse()?,
        })
    }
}

impl Seq {
    pub(crate) fn expand(self) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote! {})
    }
}
