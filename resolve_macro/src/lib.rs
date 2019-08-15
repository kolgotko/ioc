extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{ parse, Ident, ItemImpl, ImplItem };

#[proc_macro_attribute]
pub fn resolvable_via_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemImpl = parse(item).unwrap();
    let self_type = &ast.self_ty;
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics Resolvable for #self_type {
            fn resolve(container: &Container) -> Self {
                <Self as std::default::Default>::default()
            }
        }
        #ast
    };

    return gen.into();
}

#[proc_macro_attribute]
pub fn resolvable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let constructor_name: Ident = parse(attr).unwrap();
    let ast: ItemImpl = parse(item.clone()).unwrap();
    let self_type = &ast.self_ty;
    let generics = &ast.generics;

    let constructor_method = ast.items.clone()
        .into_iter()
        .filter(|item| {
            match item {
                ImplItem::Method(_) => true,
                _ => false,
            }
        })
        .find(|method| {
            if let ImplItem::Method(method) = method {
                method.sig.ident == constructor_name
            } else { false }
        });

    let constructor_method = match constructor_method {
        Some(ImplItem::Method(method)) => method,
        _ => panic!("constructor not found"),
    };

    let inputs_len = constructor_method.sig.decl.inputs.len();
    let mut args = quote! {};

    for _ in 0..inputs_len {
        args = quote! {
            #args
            container.try_resolve().unwrap(),
        }
    }

    let gen = quote! {
        impl #generics Resolvable for #self_type {
            fn resolve(container: &Container) -> Self {
                Self::#constructor_name(#args)
            }
        }
        #ast
    };

    gen.into()
}
