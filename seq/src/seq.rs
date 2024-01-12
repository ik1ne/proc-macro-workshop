use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Block, Ident, Lit, Token};

pub(crate) struct Seq {
    i: Ident,
    _in: Token![in],
    range_begin: Lit,
    _dot_dot: Token![..],
    range_end: Lit,
    body: Block,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Seq {
            i: input.parse()?,
            _in: input.parse()?,
            range_begin: input.parse()?,
            _dot_dot: input.parse()?,
            range_end: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl Seq {
    pub(crate) fn expand(self) -> syn::Result<proc_macro2::TokenStream> {
        Ok(quote!())
    }
}
