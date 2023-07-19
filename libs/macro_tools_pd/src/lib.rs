use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ArrayPosition)]
pub fn arr_pos(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_arr_pos_macro(&ast)
}

fn impl_arr_pos_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ArrayPosition for #name {}
    };
    gen.into()
}
