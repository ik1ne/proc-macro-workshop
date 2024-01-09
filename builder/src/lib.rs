use proc_macro2::{Ident, TokenStream};
use quote::__private::ext::RepToTokensExt;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, GenericArgument, PathArguments, Type,
};

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

    let ident_builder_fields = map_fields(data, |field| {
        let field_ident = &field.ident;

        quote_spanned! {field.span()=>
            #field_ident: None
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

    let struct_fields = map_fields(data, |field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        if extract_option_type(field_ty).is_some() {
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
    let setters = map_fields(data, |field| {
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
            quote_spanned! {field.span()=>
                pub fn #field_ident(&mut self, #field_ident: #field_ty) -> &mut Self {
                    self.#field_ident = Some(#field_ident);
                    self
                }
            }
        }
    });

    quote! {
        #(#setters)*
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
    let lines = map_fields(data, |field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        if extract_option_type(field_ty).is_some() {
            quote_spanned! {field.span() =>
                let #field_ident = self.#field_ident.take();
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
    let fields = map_fields(data, |field| {
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

fn map_fields<'a>(
    data: &'a Data,
    f: impl Fn(&Field) -> TokenStream + 'a,
) -> Box<dyn Iterator<Item = TokenStream> + 'a> {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Box::new(fields.named.iter().map(f)),
            _ => todo!(),
        },
        _ => unimplemented!("only struct is supported"),
    }
}

fn extract_option_type(ty: &Type) -> Option<&Type> {
    let Type::Path(path) = ty else {
        return None;
    };

    let first_segment = path.path.segments.first()?;
    if first_segment.ident != "Option" {
        return None;
    }

    let first_argument = first_segment.arguments.next()?;
    let PathArguments::AngleBracketed(first_angle_bracketed) = first_argument else {
        return None;
    };

    let generic_arg = first_angle_bracketed.args.first()?;
    let GenericArgument::Type(ty) = generic_arg else {
        return None;
    };

    Some(ty)
}
