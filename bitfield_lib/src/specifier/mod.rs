use crate::checks::OneMod8;
use quote::quote;

pub(crate) mod generate;

pub trait Specifier {
    const BITS: usize;
    type BitFieldType;
    type ModuloType;

    fn get(arr: &[u8], offset: usize) -> Self::BitFieldType;
    fn set(arr: &mut [u8], offset: usize, value: Self::BitFieldType);
}

fn get_default_specifier_get_impl() -> proc_macro2::TokenStream {
    quote! {
        fn get(arr: &[u8], mut bit_offset: usize) -> Self::BitFieldType {
            let mut value = 0u64;
            let mut remaining_len = Self::BITS;

            loop {
                let byte_offset = bit_offset / 8;
                let bit_offset_in_byte = bit_offset % 8;
                let bits_to_read = ::core::cmp::min(remaining_len, 8 - bit_offset_in_byte);

                let mut byte = arr[byte_offset];
                byte <<= bit_offset_in_byte;
                byte >>= (8 - bits_to_read);
                let byte = byte as u64;

                value <<= bits_to_read;
                value |= byte;

                remaining_len -= bits_to_read;
                bit_offset += bits_to_read;

                if remaining_len == 0 {
                    break;
                }
            }

            value as Self::BitFieldType
        }
    }
}

fn get_default_specifier_set_impl() -> proc_macro2::TokenStream {
    quote! {
        fn set(arr: &mut [u8], mut offset: usize, value: Self::BitFieldType) {
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

impl Specifier for bool {
    const BITS: usize = 1;
    type BitFieldType = bool;
    type ModuloType = OneMod8;

    fn get(arr: &[u8], offset: usize) -> Self::BitFieldType {
        arr[offset / 8] & (1 << (offset % 8)) != 0
    }

    fn set(arr: &mut [u8], offset: usize, value: Self::BitFieldType) {
        if value {
            arr[offset / 8] |= 1 << (offset % 8);
        } else {
            arr[offset / 8] &= !(1 << (offset % 8));
        }
    }
}
