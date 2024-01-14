use quote::quote;
use std::str::FromStr;

fn main() {
    let input = proc_macro2::TokenStream::from_str(
        r#"
pub enum Conference {
    RustBeltRust,
    RustConf,
    RustFest,
    RustLatam,
    RustRush,
}"#,
    )
    .unwrap();

    println!("{}", sorted_lib::derive(quote! {}, input).unwrap());
}
