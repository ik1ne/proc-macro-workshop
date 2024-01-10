use proc_macro::TokenStream;

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

mod helper;

macro_rules! iterate_data_field {
    ($input:ident, $fields:ident, $field:ident, $field_ident:ident, $field_ty:ident, $body:block) => {
        match &$input.data {
            Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => {
                    for $field in &fields.named {
                        {
                            let $field_ident = &$field.ident;
                            let $field_ty = &$field.ty;

                            $body
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    for (i, $field) in fields.unnamed.iter().enumerate() {
                        {
                            let $field_ident = format_ident!("field{}", i);
                            let $field_ty = &$field.ty;

                            $body
                        }
                    }
                }
                _ => unimplemented!("Unit struct is not supported"),
            },
            _ => unimplemented!("Only struct is supported"),
        }
    };
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident_impl = ident_impl(&input)?;

    let builder_struct_impl = builder_struct_impl(&input)?;

    Ok(quote! {
        #ident_impl

        #builder_struct_impl
    })
}

/// ```rust,ignore
/// impl Ident {
///     pub fn builder() -> IdentBuilder {
///         IdentBuilder {
///             field1: None,
///             field2: None,
///        }
///     }
/// }
///```
fn ident_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    let ident_builder = format_ident!("{}Builder", ident);

    let mut ident_builder_fields_with_none = vec![];

    iterate_data_field!(input, fields, field, field_ident, _field_ty, {
        ident_builder_fields_with_none.push(quote_spanned! {field.span()=>
            #field_ident: None
        });
    });

    Ok(quote! {
        impl #ident {
            pub fn builder() -> #ident_builder {
                #ident_builder {
                    #(#ident_builder_fields_with_none),*
                }
            }
        }
    })
}

/// ```rust,ignore
/// pub struct IdentBuilder {
///     field1: Option<String>,
///     field2: Option<String>,
/// }
///
/// impl IdentBuilder {
///     pub fn field1(&mut self, field1: String) -> &mut Self {
///         self.field1 = Some(field1);
///         self
///     }
/// }
/// ```
fn builder_struct_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    let ident_builder = format_ident!("{}Builder", ident);

    let mut ident_builder_fields_with_ty = vec![];

    iterate_data_field!(input, fields, field, field_ident, field_ty, {
        if get_option_inner_ty(field_ty)?.is_some() {
            ident_builder_fields_with_ty.push(quote_spanned! {field.span()=>
                #field_ident: #field_ty
            });
        } else {
            ident_builder_fields_with_ty.push(quote_spanned! {field.span()=>
                #field_ident: Option<#field_ty>
            });
        }
    });

    let mut setter_methods = vec![];

    iterate_data_field!(input, fields, field, field_ident, field_ty, {
        if let Some(inner_ty) = get_option_inner_ty(field_ty)? {
            setter_methods.push(quote_spanned! {field.span() =>
                pub fn #field_ident(&mut self, #field_ident: #inner_ty) -> &mut Self {
                    self.#field_ident = Some(#field_ident);
                    self
                }
            });
        } else {
            setter_methods.push(quote_spanned! {field.span() =>
                pub fn #field_ident(&mut self, #field_ident: #field_ty) -> &mut Self {
                    self.#field_ident = Some(#field_ident);
                    self
                }
            });
        }
    });

    let mut build_method_fields = vec![];

    iterate_data_field!(input, fields, field, field_ident, field_ty, {
        if get_option_inner_ty(field_ty)?.is_some() {
            build_method_fields.push(quote_spanned! {field.span() =>
                #field_ident: self.#field_ident.take()
            });
        } else {
            build_method_fields.push(quote_spanned! {field.span() =>
                #field_ident: self.#field_ident.take().ok_or("field is not set")?
            });
        }
    });

    Ok(quote! {
        pub struct #ident_builder {
            #(#ident_builder_fields_with_ty),*
        }

        impl #ident_builder {
            #(#setter_methods)*
        }

        impl #ident_builder {
            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(#ident {
                    #(#build_method_fields),*
                })
            }
        }
    })
}

fn get_option_inner_ty(ty: &Type) -> syn::Result<Option<&Type>> {
    let Type::Path(path) = ty else {
        return Ok(None);
    };

    let path_segment = path.path.segments.first().ok_or(syn::Error::new_spanned(
        path,
        "Option must have one path segment",
    ))?;

    if path_segment.ident != "Option" {
        return Ok(None);
    }

    let PathArguments::AngleBracketed(args) = &path_segment.arguments else {
        return Err(syn::Error::new_spanned(
            path_segment,
            "Option must have angle bracketed arguments",
        ));
    };

    let arg = args.args.first().ok_or(syn::Error::new_spanned(
        args,
        "Option must have one argument",
    ))?;

    let arg = match arg {
        GenericArgument::Type(ty) => ty,
        _ => {
            return Err(syn::Error::new_spanned(
                arg,
                "Option must have one type argument",
            ))
        }
    };

    Ok(Some(arg))
}
