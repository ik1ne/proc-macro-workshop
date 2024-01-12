use proc_macro2::{Group, Literal, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Brace;
use syn::{braced, Ident, LitInt, Token};

pub(crate) struct Seq {
    ident_n: Ident,
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
            ident_n: input.parse()?,
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
    pub(crate) fn expand(self) -> syn::Result<proc_macro2::TokenStream> {
        let range = self.range_begin()?..self.range_end()?;
        let token_trees = self.body.into_iter().collect::<Vec<_>>();

        let mut result: Vec<TokenTree> = vec![];
        for i in range {
            Seq::expand_once(&mut result, token_trees.iter(), &self.ident_n, i)?;
        }

        let result: proc_macro2::TokenStream = result.into_iter().collect();
        Ok(quote! { #result })
    }

    fn range_begin(&self) -> syn::Result<usize> {
        self.range_begin
            .to_string()
            .parse()
            .map_err(|_| syn::Error::new_spanned(&self.range_begin, "expected integer literal"))
    }

    fn range_end(&self) -> syn::Result<usize> {
        self.range_end
            .to_string()
            .parse()
            .map_err(|_| syn::Error::new_spanned(&self.range_end, "expected integer literal"))
    }

    fn expand_once<'a>(
        result: &mut Vec<TokenTree>,
        tokens: impl Iterator<Item = &'a TokenTree>,
        ident_n: &Ident,
        i: usize,
    ) -> syn::Result<()> {
        for token in tokens {
            Seq::expand_single_token(result, token, ident_n, i)?;
        }

        Ok(())
    }

    fn expand_single_token(
        result: &mut Vec<TokenTree>,
        token: &TokenTree,
        ident_n: &Ident,
        i: usize,
    ) -> syn::Result<()> {
        match token {
            TokenTree::Group(group) => {
                let mut group_inner = vec![];
                let group_token_stream = group.stream().into_iter().collect::<Vec<_>>();
                Seq::expand_once(&mut group_inner, group_token_stream.iter(), ident_n, i)?;
                let mut new_group =
                    Group::new(group.delimiter(), group_inner.into_iter().collect());

                new_group.set_span(group.span());

                result.push(TokenTree::Group(new_group));
            }
            TokenTree::Ident(ident) => {
                if *ident != *ident_n {
                    result.push(TokenTree::Ident(ident.clone()));
                } else {
                    let mut literal = Literal::usize_unsuffixed(i);
                    literal.set_span(ident.span());
                    result.push(TokenTree::Literal(literal));
                }
            }
            TokenTree::Punct(punct) => result.push(TokenTree::Punct(punct.clone())),

            TokenTree::Literal(literal) => result.push(TokenTree::Literal(literal.clone())),
        }

        Ok(())
    }
}
