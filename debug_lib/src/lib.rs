use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_quote, Field, Fields, GenericArgument, GenericParam, Generics, ItemStruct, PathArguments,
    Type,
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

    add_trait_bounds_if_not_inside_phantom_data(&mut item_struct.generics, &item_struct.fields);

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

fn add_trait_bounds_if_not_inside_phantom_data(generics: &mut Generics, fields: &Fields) {
    for param in &mut generics.params {
        if let GenericParam::Type(ty) = param {
            let ident = &ty.ident;
            if is_ident_inside_phantom_data(ident, fields) {
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

fn debug_field(field: &syn::Field) -> syn::Result<proc_macro2::TokenStream> {
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
