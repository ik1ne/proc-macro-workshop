use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, ExprLit, Lit};

use crate::helper::{extract_option_type, get_each_from_builder_attribute, ty_inside_vec};

mod helper;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let impl_ident = impl_ident(&ident, &input.data);

    let struct_ident_builder = struct_ident_builder(&ident, &input.data);

    let impl_ident_builder = impl_ident_builder(&ident, &input.data);

    proc_macro::TokenStream::from(quote! {
        #impl_ident

        #struct_ident_builder

        #impl_ident_builder
    })
}

fn impl_ident(ident: &Ident, data: &Data) -> TokenStream {
    let ident_builder = format_ident!("{}Builder", ident);

    let ident_builder_fields = helper::map_fields(data, |field| {
        let field_ident = &field.ident;

        if let Ok(Some(_)) = get_each_from_builder_attribute(field) {
            quote_spanned! {field.span()=>
                #field_ident: vec![]
            }
        } else {
            quote_spanned! {field.span()=>
                #field_ident: None
            }
        }
    });

    quote! {
        impl #ident {
            pub fn builder() -> #ident_builder {
                #ident_builder {
                    #(#ident_builder_fields),*
                }
            }
        }
    }
}

fn struct_ident_builder(ident: &Ident, data: &Data) -> TokenStream {
    let ident_builder = format_ident!("{}Builder", ident);

    let struct_fields = helper::map_fields(data, |field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        if extract_option_type(field_ty).is_some() {
            quote_spanned! {field.span()=>
                #field_ident: #field_ty
            }
        } else if let Ok(Some(_)) = get_each_from_builder_attribute(field) {
            quote_spanned! {field.span()=>
                #field_ident: #field_ty
            }
        } else {
            quote_spanned! {field.span()=>
                  #field_ident: Option<#field_ty>
            }
        }
    });

    quote! {
        struct #ident_builder {
            #(#struct_fields),*
        }
    }
}

fn impl_ident_builder(ident: &Ident, data: &Data) -> TokenStream {
    let ident_builder = format_ident!("{}Builder", ident);

    let setter = impl_ident_builder_setter(data);

    let builder = impl_ident_builder_build(ident, data);

    quote! {
        impl #ident_builder {
            #setter

            #builder
        }
    }
}

fn impl_ident_builder_setter(data: &Data) -> TokenStream {
    let setters = helper::map_fields(data, |field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        if let Some(option_ty) = extract_option_type(field_ty) {
            quote_spanned! {field.span()=>
                pub fn #field_ident(&mut self, #field_ident: #option_ty) -> &mut Self {
                    self.#field_ident = Some(#field_ident);
                    self
                }
            }
        } else {
            if let Ok(Some(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            })) = get_each_from_builder_attribute(field)
            {
                if *field_ident.as_ref().unwrap() == lit_str.value() {
                    return quote! {};
                } else {
                    return quote_spanned! {field.span()=>
                        pub fn #field_ident(&mut self, #field_ident: #field_ty) -> &mut Self {
                            self.#field_ident = #field_ident;
                            self
                        }
                    };
                }
            }

            quote_spanned! {field.span()=>
                pub fn #field_ident(&mut self, #field_ident: #field_ty) -> &mut Self {
                    self.#field_ident = Some(#field_ident);
                    self
                }
            }
        }
    });

    let each_setters = helper::map_fields(data, |field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        let Ok(Some(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        })) = get_each_from_builder_attribute(field)
        else {
            return quote! {};
        };

        let field_ty_inside_vec = ty_inside_vec(field_ty);

        let fn_ident = format_ident!("{}", lit_str.value());

        quote_spanned! {field.span()=>
            pub fn #fn_ident(&mut self, val: #field_ty_inside_vec) -> &mut Self {
                self.#field_ident.push(val);
                self
            }
        }
    });

    quote! {
        #(#setters)*

        #(#each_setters)*
    }
}

fn impl_ident_builder_build(ident: &Ident, data: &Data) -> TokenStream {
    let local_vars = local_vars(data);

    let ok_ident = ok_ident(ident, data);

    quote! {
        pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
            #local_vars

            #ok_ident
        }
    }
}

fn local_vars(data: &Data) -> TokenStream {
    let lines = helper::map_fields(data, |field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        if extract_option_type(field_ty).is_some() {
            quote_spanned! {field.span() =>
                let #field_ident = self.#field_ident.take();
            }
        } else if let Ok(Some(_)) = get_each_from_builder_attribute(field) {
            quote_spanned! {field.span()=>
                let #field_ident = self.#field_ident.drain(..).collect();
            }
        } else {
            quote_spanned! {field.span()=>
                let #field_ident = self.#field_ident.take().ok_or("#field_ident is required")?;
            }
        }
    });

    quote! {
        #(#lines)*
    }
}

fn ok_ident(ident: &Ident, data: &Data) -> TokenStream {
    let fields = helper::map_fields(data, |field| {
        let field_ident = &field.ident;

        quote_spanned! {field.span()=>
            #field_ident,
        }
    });

    quote! {
        Ok(#ident {
            #(#fields)*
        })
    }
}
