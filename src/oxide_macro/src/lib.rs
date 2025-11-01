use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemImpl, ReturnType, Type, parse_macro_input};

const BEAN_IDENTIFIER: &'static str = "bean";

// #[proc_macro_attribute]
fn bean(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item.into()
}
// #[proc_macro_attribute]
fn config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    todo!()
}

fn get_output_type_of_sig(sig: &syn::Signature) -> Option<Type> {
    if let ReturnType::Type(_, b) = &sig.output {
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

#[proc_macro_derive(Component, attributes(wired))]
pub fn component(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    let name = &input.ident;

    // список полей с #[wired]
    let mut wired_fields = Vec::new();
    let mut dep_types = Vec::new();

    if let syn::Data::Struct(ds) = &input.data {
        if let syn::Fields::Named(fields) = &ds.fields {
            for field in &fields.named {
                if field.attrs.iter().any(|a| a.path().is_ident("wired")) {
                    let ident = field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    wired_fields.push((ident.clone(), ty.clone()));
                    dep_types.push(quote! { || std::any::TypeId::of::<#ty>() });
                }
            }
        }
    }

    // имя фабрики
    let ctor_fn = syn::Ident::new(&format!("__create_{}", name), name.span());
    let def_name = syn::Ident::new(&format!("__DEF_{}", name), name.span());
    let field_idents: Vec<_> = wired_fields.iter().map(|(i, _)| i).collect();
    let field_types: Vec<_> = wired_fields.iter().map(|(_, t)| t).collect();

    let g = quote! {
        // #input

        fn #ctor_fn(ctx: &Context) -> Box<dyn std::any::Any> {
            let instance = #name {
                #(
                    #field_idents: ctx.get::<#field_types>().unwrap().clone(),
                )*
            };
            Box::new(instance)
        }

        #[linkme::distributed_slice(ALL_BEANS)]
        static #def_name: ComponentDef = ComponentDef {
            bean_type: || std::any::TypeId::of::<#name>(),
            deps: &[ #( #dep_types ),* ],
            name: stringify!(#name),
            ctor: #ctor_fn,
        };
    };
    g.into()
}


