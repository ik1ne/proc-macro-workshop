use proc_macro2::Ident;
use quote::{format_ident, quote};

pub trait Specifier {
    const BITS: usize;
}

pub trait ValueGetSet: Specifier {
    type ValueType;

    fn get(data: &[u8], bit_offset: usize) -> Self::ValueType;
    fn set(data: &mut [u8], bit_offset: usize, value: Self::ValueType);
}

impl Specifier for bool {
    const BITS: usize = 1;
}

impl ValueGetSet for bool {
    type ValueType = bool;

    fn get(arr: &[u8], offset: usize) -> Self::ValueType {
        arr[offset / 8] & (1 << (offset % 8)) != 0
    }

    fn set(arr: &mut [u8], offset: usize, value: Self::ValueType) {
        if value {
            arr[offset / 8] |= 1 << (offset % 8);
        } else {
            arr[offset / 8] &= !(1 << (offset % 8));
        }
    }
}

pub fn generate_specifier_impls() -> syn::Result<proc_macro2::TokenStream> {
    let impls = (1..=64).map(generate_specifier_impl);

    Ok(quote! {
        #(#impls)*
    })
}

fn generate_specifier_impl(bits: usize) -> proc_macro2::TokenStream {
    let name = Ident::new(&format!("B{}", bits), proc_macro2::Span::call_site());

    let bytes = get_min_bytes(bits);
    let value_ident = format_ident!("u{}", bytes * 8);

    quote! {
        pub enum #name {}

        impl Specifier for #name {
            const BITS: usize = #bits;
        }

        impl ValueGetSet for #name {
            type ValueType = #value_ident;

            fn get(arr: &[u8], mut bit_offset: usize) -> Self::ValueType {
                let mut value = 0u64;
                let mut remaining_len = Self::BITS;

                loop {
                    let byte_offset = bit_offset / 8;
                    let bit_offset_in_byte = bit_offset % 8;
                    let bits_to_read = ::core::cmp::min(remaining_len, 8 - bit_offset_in_byte);

                    let byte = arr[byte_offset];
                    let byte = byte << bit_offset_in_byte;
                    let byte = byte >> (8 - bits_to_read);
                    let byte = byte as u64;

                    value <<= bits_to_read;
                    value |= byte;

                    remaining_len -= bits_to_read;
                    bit_offset += bits_to_read;

                    if remaining_len == 0 {
                        break;
                    }
                }

                value as Self::ValueType
            }

            fn set(arr: &mut [u8], mut offset: usize, value: Self::ValueType) {
                let mut value = value as u64;
                let mut remaining_len = Self::BITS;

                loop {
                    let byte_offset = offset / 8;
                    let bit_offset_in_byte = offset % 8;
                    let bits_to_write = ::core::cmp::min(remaining_len, 8 - bit_offset_in_byte);

                    let mut byte = value >> (remaining_len - bits_to_write);
                    byte <<= (8 - bits_to_write);
                    byte >>= bit_offset_in_byte;

                    arr[byte_offset] |= byte as u8;

                    remaining_len -= bits_to_write;
                    offset += bits_to_write;

                    if remaining_len == 0 {
                        break;
                    }
                }
            }
        }
    }
}

fn get_min_bytes(bits: usize) -> usize {
    match bits {
        1..=8 => 1,
        9..=16 => 2,
        17..=32 => 4,
        _ => 8,
    }
}
