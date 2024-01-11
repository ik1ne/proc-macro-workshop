use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;

    let mut debug_fields: Vec<proc_macro2::TokenStream> = vec![];
    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                for field in fields.named.iter() {
                    debug_fields.push(debug_row(field)?);
                }
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    ident,
                    "derive(CustomDebug) only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                ident,
                "derive(CustomDebug) only supports structs",
            ))
        }
    }

    Ok(quote! {
        impl ::std::fmt::Debug for #ident {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct(stringify!(#ident))
                    #(#debug_fields)*
                    .finish()
            }
        }
    })
}

fn debug_row(field: &Field) -> syn::Result<proc_macro2::TokenStream> {
    let field_ident = &field.ident;

    Ok(quote! {
        .field(stringify!(#field_ident), &self.#field_ident)
    })
}
