use quote::quote;
use syn::Item;

pub fn derive(
    _args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let item: Item = syn::parse2(input)?;

    let Item::Enum(item_enum) = item else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        ));
    };

    Ok(quote! {
        #item_enum
    })
}
