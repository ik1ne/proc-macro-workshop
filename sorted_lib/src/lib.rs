use quote::quote;
use syn::{Item, ItemEnum};

pub fn derive(
    _args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let item: Item = syn::parse2(input)?;

    let result = match &item {
        Item::Enum(item_enum) => check_enum_sorted(item_enum),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        )),
    };

    match result {
        Ok(_) => Ok(quote! { #item }),
        Err(err) => {
            let err = err.to_compile_error();
            Ok(quote! { #item #err })
        }
    }
}

fn check_enum_sorted(item_enum: &ItemEnum) -> syn::Result<()> {
    let mut sorted_order = item_enum.variants.iter().collect::<Vec<_>>();
    sorted_order.sort_by_key(|variant| &variant.ident);

    item_enum
        .variants
        .iter()
        .zip(sorted_order.iter())
        .try_for_each(|(variant, sorted_variant)| {
            if variant.ident != sorted_variant.ident {
                Err(syn::Error::new(
                    sorted_variant.ident.span(),
                    format!(
                        "{} should sort before {}",
                        sorted_variant.ident, variant.ident,
                    ),
                ))
            } else {
                Ok(())
            }
        })?;

    Ok(())
}
