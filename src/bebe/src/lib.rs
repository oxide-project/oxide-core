use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

#[proc_macro_attribute]
pub fn collect_beans(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(item as ItemMod);

    let mut registrations = Vec::new();

    if let Some((_, ref items)) = module.content {
        for i in items {
            if let syn::Item::Fn(f) = i {
                if f.attrs.iter().any(|a| a.path().is_ident("bean")) {
                    let fn_name = &f.sig.ident;
                    let ret_type = &f.sig.output; // предполагаем простой тип
                    registrations.push(quote! {
                        ctx.register::<#ret_type>(|| #fn_name());
                    });
                }
            }
        }
    }

    let expanded = quote! {
        #module
        pub fn register_beans(ctx: &mut crate::context::ApplicationContext) {
            #(#registrations)*
        }
    };

    TokenStream::from(expanded)
}