// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

// use derive_builder::Builder;
//
// #[derive(Builder)]
// pub struct Command {
//     executable: String,
//     #[builder(eac = "arg")]
//     args: Vec<String>,
//     env: Vec<String>,
//     current_dir: Option<String>,
// }

use quote::ToTokens;
use syn::{parse_quote, Expr, ExprAssign, Field, Lit};

fn main() {
    let incorrect_field: Field = parse_quote! {
        #[builder(eac = "arg")]
        args: Vec<String>
    };

    get_each_attr(&incorrect_field).unwrap();
}

fn get_each_attr(field: &Field) -> syn::Result<Option<String>> {
    let Some(attribute) = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("builder"))
    else {
        return Ok(None);
    };

    let expr: ExprAssign = attribute.parse_args()?;

    eprintln!("{:?}", expr.left.as_ref().to_token_stream());

    let Expr::Path(path_left) = expr.left.as_ref() else {
        panic!("builder attribute must be literal");
    };

    panic!("{:?}", path_left.path.to_token_stream());

    let Expr::Lit(lit_right) = expr.right.as_ref() else {
        panic!("builder attribute must be literal");
    };

    let Lit::Str(lit_str) = &lit_right.lit else {
        panic!("builder attribute must be string literal");
    };

    Ok(Some(lit_str.value()))
}
