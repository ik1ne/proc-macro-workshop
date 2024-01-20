use quote::quote;

fn main() {
    let input = quote! {
        pub fn region(&self) -> &str {
            use self::Conference::*;

            #[sorted]
            match self {
                RustFest => "Europe",
                RustLatam => "Latin America",
                _ => "elsewhere",
            }
        }
    };

    println!("{}", sorted_lib::check::derive(quote! {}, input).unwrap());
}
