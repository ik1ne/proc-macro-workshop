use quote::quote;
use syn::Item;

pub fn derive(
    _args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let item: Item = syn::parse2(input)?;

    Ok(quote! {
        #item
    })
}
