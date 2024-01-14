use quote::quote;
use syn::Item;

use crate::enum_sorted::check_enum_sorted;

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
