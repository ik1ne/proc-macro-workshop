use quote::quote;

pub use specifier::generate_specifier_impls;
pub use specifier::Specifier;

mod specifier;

pub fn derive_bitfield(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {#input})
}
