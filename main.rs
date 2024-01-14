use std::str::FromStr;

fn main() {
    let input = proc_macro2::TokenStream::from_str(
        r#"pub struct Field<T: Trait> {
        values: Vec<T::Value>,
    }"#,
    )
    .unwrap();

    println!("{}", debug_lib::derive(input).unwrap());
}
