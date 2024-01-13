use std::ops::Range;

use proc_macro2::{Group, Ident, TokenTree};

use crate::seq::parse_repetition_group;
use crate::Seq;

impl Seq {
    pub(crate) fn expand_tag_repetition(
        result: &mut Vec<TokenTree>,
        tokens: &[TokenTree],
        ident_repetition: &Ident,
        range: Range<usize>,
    ) -> syn::Result<()> {
        let mut i = 0;

        while let Some(token) = tokens.get(i) {
            if let TokenTree::Group(group) = token {
                let group_tokens = group.stream().into_iter().collect::<Vec<_>>();
                let mut group_inner = vec![];
                Seq::expand_tag_repetition(
                    &mut group_inner,
                    &group_tokens,
                    ident_repetition,
                    range.clone(),
                )?;

                let mut new_group =
                    Group::new(group.delimiter(), group_inner.into_iter().collect());
                new_group.set_span(group.span());

                result.push(TokenTree::Group(new_group));

                i += 1;
                continue;
            }

            if let Some(group) = parse_repetition_group(tokens, i) {
                for n in range.clone() {
                    Seq::expand_simple_once(
                        result,
                        group.stream().into_iter().collect::<Vec<_>>().as_slice(),
                        ident_repetition,
                        n,
                    )?;
                }

                i += 3;
            } else {
                result.push(token.clone());
                i += 1;
            }
        }

        Ok(())
    }
}
