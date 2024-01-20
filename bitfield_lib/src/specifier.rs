use proc_macro2::Ident;
use quote::quote;

pub trait Specifier {
    const BITS: usize;
}

pub fn generate_specifier_impls() -> syn::Result<proc_macro2::TokenStream> {
    let impls = (1..=64).map(generate_specifier_impl);

    Ok(quote! {
        #(#impls)*
    })
}

fn generate_specifier_impl(n: usize) -> proc_macro2::TokenStream {
    let name = Ident::new(&format!("B{}", n), proc_macro2::Span::call_site());
    let bits = n;

    quote! {
        pub enum #name {}

        impl Specifier for #name {
            const BITS: usize = #bits;
        }
    }
}
