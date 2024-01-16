use quote::{format_ident, quote};

use crate::specifier::generate::get_modulo_type;
use crate::specifier::{get_default_specifier_get_impl, get_default_specifier_set_impl};

pub fn generate_token_stream_for_bitfield_type(i: usize) -> proc_macro2::TokenStream {
    let ident = format_ident!("B{}", i);

    let min_size = get_min_size(i);
    let modulo_type = get_modulo_type(i);

    let bit_field_type = format_ident!("u{}", min_size * 8);

    let specifier_get_impl = get_default_specifier_get_impl();
    let specifier_set_impl = get_default_specifier_set_impl();

    quote! {
        pub struct #ident;

        impl Specifier for #ident {
            const BITS: usize = #i;
            type BitFieldType = #bit_field_type;
            type ModuloType = #modulo_type;

            #specifier_get_impl

            #specifier_set_impl
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

pub fn generate_token_stream_for_bitfield_types() -> proc_macro2::TokenStream {
    let quotes = (1..=64).map(generate_token_stream_for_bitfield_type);

    quotes.collect()
}
