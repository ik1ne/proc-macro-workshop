use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Expr, Field, GenericArgument, GenericParam,
    Generics, PathArguments, Type,
};

use crate::helper::iterate_field;

mod helper;
#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_inner(mut input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;

    let debug_fields = debug_fields(&input)?;

    let span = input.span();
    add_trait_bound(&mut input.generics, &input.data, span)?;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

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
    let mut debug_fields = vec![];

    iterate_field(&input.data, input.span(), |field| {
        debug_fields.push(debug_row(field)?);

        Ok(())
    })?;

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

fn add_trait_bound(
    generics: &mut Generics,
    data: &Data,
    span: proc_macro2::Span,
) -> syn::Result<()> {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            let mut has_phantom_t = false;
            iterate_field(data, span, |field| {
                if is_type_phantom_t(&field.ty, &type_param.ident)? {
                    has_phantom_t = true;
                }

                Ok(())
            })?;

            if !has_phantom_t {
                type_param.bounds.push(parse_quote!(::std::fmt::Debug));
            }
        }
    }

    Ok(())
}

// TODO: refactor? Is this the best way to do this?
fn is_type_phantom_t(ty: &Type, t: &Ident) -> syn::Result<bool> {
    let Type::Path(path) = ty else {
        return Ok(false);
    };

    let Some(last_segment) = path.path.segments.last() else {
        return Ok(false);
    };

    if last_segment.ident != "PhantomData" {
        return Ok(false);
    }

    let PathArguments::AngleBracketed(args) = &last_segment.arguments else {
        return Ok(false);
    };

    let arg = args.args.first().ok_or(syn::Error::new_spanned(
        args,
        "PhantomData must have one argument",
    ))?;

    let arg = match arg {
        GenericArgument::Type(ty) => ty,
        _ => {
            return Err(syn::Error::new_spanned(
                arg,
                "PhantomData must have one type argument",
            ));
        }
    };

    Ok(arg.to_token_stream().to_string() == t.to_token_stream().to_string())
}
