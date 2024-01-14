use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_quote, Field, Fields, GenericArgument, GenericParam, Generics, ItemStruct, PathArguments,
    Type, TypePath, WhereClause,
};

pub fn derive(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let mut item_struct = syn::parse2::<ItemStruct>(input)?;
    let impl_debug_for_item_struct = impl_debug_for(&mut item_struct)?;

    Ok(quote! {
        #impl_debug_for_item_struct
    })
}

fn impl_debug_for(item_struct: &mut ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &item_struct.ident;

    add_where_trait_bounds_for_associated_types(
        &mut item_struct.generics.where_clause,
        &item_struct.generics.params,
        &item_struct.fields,
    );

    add_trait_bounds(&mut item_struct.generics, &item_struct.fields);

    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    // vector of `.field("bar", &self.bar)` as proc_macro2::TokenStream
    let field_adds = item_struct
        .fields
        .iter()
        .map(debug_field)
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics ::std::fmt::Debug for #struct_ident #type_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut fields = f.debug_struct(stringify!(#struct_ident));
                fields
                    #(#field_adds)*
                    .finish()
            }
        }
    })
}

fn add_trait_bounds(generics: &mut Generics, fields: &Fields) {
    let associated_types = fields
        .iter()
        .filter_map(|field| {
            if let Type::Path(path) = &field.ty {
                Some(path)
            } else {
                None
            }
        })
        .flat_map(|path| get_associated_type_paths(path, &generics.params))
        .collect::<Vec<_>>();

    for param in &mut generics.params {
        if let GenericParam::Type(ty) = param {
            let ident = &ty.ident;
            if is_ident_inside_phantom_data(ident, fields) {
                continue;
            }

            if associated_types.iter().any(|associated_type| {
                associated_type
                    .path
                    .segments
                    .first()
                    .is_some_and(|segment| &segment.ident == ident)
            }) {
                continue;
            }

            ty.bounds.push(parse_quote!(::std::fmt::Debug));
        }
    }
}
fn is_ident_inside_phantom_data(ident: &Ident, fields: &Fields) -> bool {
    fields.iter().any(|field| is_phantom_t(ident, field))
}

fn is_phantom_t(ident: &Ident, field: &Field) -> bool {
    if let Type::Path(path) = &field.ty {
        if let Some(phantom_path_segment) = path.path.segments.last() {
            if phantom_path_segment.ident == "PhantomData" {
                if let PathArguments::AngleBracketed(args) = &phantom_path_segment.arguments {
                    if let Some(GenericArgument::Type(Type::Path(path))) = args.args.first() {
                        return path.path.is_ident(ident);
                    }
                }
            }
        }
    }

    false
}

fn debug_field(field: &Field) -> syn::Result<proc_macro2::TokenStream> {
    let name = field.ident.as_ref().unwrap();

    let debug_expr_assign = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("debug"));

    if let Some(debug_expr_assign) = debug_expr_assign {
        let name_value = debug_expr_assign.meta.require_name_value()?;
        let value = &name_value.value;

        Ok(quote! {
            .field(stringify!(#name), &::std::format_args!(#value, &self.#name))
        })
    } else {
        Ok(quote! {
            .field(stringify!(#name), &self.#name)
        })
    }
}

fn add_where_trait_bounds_for_associated_types(
    where_clause: &mut Option<WhereClause>,
    generics: &Punctuated<GenericParam, Comma>,
    fields: &Fields,
) {
    for field in fields.iter() {
        add_trait_bound_for_associated_type(where_clause, generics, field);
    }
}

fn add_trait_bound_for_associated_type(
    where_clause_opt: &mut Option<WhereClause>,
    generics: &Punctuated<GenericParam, Comma>,
    field: &Field,
) {
    let where_clause = match where_clause_opt {
        None => where_clause_opt.insert(parse_quote! { where }),
        Some(where_clause) => where_clause,
    };

    if let Type::Path(path) = &field.ty {
        for associated_type_path in get_associated_type_paths(path, generics) {
            where_clause.predicates.push(parse_quote! {
                #associated_type_path: ::std::fmt::Debug
            });
        }
    }
}

fn get_associated_type_paths<'a>(
    path: &'a TypePath,
    generics: &Punctuated<GenericParam, Comma>,
) -> Vec<&'a TypePath> {
    let mut result = vec![];
    // 1. type itself is associated type
    if let Some(first_segment) = path.path.segments.first() {
        if generics.iter().any(
            |generic| matches!(generic, GenericParam::Type(ty) if ty.ident == first_segment.ident),
        ) && path.path.segments.len() > 1
        {
            result.push(path);
        }
    }

    // 2. type contains associated type
    if let Some(last_segment) = path.path.segments.last() {
        if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
            for arg in &args.args {
                if let GenericArgument::Type(Type::Path(path)) = arg {
                    result.append(&mut get_associated_type_paths(path, generics));
                }
            }
        }
    }

    result
}
