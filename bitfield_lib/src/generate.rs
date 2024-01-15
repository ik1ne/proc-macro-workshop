use quote::{format_ident, quote};

pub fn generate_token_stream_for_bitfield_types() -> proc_macro2::TokenStream {
    let range = 1..=64usize;
    let ident = range.clone().map(|i| format_ident!("B{}", i));

    quote! {
        #(
            pub struct #ident;

            impl Specifier for #ident {
                const BITS: usize = #range;
            }
        )*
    }
}
