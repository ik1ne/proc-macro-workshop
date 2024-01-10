use proc_macro::TokenStream;

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Expr, ExprAssign, Field, Fields, GenericArgument, Lit,
    PathArguments, Type,
};

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
                            let $field_ident = Some(format_ident!("field{}", i));
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

#[proc_macro_derive(Builder, attributes(builder))]
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
        if get_each_attr(field)?.is_some() {
            ident_builder_fields_with_none.push(quote_spanned! {field.span()=>
                #field_ident: vec![]
            });
            continue;
        }
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
///
/// impl IdentBuilder {
///     pub fn build(&mut self) -> Result<Ident, Box<dyn std::error::Error>> {
///         Ok(Ident {
///             field1: self.field1.take().ok_or("field is not set")?,
///             field2: self.field2.take().ok_or("field is not set")?,
///         })
///     }
/// }
/// ```
fn builder_struct_impl(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    let ident_builder = format_ident!("{}Builder", ident);

    let mut ident_builder_fields_with_ty = vec![];

    iterate_data_field!(input, fields, field, field_ident, field_ty, {
        // 1. type is Option<T>
        // 2. type has #[builder(each = "arg")] attribute
        if get_option_inner_ty(field_ty)?.is_some() || get_each_attr(field)?.is_some() {
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
        let each_attr_opt = get_each_attr(field)?;
        let is_field_name_same_as_attr_name = if let (Some(field_ident), Some(each_attr_opt)) =
            (field_ident.as_ref(), &each_attr_opt)
        {
            field_ident == each_attr_opt
        } else {
            false
        };

        // 1. type is Option<T>
        if let Some(inner_ty) = get_option_inner_ty(field_ty)? {
            setter_methods.push(quote_spanned! {field.span() =>
                pub fn #field_ident(&mut self, #field_ident: #inner_ty) -> &mut Self {
                    self.#field_ident = Some(#field_ident);
                    self
                }
            });
            continue;
        } else if let Some(each_attr) = &each_attr_opt {
            // 2. type has #[builder(each = "arg")] attribute, type is Vec<T>
            let each_attr_ident = format_ident!("{}", each_attr);
            // TODO replace #field_ty with inner type of Vec
            setter_methods.push(quote_spanned! {field.span() =>
                pub fn #each_attr_ident(&mut self, val: String) -> &mut Self {
                    self.#field_ident.push(val);
                    self
                }
            });

            if !is_field_name_same_as_attr_name {
                setter_methods.push(quote_spanned! {field.span() =>
                    pub fn #field_ident(&mut self, val: #field_ty) -> &mut Self {
                        self.#field_ident = val;
                        self
                    }
                });
            }
        } else {
            // 3. type is T
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
        if get_each_attr(field)?.is_some() {
            build_method_fields.push(quote_spanned! {field.span() =>
                #field_ident: self.#field_ident.drain(..).collect()
            });
        } else if get_option_inner_ty(field_ty)?.is_some() {
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

fn get_each_attr(field: &Field) -> syn::Result<Option<String>> {
    let Some(attribute) = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("builder"))
    else {
        return Ok(None);
    };

    let expr: ExprAssign = attribute.parse_args()?;

    let Expr::Lit(lit) = expr.right.as_ref() else {
        panic!("builder attribute must be literal");
    };

    let Lit::Str(lit_str) = &lit.lit else {
        panic!("builder attribute must be string literal");
    };

    Ok(Some(lit_str.value()))
}
