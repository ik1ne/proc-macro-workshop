use quote::quote;

pub mod bitfield_type;
pub mod derive_bitfield_specifier;

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
