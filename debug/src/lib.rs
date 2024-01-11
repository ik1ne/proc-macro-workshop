use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Expr, Field, Fields, GenericParam, Generics,
};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;

    let debug_fields = debug_fields(&input)?;

    let generics = add_trait_bound(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::std::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct(stringify!(#ident))
                    #(#debug_fields)*
                    .finish()
            }
        }
    })
}

fn debug_fields(input: &DeriveInput) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let ident: &Ident = &input.ident;

    let mut debug_fields = vec![];

    match &input.data {
        Data::Struct(data) => match &data.fields {
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

    Ok(debug_fields)
}

fn debug_row(field: &Field) -> syn::Result<proc_macro2::TokenStream> {
    let field_ident = &field.ident;

    if let Some(attr) = field.attrs.first() {
        let name_value = attr.meta.require_name_value()?;
        let Expr::Lit(lit) = &name_value.value else {
            return Err(syn::Error::new_spanned(
                name_value,
                "expected a string literal like `#[debug = \"...\"]`",
            ));
        };
        Ok(quote_spanned! {field.span()=>
            .field(stringify!(#field_ident), &format_args!(#lit, &self.#field_ident))
        })
    } else {
        Ok(quote_spanned! {field.span()=>
            .field(stringify!(#field_ident), &self.#field_ident)
        })
    }
}

fn add_trait_bound(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(::std::fmt::Debug));
        }
    }

    generics
}
