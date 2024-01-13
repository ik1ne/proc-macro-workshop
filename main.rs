// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use quote::quote;

use crate::seq::Seq;

mod seq {
    include!("seq/src/seq.rs");
}

fn main() {
    let input: proc_macro2::TokenStream = quote! {
        N in 0..16 {
            #[derive(Copy, Clone, PartialEq, Debug)]
            enum Interrupt {
                #(
                    Irq~N,
                )
            }
        }
    };

    let result = syn::parse2::<Seq>(input).unwrap().expand().unwrap();

    println!("{}", result);
}
