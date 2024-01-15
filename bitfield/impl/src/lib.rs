use proc_macro::TokenStream;

mod generate;

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}

#[proc_macro]
pub fn generate_bitfield_types(_input: TokenStream) -> TokenStream {
    generate::generate_bitfield_types().into()
}
