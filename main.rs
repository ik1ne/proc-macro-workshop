use quote::quote;

fn main() {
    let input = quote! {
        pub struct MyFourBytes {
            a: B1,
            b: B3,
            c: B4,
            d: B24,
        }
    };

    println!("{}", bitfield_lib::bitfield_inner(input).unwrap());
}

use bitfield::*;
//
// #[bitfield]
// pub struct MyFourBytes {
//     a: B1,
//     b: B3,
//     c: B4,
//     d: B24,
// }
