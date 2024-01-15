use quote::{format_ident, quote};

pub fn generate_token_stream_for_bitfield_types() -> proc_macro2::TokenStream {
    let quotes = (1..=64usize).map(generate_token_stream_for_bitfield_type);

    quotes.collect()
}

fn generate_token_stream_for_bitfield_type(i: usize) -> proc_macro2::TokenStream {
    let ident = format_ident!("B{}", i);

    let min_size = get_min_size(i);

    let bit_field_type = format_ident!("u{}", min_size * 8);

    quote! {
        pub struct #ident;

        impl Specifier for #ident {
            const BITS: usize = #i;
            type BitFieldType = #bit_field_type;

            fn get(arr: &[u8], mut bit_offset: usize) -> Self::BitFieldType {
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

                    value = value << bits_to_read;
                    value = value | byte;

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
                    byte = byte << (8 - bits_to_write);
                    byte = byte >> bit_offset_in_byte;

                    arr[byte_offset] = arr[byte_offset] | byte as u8;

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
