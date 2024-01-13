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

    Ok(quote! {
        .field(stringify!(#name), &self.#name)
    })
}
