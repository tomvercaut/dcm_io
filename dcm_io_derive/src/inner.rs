use syn::{GenericArgument, PathArguments, Type, TypePath};

pub(crate) fn get_inner_bracketed_type<'a>(
    ty: &'a Type,
    outer_ident_name: &str,
) -> Option<&'a Type> {
    let Type::Path(TypePath { path, .. }) = ty else {
        return None;
    };

    let segments = &path.segments;
    let last_segment = segments.last()?;
    let ident = &last_segment.ident;

    let ident_name = ident.to_string();
    if ident_name.as_str() != outer_ident_name {
        return None;
    }

    let PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
        return None;
    };

    generics.args.first().and_then(|arg| {
        if let GenericArgument::Type(inner_type) = arg {
            Some(inner_type)
        } else {
            None
        }
    })
}

pub(crate) fn get_inner_type_option(ty: &Type) -> Option<&Type> {
    get_inner_bracketed_type(ty, "Option")
}

pub(crate) fn get_inner_type_vec(ty: &Type) -> Option<&Type> {
    get_inner_bracketed_type(ty, "Vec")
}
