use proc_macro2::{Delimiter, Ident, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{braced, LitInt, Token};

mod simple_repetition;
mod tag_repetition;

pub struct Seq {
    ident_repetition: Ident,
    _in: Token![in],
    range_begin: LitInt,
    _dot_dot: Token![..],
    range_end: LitInt,
    _brace_token: Brace,
    body: proc_macro2::TokenStream,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Seq {
            ident_repetition: input.parse()?,
            _in: input.parse()?,
            range_begin: input.parse()?,
            _dot_dot: input.parse()?,
            range_end: input.parse()?,
            _brace_token: braced!(content in input),
            body: content.parse()?,
        })
    }
}

impl Seq {
    pub fn expand(self) -> syn::Result<proc_macro2::TokenStream> {
        let token_trees = self.body.into_iter().collect::<Vec<_>>();
        let is_tag_repetition = contains_repetition_tag(&token_trees);
        let range =
            self.range_begin.base10_parse::<usize>()?..self.range_end.base10_parse::<usize>()?;

        let mut result: Vec<TokenTree> = vec![];

        if is_tag_repetition {
            Seq::expand_tag_repetition(&mut result, &token_trees, &self.ident_repetition, range)?;
        } else {
            for i in range {
                Seq::expand_simple_once(&mut result, &token_trees, &self.ident_repetition, i)?;
            }
        };

        let result: proc_macro2::TokenStream = result.into_iter().collect();
        Ok(quote! { #result })
    }
}

fn contains_repetition_tag(body: &[TokenTree]) -> bool {
    for (i, token) in body.iter().enumerate() {
        match token {
            TokenTree::Group(group) => {
                let group_tokens = group.stream().into_iter().collect::<Vec<_>>();
                if contains_repetition_tag(&group_tokens) {
                    return true;
                }
            }
            TokenTree::Punct(punct) => {
                // i is '#', i + 1 is parentheses group,
                if punct.as_char() == '#'
                    && matches!(body.get(i + 1), Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis)
                {
                    // i + 2 is '*'
                    if let Some(TokenTree::Punct(punct)) = body.get(i + 2) {
                        if punct.as_char() == '*' {
                            return true;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    false
}
