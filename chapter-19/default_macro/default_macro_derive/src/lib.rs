extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data};
use syn::DeriveInput;

#[proc_macro_derive(MyDefault)]
pub fn my_default_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    return impl_my_default(&ast);
}

fn impl_my_default(ast: &syn::DeriveInput) -> TokenStream {
    let id = &ast.ident;

    let Data::Struct(s) = &ast.data else {
        panic!("MyDefault derive macro must be use in struct");
    };

    let mut field_ast = quote!();
    for (idx, f) in s.fields.iter().enumerate() {
        let (field_id, field_type) = (&f.ident, &f.ty);
        if field_id.is_none() {
            let field_idx = syn::Index::from(idx);
            field_ast.extend(quote! {
                #field_idx: #field_type::default(),
            });
        } else {
            field_ast.extend(quote! {
                #field_id: #field_type::default(),
            });
        }

    }
    return quote! {
        impl Default for # id {
            fn default() -> Self {
                return Self {
                    #field_ast
                };
            }
        }
    }.into();
}
