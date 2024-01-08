use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
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

    let ident_builder_fields = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_ident = &field.ident;

                    quote_spanned! {field.span()=>
                        #field_ident: None,
                    }
                });
                quote! {
                    #(#fields)*
                }
            }
            _ => todo!(),
        },
        _ => unimplemented!("only struct is supported"),
    };

    quote! {
        impl #ident {
            pub fn builder() -> #ident_builder {
                #ident_builder {
                    #ident_builder_fields
                }
            }
        }
    }
}

fn struct_ident_builder(ident: &Ident, data: &Data) -> TokenStream {
    let ident_builder = format_ident!("{}Builder", ident);

    let struct_fields = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_ident = &field.ident;
                    let field_type = &field.ty;

                    quote_spanned! {field.span()=>
                        #field_ident: Option<#field_type>
                    }
                });
                quote! {
                    #(#fields),*
                }
            }
            Fields::Unnamed(fields) => {
                let fields = fields.unnamed.iter().enumerate().map(|(i, field)| {
                    let field_ident = format_ident!("field_{}", i);
                    let field_type = &field.ty;

                    quote_spanned! {field.span() =>
                        #field_ident: Option<#field_type>
                    }
                });
                quote! {
                    #(#fields),*
                }
            }
            Fields::Unit => quote! {},
        },
        _ => unimplemented!("only struct is supported"),
    };

    quote! {
        struct #ident_builder {
            #struct_fields
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
    let impl_body = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let method = fields.named.iter().map(|field| {
                    let field_ident = &field.ident;
                    let field_ty = &field.ty;

                    quote_spanned! {field.span()=>
                        pub fn #field_ident(&mut self, #field_ident: #field_ty) -> &mut Self {
                            self.#field_ident = Some(#field_ident);
                            self
                        }
                    }
                });
                quote! {
                    #(#method)*
                }
            }
            Fields::Unnamed(_fields) => {
                todo!()
            }
            Fields::Unit => quote! {},
        },
        _ => unimplemented!("only struct is supported"),
    };

    quote! {
            #impl_body
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
    let lines = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().map(|field| {
                let field_ident = &field.ident;
                let field_ty = &field.ty;
                let opt = field_ty
                    .to_token_stream()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .swap_remove(0);

                if let TokenTree::Ident(ident) = opt {
                    eprintln!("{}", ident == "Option");
                };

                quote_spanned! {field.span()=>
                    let #field_ident = self.#field_ident.take().ok_or("#field_ident is required")?;
                }
            }),
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
        },
        _ => unimplemented!("only struct is supported"),
    };

    quote! {
        #(#lines)*
    }
}

fn ok_ident(ident: &Ident, data: &Data) -> TokenStream {
    let fields = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter().map(|field| {
                let field_ident = &field.ident;

                quote_spanned! {field.span()=>
                    #field_ident,
                }
            }),
            _ => todo!(),
        },
        _ => unimplemented!("only struct is supported"),
    };

    quote! {
        Ok(#ident {
            #(#fields)*
        })
    }
}
