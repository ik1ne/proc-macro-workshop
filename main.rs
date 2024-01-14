use quote::quote;

fn main() {
    let input = quote! {
        pub enum Error {
            Fmt(fmt::Error),
            Io(io::Error),
            Utf8(Utf8Error),
            Var(VarError),
            Dyn(Box<dyn StdError>),
        }
    };

    println!("{}", sorted_lib::derive(quote! {}, input).unwrap());
}
