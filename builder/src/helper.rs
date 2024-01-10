use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::{
    Data, Expr, ExprLit, Field, Fields, GenericArgument, Ident, PathArguments, Result, Token, Type,
};

pub fn map_fields<'a>(
    data: &'a Data,
    f: impl Fn(&Field) -> TokenStream + 'a,
) -> Box<dyn Iterator<Item = TokenStream> + 'a> {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Box::new(fields.named.iter().map(f)),
            _ => todo!(),
        },
        _ => unimplemented!("only struct is supported"),
    }
}

pub fn extract_option_type(ty: &Type) -> Option<&Type> {
    let Type::Path(path) = ty else {
        return None;
    };

    let first_segment = path.path.segments.first()?;
    if first_segment.ident != "Option" {
        return None;
    }

    let PathArguments::AngleBracketed(first_angle_bracketed) = &first_segment.arguments else {
        return None;
    };

    let generic_arg = first_angle_bracketed.args.first()?;
    let GenericArgument::Type(ty) = generic_arg else {
        return None;
    };

    Some(ty)
}

struct BuilderAttr {
    each_ident: Ident,
    _eq_token: Token![=],
    each_expr: Expr,
}

impl Parse for BuilderAttr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        Ok(Self {
            each_ident: input.parse()?,
            _eq_token: input.parse()?,
            each_expr: input.parse()?,
        })
    }
}

pub fn get_each_from_builder_attribute(field: &Field) -> Result<Option<ExprLit>> {
    let Some(builder_attr) = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("builder"))
    else {
        return Ok(None);
    };

    let expr_tokens = builder_attr.parse_args::<Expr>()?.to_token_stream();
    let parsed_attr = syn::parse2::<BuilderAttr>(expr_tokens)?;

    if parsed_attr.each_ident != "each" {
        panic!("only `each` is supported");
    }

    let Expr::Lit(lit) = parsed_attr.each_expr else {
        panic!("only literal is supported");
    };

    Ok(Some(lit))
}
