use syn::{GenericArgument, PathArguments, Type};

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
