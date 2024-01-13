use quote::quote;
use syn::ItemStruct;

pub fn derive(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let item_struct = syn::parse2::<ItemStruct>(input)?;
    let impl_debug_for_item_struct = impl_debug_for(&item_struct)?;

    Ok(quote! {
        #impl_debug_for_item_struct
    })
}

fn impl_debug_for(item_struct: &ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &item_struct.ident;

    // vector of `.field("bar", &self.bar)` as proc_macro2::TokenStream
    let field_adds = item_struct
        .fields
        .iter()
        .map(debug_field)
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        impl ::std::fmt::Debug for #struct_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut fields = f.debug_struct(stringify!(#struct_ident));
                fields
                    #(#field_adds)*
                    .finish()
            }
        }
    })
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
