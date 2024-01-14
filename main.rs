use std::str::FromStr;

fn main() {
    let input = proc_macro2::TokenStream::from_str(
        r#"#[debug(bound = "T::Value: Debug")]
        pub struct Wrapper<T: Trait> {
            field: Field<T>,
        }"#,
    )
    .unwrap();

    println!("{}", debug_lib::derive(input).unwrap());
}
