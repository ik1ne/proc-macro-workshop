pub use derive_bitfield::*;
use quote::{format_ident, quote};
pub use specifier::*;
use syn::ItemEnum;

mod derive_bitfield;
mod specifier;

pub mod checks;

pub fn derive_bitfield_specifier(
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let ItemEnum {
        ident,
        generics,
        variants,
        ..
    }: ItemEnum = syn::parse2(input)?;

    let num_fields = variants.len();
    let bytes = num_fields.next_power_of_two().trailing_zeros() as usize;

    let b_ty = format_ident!("B{}", bytes);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::bitfield::Specifier for #ident #ty_generics #where_clause {
            const BITS: usize = #bytes;
        }

        impl #impl_generics ::bitfield::ValueGetSet for #ident #ty_generics #where_clause {
            type ValueType = #ident;

            fn get(arr: &[u8], offset: usize) -> Self::ValueType {
                let value = <#b_ty as ::bitfield::ValueGetSet>::get(arr, offset);
                unsafe { ::core::mem::transmute(value) }
            }

            fn set(arr: &mut [u8], offset: usize, value: Self::ValueType) {
                let value = value as <#b_ty as ::bitfield::ValueGetSet>::ValueType;
                <#b_ty as ::bitfield::ValueGetSet>::set(arr, offset, value);
            }
        }
    })
}
