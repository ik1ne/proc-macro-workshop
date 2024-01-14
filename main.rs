use quote::quote;

fn main() {
    let input = quote! {
        pub enum Conference {
            RustBeltRust,
            RustConf,
            RustFest,
            RustLatam,
            RustRush,
        }
    };

    println!("{}", sorted_lib::derive(quote! {}, input).unwrap());
}
