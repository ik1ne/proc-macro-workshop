use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Field, ItemStruct, MetaNameValue};

pub fn derive_bitfield(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let item: ItemStruct = syn::parse2(input.clone())?;
    let ItemStruct {
        attrs,
        vis,
        ident,
        generics,
        fields,
        ..
    } = &item;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let field_type_as_specifier_bits = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            quote! { <#ty as Specifier>::BITS }
        })
        .collect::<Vec<_>>();

    let mut field_getter_setter_impls = vec![];
    let mut accumulated_offset = vec![quote! { 0 }];
    let mut bits_checks = vec![];

    for field in fields {
        field_getter_setter_impls.push(generate_field_getter_setter_impls(
            field,
            &mut accumulated_offset,
            &item,
        )?);

        if let Some(bit_check) = generate_bit_check(field) {
            bits_checks.push(bit_check);
        }
    }

    Ok(quote! {
        #(#attrs)*
        #[derive(Default)]
        #[repr(C)]
        #vis struct #ident #impl_generics #type_generics #where_clause {
            data: [u8; (#(#field_type_as_specifier_bits)+*) / 8],
        }

        #(#field_getter_setter_impls)*

        impl #impl_generics #ident #type_generics #where_clause {
            pub fn new() -> Self {
                Default::default()
            }
        }

        fn _check() {
            let _: bitfield::checks::MultipleOfEight<[(); (#(#field_type_as_specifier_bits)+*) % 8]> = ();
        }

        #(#bits_checks)*
    })
}

fn generate_field_getter_setter_impls(
    field: &Field,
    accumulated_offset: &mut Vec<proc_macro2::TokenStream>,
    item_struct: &ItemStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let Some(field_ident) = &field.ident else {
        return Err(syn::Error::new_spanned(
            field,
            "Bitfield fields must have an identifier",
        ));
    };
    let field_ty = &field.ty;

    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();
    let ident = &item_struct.ident;

    let get_field_ident = format_ident!("get_{}", field_ident);
    let set_field_ident = format_ident!("set_{}", field_ident);

    let result = quote! {
        impl #impl_generics #ident #type_generics #where_clause {
            pub fn #get_field_ident(&self) -> <#field_ty as ValueGetSet>::ValueType {
                let offset = #(#accumulated_offset)+*;
                <#field_ty as ValueGetSet>::get(&self.data, offset)
            }

            pub fn #set_field_ident(&mut self, value: <#field_ty as ValueGetSet>::ValueType) {
                let offset = #(#accumulated_offset)+*;
                <#field_ty as ValueGetSet>::set(&mut self.data, offset, value);
            }
        }
    };

    accumulated_offset.push(quote! { <#field_ty as Specifier>::BITS });

    Ok(result)
}

fn generate_bit_check(field: &Field) -> Option<proc_macro2::TokenStream> {
    let MetaNameValue { value, .. } = &field
        .attrs
        .iter()
        .filter_map(|attr| attr.meta.require_name_value().ok())
        .find(|meta| meta.path.get_ident().is_some_and(|ident| ident == "bits"))?;

    let bits = value.to_token_stream().to_string().parse::<usize>().ok()?;

    let field_ty = &field.ty;

    Some(quote_spanned! {value.span()=>
        const _: [(); #bits] = [(); <#field_ty as Specifier>::BITS];
    })
}
