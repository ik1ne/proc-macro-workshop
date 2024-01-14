use quote::quote;

fn main() {
    let input = quote! {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::Error::*;

            #[sorted]
            match self {
                Io(e) => write!(f, "{}", e),
                Fmt(e) => write!(f, "{}", e),
            }
        }
    };

    println!("{}", sorted_lib::check::derive(quote! {}, input).unwrap());
}
