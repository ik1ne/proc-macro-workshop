use syn::{Data, Field, Fields};

pub(crate) fn iterate_field(
    data: &Data,
    span: proc_macro2::Span,
    mut f: impl FnMut(&Field) -> syn::Result<()>,
) -> syn::Result<()> {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in fields.named.iter() {
                    f(field)?;
                }
            }
            _ => {
                return Err(syn::Error::new(
                    span,
                    "derive(CustomDebug) only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new(
                span,
                "derive(CustomDebug) only supports structs",
            ))
        }
    }

    Ok(())
}
