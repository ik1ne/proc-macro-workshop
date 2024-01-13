use proc_macro2::{Delimiter, Ident, TokenTree};
use quote::quote;
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
    eq: Option<Token![=]>,
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
            eq: input.parse()?,
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
        let range_begin = self.range_begin.base10_parse()?;
        let range_end = self.range_end.base10_parse()?;

        let range = if self.eq.is_some() {
            // This should have been range_begin..=range_end, but .. and ..= are two separate types and I am lazy.
            range_begin..(range_end + 1)
        } else {
            range_begin..range_end
        };

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
        if parse_repetition_group(body, i).is_some() {
            return true;
        }

        if let TokenTree::Group(group) = token {
            let group_tokens = group.stream().into_iter().collect::<Vec<_>>();
            if contains_repetition_tag(&group_tokens) {
                return true;
            }
        }
    }

    false
}

pub(crate) fn parse_repetition_group(body: &[TokenTree], i: usize) -> Option<&proc_macro2::Group> {
    if !matches!(body.get(i), Some(TokenTree::Punct(punct)) if punct.as_char() == '#')
        || !matches!(body.get(i + 2), Some(TokenTree::Punct(punct)) if punct.as_char() == '*')
    {
        return None;
    }

    let Some(TokenTree::Group(group)) = body.get(i + 1) else {
        return None;
    };

    if group.delimiter() != Delimiter::Parenthesis {
        return None;
    }

    Some(group)
}
