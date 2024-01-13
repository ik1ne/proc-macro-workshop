// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use std::str::FromStr;

fn main() {
    let input = proc_macro2::TokenStream::from_str(
        "pub struct Field {
            name: &'static str,
            bitmask: u16,
        }",
    )
    .unwrap();

    println!("{}", debug_lib::derive(input).unwrap());
}
