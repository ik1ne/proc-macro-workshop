use quote::{format_ident, quote};

pub fn generate_token_stream_for_bitfield_types() -> proc_macro2::TokenStream {
    let quotes = (1..=64usize).map(generate_token_stream_for_bitfield_type);

    quotes.collect()
}

fn generate_token_stream_for_bitfield_type(i: usize) -> proc_macro2::TokenStream {
    let ident = format_ident!("B{}", i);

    let min_size = get_min_size(i);
    let modulo_type = get_modulo_type(i);

    let bit_field_type = format_ident!("u{}", min_size * 8);

    quote! {
        pub struct #ident;

        impl Specifier for #ident {
            const BITS: usize = #i;
            type BitFieldType = #bit_field_type;
            type ModuloType = #modulo_type;

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
}

fn get_min_size(i: usize) -> usize {
    match i {
        1..=8 => 1,
        9..=16 => 2,
        17..=32 => 4,
        33..=64 => 8,
        i => unreachable!("{} is not a valid bitfield size", i),
    }
}

fn get_modulo_type(i: usize) -> proc_macro2::TokenStream {
    match i % 8 {
        0 => quote! { ::bitfield_lib::checks::ZeroMod8 },
        1 => quote! { ::bitfield_lib::checks::OneMod8 },
        2 => quote! { ::bitfield_lib::checks::TwoMod8 },
        3 => quote! { ::bitfield_lib::checks::ThreeMod8 },
        4 => quote! { ::bitfield_lib::checks::FourMod8 },
        5 => quote! { ::bitfield_lib::checks::FiveMod8 },
        6 => quote! { ::bitfield_lib::checks::SixMod8 },
        _ => quote! { ::bitfield_lib::checks::SevenMod8 },
    }
}
