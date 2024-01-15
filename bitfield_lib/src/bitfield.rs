use quote::{format_ident, quote, TokenStreamExt};
use syn::{Fields, ItemStruct};

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
        quote! { <#field_type as ::bitfield::Specifier>::BITS }
    });

    let (getters, setters) = getters_setters_tokens(&fields);

    let mod_struct_ident = calculate_modulo_type_ident(&fields);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[repr(C)]
        #(#attrs)*
        #vis struct #ident #ty_generics #where_clause {
            data: [u8; (#(#size)+*) / 8],
        } #semi_token

        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn new() -> Self {
                Self {
                    data: ::core::default::Default::default(),
                }
            }

            #(#getters)*

            #(#setters)*
        }

        fn _check_modulo(_result: #mod_struct_ident) -> impl bitfield::checks::TotalSizeIsMultipleOfEightBits {
            _result
        }
    })
}

fn getters_setters_tokens(
    fields: &Fields,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut offset = quote! { 0usize };
    let mut getters = Vec::new();
    let mut setters = Vec::new();

    for field in fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let get_ident = format_ident!("get_{}", field_ident);
        let set_ident = format_ident!("set_{}", field_ident);

        getters.push(quote! {
            pub fn #get_ident(&self) -> <#field_type as ::bitfield::Specifier>::BitFieldType {
                <#field_type as ::bitfield::Specifier>::get(&self.data, #offset)
            }
        });

        setters.push(quote! {
            pub fn #set_ident(&mut self, value: <#field_type as ::bitfield::Specifier>::BitFieldType) {
                <#field_type as ::bitfield::Specifier>::set(&mut self.data, #offset, value);
            }
        });

        offset.append_all(quote! { + <#field_type as ::bitfield::Specifier>::BITS });
    }

    (getters, setters)
}

fn calculate_modulo_type_ident(fields: &Fields) -> proc_macro2::TokenStream {
    // <<field1 as Specifier>::ModuloType as AddRhs<<field2 as Specifier>::ModuloType>>::Output
    let mut fields_iter = fields.iter();
    let Some(first_field) = fields_iter.next() else {
        panic!("Bitfield must have at least one field"); // TODO change to syn::Error
    };

    let first_field_ty = &first_field.ty;

    let mut result = quote! { <#first_field_ty as ::bitfield::Specifier>::ModuloType };

    for field in fields_iter {
        let field_type = &field.ty;
        result = quote! { <<#field_type as ::bitfield::Specifier>::ModuloType as ::bitfield::checks::AddMod8<#result>>::Output };
    }

    result
}
