use proc_macro2::{Group, Ident, Literal, Punct, TokenTree};

use crate::seq::simple_repetition;
use crate::Seq;

impl Seq {
    pub(crate) fn expand_simple_once(
        result: &mut Vec<TokenTree>,
        tokens: &[TokenTree],
        ident_repetition: &Ident,
        repetition_index: usize,
    ) -> syn::Result<()> {
        let mut i = 0;

        while i < tokens.len() {
            let fed = Seq::expand_ith_token(result, tokens, i, ident_repetition, repetition_index)?;
            if fed == 0 {
                panic!("fed 0 tokens");
            }

            i += fed;
        }

        Ok(())
    }

    fn expand_ith_token(
        result: &mut Vec<TokenTree>,
        tokens: &[TokenTree],
        i: usize,
        ident_repetition: &Ident,
        repetition_index: usize,
    ) -> syn::Result<usize> {
        match &tokens[i] {
            TokenTree::Group(group) => {
                let mut group_inner = vec![];
                let group_token_stream = group.stream().into_iter().collect::<Vec<_>>();
                Seq::expand_simple_once(
                    &mut group_inner,
                    &group_token_stream,
                    ident_repetition,
                    repetition_index,
                )?;
                let mut new_group =
                    Group::new(group.delimiter(), group_inner.into_iter().collect());

                new_group.set_span(group.span());

                result.push(TokenTree::Group(new_group));
            }
            TokenTree::Ident(ident) => {
                if let Some(new_ident) = simple_repetition::try_combine_tilde_ident(
                    tokens,
                    i,
                    ident_repetition,
                    repetition_index,
                ) {
                    result.push(TokenTree::Ident(new_ident));
                    return Ok(3);
                }
                if *ident != *ident_repetition {
                    result.push(TokenTree::Ident(ident.clone()));
                } else {
                    let mut literal = Literal::usize_unsuffixed(repetition_index);
                    literal.set_span(ident.span());
                    result.push(TokenTree::Literal(literal));
                }
            }
            TokenTree::Punct(punct) => result.push(TokenTree::Punct(punct.clone())),

            TokenTree::Literal(literal) => result.push(TokenTree::Literal(literal.clone())),
        }

        Ok(1)
    }
}

pub fn try_combine_tilde_ident(
    tokens: &[TokenTree],
    i: usize,
    ident_repetition: &Ident,
    repetition_index: usize,
) -> Option<Ident> {
    let first_ident = get_ith_ident(tokens, i)?;
    let ith_punct = get_ith_punct(tokens, i + 1)?;
    if ith_punct.as_char() != '~' {
        return None;
    }

    let second_ident = get_ith_ident(tokens, i + 2)?;

    let ident_repetition_string = ident_repetition.to_string();

    if !second_ident
        .to_string()
        .starts_with(&ident_repetition_string)
    {
        return None;
    }

    let new_second_ident_string = second_ident.to_string().replacen(
        &ident_repetition_string,
        &repetition_index.to_string(),
        1,
    );
    let new_ident_string = format!("{}{}", first_ident, new_second_ident_string);
    let new_ident = Ident::new(&new_ident_string, second_ident.span());

    Some(new_ident)
}

fn get_ith_ident(tokens: &[TokenTree], i: usize) -> Option<&Ident> {
    match &tokens.get(i)? {
        TokenTree::Ident(ident) => Some(ident),
        _ => None,
    }
}

fn get_ith_punct(tokens: &[TokenTree], i: usize) -> Option<&Punct> {
    match &tokens.get(i)? {
        TokenTree::Punct(punct) => Some(punct),
        _ => None,
    }
}
