extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Make a repr of Rust code as a syntax tree that
    // can be manipulated
    let ast = syn::parse(input).unwrap();

    // Make the trait impl
    impl_hello_macro(&ast)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
