use syn::ItemEnum;

pub(crate) fn check_enum_sorted(item_enum: &ItemEnum) -> syn::Result<()> {
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
