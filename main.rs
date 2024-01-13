// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use std::str::FromStr;

use seq_lib::Seq;

fn main() {
    let input_simple = proc_macro2::TokenStream::from_str(
        "N in 1..4 {
            fn f~N () -> u64 {
                N * 2
            }
        }",
    )
    .unwrap();

    let _simple_result = syn::parse2::<Seq>(input_simple).unwrap().expand().unwrap();

    let input_repetition: proc_macro2::TokenStream = proc_macro2::TokenStream::from_str(
        "N in 0..2 {
            #[derive(Copy, Clone, PartialEq, Debug)]
            enum Interrupt {
                #(
                    Irq~N,
                )*
            }
        }",
    )
    .unwrap();

    let result = syn::parse2::<Seq>(input_repetition)
        .unwrap()
        .expand()
        .unwrap();

    println!("{}", result);
}
