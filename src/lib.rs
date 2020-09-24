#![feature(extend_one, iterator_fold_self)]

extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Builder)]
pub fn make_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let name_and_tys: Vec<_> = match item.fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .into_iter()
            .map(|x| {
                let ty = match x.ty {
                    syn::Type::Path(ty_path) => ty_path.path.segments,
                    _ => unimplemented!(),
                };
                (x.ident.unwrap(), ty[0].clone().ident)
            })
            .collect(),
        _ => unimplemented!(),
    };
    let struct_name = item.ident;
    let x = name_and_tys.into_iter().map(|(name, ty)| {
        quote! {
                impl #struct_name {
                    pub fn #name(self, val: #ty) -> Self {
                        Self {
                            #name: val,
                            ..self
                        }
                    }
                }
        }
    });
    x.fold_first(|acc, token| {
        let mut acc = acc;
        acc.extend_one(token);
        acc
    })
    .unwrap()
    .into()
}
