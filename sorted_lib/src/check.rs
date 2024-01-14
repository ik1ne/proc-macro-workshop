use quote::quote;
use syn::ItemFn;
use syn::visit_mut::VisitMut;

use crate::match_sorted::MatchCheckReplace;

pub fn derive(
    _args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut item_fn: ItemFn = syn::parse2(input)?;

    let mut visitor = MatchCheckReplace { first_error: None };
    visitor.visit_item_fn_mut(&mut item_fn);

    match visitor.first_error {
        None => Ok(quote! { #item_fn }),
        Some(err) => {
            let err = err.to_compile_error();
            Ok(quote! { #item_fn #err })
        }
    }
}
