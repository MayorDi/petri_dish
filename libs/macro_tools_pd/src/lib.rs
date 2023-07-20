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

#[proc_macro_derive(GetPosition)]
pub fn get_pos(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_get_pos_macro(&ast)
}

fn impl_get_pos_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl GetPosition for #name {
            fn get_position(&self) -> Position {
                self.position
            }
        }
    };

    gen.into()
}