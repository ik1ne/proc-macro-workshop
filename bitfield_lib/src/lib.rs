use quote::quote;

pub fn derive_bitfield(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {#input})
}
