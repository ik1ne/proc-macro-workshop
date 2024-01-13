use std::ops::Range;

use proc_macro2::{Ident, TokenTree};

use crate::Seq;

impl Seq {
    pub(crate) fn expand_tag_repetition(
        result: &mut Vec<TokenTree>,
        tokens: &[TokenTree],
        ident_repetition: &Ident,
        range: Range<usize>,
    ) -> syn::Result<()> {
        todo!()
    }
}
