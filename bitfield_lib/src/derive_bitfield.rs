use quote::quote;
use syn::ItemStruct;

pub fn derive_bitfield(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let ItemStruct {
        attrs,
        vis,
        ident,
        generics,
        fields,
        ..
    }: ItemStruct = syn::parse2(input.clone())?;

    let field_type_as_specifier_bits = fields.iter().map(|field| {
        let ty = &field.ty;
        quote! { <#ty as Specifier>::BITS }
    });

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #(#attrs)*
        #vis struct #ident #impl_generics #type_generics #where_clause {
            data: [u8; (#(#field_type_as_specifier_bits)+*) / 8],
        }
    })
}
