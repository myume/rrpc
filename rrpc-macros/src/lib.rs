use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{ItemTrait, TraitItemFn, parse_macro_input};

mod client;
mod server;

#[proc_macro_attribute]
pub fn service(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(input as ItemTrait);

    let client = client::gen_client_impl(&trait_def);
    let server = server::gen_server_impl(&trait_def);
    let call_variants = func_variants(&trait_def);

    quote! {
        #trait_def

        #call_variants

        #client

        #server
    }
    .into()
}

fn func_variants(item: &ItemTrait) -> impl ToTokens {
    let enum_variants: Vec<_> = item
        .items
        .iter()
        .filter_map(|item| match item {
            syn::TraitItem::Fn(trait_item_fn) => Some(build_variant(trait_item_fn)),
            _ => None,
        })
        .collect();

    let enum_ident = format_ident!("{}Call", item.ident);
    let trait_generics = &item.generics;
    let variants = quote! {
        #[derive(Debug, Serialize, Deserialize)]
        pub enum #enum_ident #trait_generics{
            #(#enum_variants,)*
        }
    };

    variants
}

fn build_variant(trait_item_fn: &TraitItemFn) -> impl ToTokens {
    let params: Vec<_> = trait_item_fn
        .sig
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(i, input)| match input {
            syn::FnArg::Typed(pat_type) => {
                let ident = format_ident!("param_{i}");
                let ty = &pat_type.ty;
                Some(quote! {
                    #ident: #ty
                })
            }
            _ => None, // don't handle static methods (does it make sense to rpc call)
        })
        .collect();

    let variant_name = format_ident!("Variant{}", trait_item_fn.sig.ident);
    quote! {
        #variant_name {
            #(#params,)*
        }
    }
}
