use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{ItemEnum, Variant};

pub use derive_bitfield::*;
pub use specifier::*;

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

    if num_fields.next_power_of_two() != num_fields {
        return Err(syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "BitfieldSpecifier expected a number of variants which is a power of 2",
        ));
    }

    let b_ty = format_ident!("B{}", bytes);

    let variant_in_range_checks = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| generate_variant_in_range_check(&ident, variant, i, num_fields));

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

        impl #impl_generics #ident #ty_generics #where_clause {
            #(#variant_in_range_checks)*
        }
    })
}

fn generate_variant_in_range_check(
    enum_ident: &Ident,
    variant: &Variant,
    i: usize,
    num_fields: usize,
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let fn_name = format_ident!("_check_{}", i);

    quote_spanned! {variant.span()=>
        fn #fn_name() {
            const IS_IN_RANGE: usize = ((#enum_ident::#ident as usize) < #num_fields) as usize;
            let _: bitfield::checks::InRange<[(); IS_IN_RANGE]> = ();
        }
    }
}
