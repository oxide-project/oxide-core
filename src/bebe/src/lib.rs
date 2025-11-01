mod di;
mod util;

use proc_macro::TokenStream;
use quote::quote;
use std::any::TypeId;
use std::arch::naked_asm;
use std::collections::{HashMap, HashSet};
use syn::{FnArg, ImplItem, ItemImpl, ItemStruct, ReturnType, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let g = quote! {
        #input
        impl Default for #name {
            fn default() -> Self {
                Self {}
            }
        }

        inventory::submit! {
            #![crate = bebe]
            bebe::di::ComponentRegistration::new::<#name>()
        }
        inventory::collect!{

        }
    };

    g.into()
}

const BEAN_IDENTIFIER: &'static str = "bean";

#[proc_macro_attribute]
pub fn config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let mut bean_graph: HashMap<_, Vec<_>> = Default::default();
    for item in input.items {
        // если это функция
        if let ImplItem::Fn(f) = item {
            // и функция помечена как bean
            let output_type = if let Some(t) = get_output_type_of_sig(&f.sig) {
                t
            } else {
                continue;
            };
            let input_types = get_input_types_of_sig(&f.sig);
            bean_graph.insert(output_type, input_types);
        }
    }
    todo!()
}

fn get_output_type_of_sig(sig: &syn::Signature) -> Option<Type> {
    if let ReturnType::Type(a, b) = &sig.output {
        let t = *(b.clone());
        Some(t)
    } else {
        None
    }
}

fn get_input_types_of_sig(sig: &syn::Signature) -> Vec<Type> {
    let input_types = sig
        .inputs
        .iter()
        .filter_map(|it| {
            if let FnArg::Typed(t) = it {
                Some(*t.ty.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    input_types
}
