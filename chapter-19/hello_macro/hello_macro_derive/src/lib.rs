extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

// https://course.rs/advance/macro.html
// 调用逻辑: [derive(HelloMacro)] -> [proc_macro_derive(Hello)] -> hello_macro_derive
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    return impl_hello_macro(&ast);
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    return gen.into();
}
