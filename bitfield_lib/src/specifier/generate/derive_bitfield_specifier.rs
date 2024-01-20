use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{ItemEnum, Variant};

use crate::specifier::generate::get_modulo_type;

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

    if num_fields.next_power_of_two() != num_fields {
        return Err(syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "BitfieldSpecifier expected a number of variants which is a power of 2",
        ));
    }

    let bits = num_fields.next_power_of_two().trailing_zeros() as usize;
    let modulo_type = get_modulo_type(bits);

    let b_ty = format_ident!("B{}", bits);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let variant_max_value_checks = variants
        .iter()
        .map(|variant| generate_variant_max_value_check(&ident, variant, bits));

    Ok(quote! {
         impl #impl_generics ::bitfield::Specifier for #ident #ty_generics #where_clause {
             const BITS: usize = #bits;
             type BitFieldType = #ident;
             type ModuloType = #modulo_type;

             fn get(arr: &[u8], offset: usize) -> Self::BitFieldType {
                 let value = <#b_ty as ::bitfield::Specifier>::get(arr, offset);
                 unsafe { ::core::mem::transmute(value) }
             }

             fn set(arr: &mut [u8], offset: usize, value: Self::BitFieldType) {
                 let value = value as <#b_ty as ::bitfield::Specifier>::BitFieldType;
                 <#b_ty as ::bitfield::Specifier>::set(arr, offset, value);
             }
        }

        #(#variant_max_value_checks)*
    })
}

fn generate_variant_max_value_check(
    enum_ident: &Ident,
    variant: &Variant,
    bits: usize,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    let max_val = 1usize << bits;

    quote! {
        const _IN_RANGE: ::bitfield::checks::InRange<[(); ((#enum_ident::#variant_ident as usize) .lt(&#max_val)) as usize]> = [];
    }
}
