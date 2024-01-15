use quote::quote;
use syn::ItemStruct;

pub fn bitfield_inner(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let ItemStruct {
        attrs,
        vis,
        struct_token: _struct_token,
        ident,
        generics,
        fields,
        semi_token,
    }: ItemStruct = syn::parse2(input)?;

    let size = fields.iter().map(|field| {
        let field_type = &field.ty;
        quote! { <#field_type as ::bitfield_lib::Specifier>::BITS }
    });

    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #(#attrs)*
        #[repr(C)]
        #vis struct #ident #ty_generics #where_clause {
            data: [u8; (#(#size)+*) / 8],
        } #semi_token
    })
}
