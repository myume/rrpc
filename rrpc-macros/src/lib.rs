use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{ItemTrait, parse_macro_input};

mod client;
mod server;

#[proc_macro_attribute]
pub fn service(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(input as ItemTrait);

    let client = client::gen_client_impl(&trait_def);
    let server = server::gen_server_impl(&trait_def);
    let function_calls = function_call_enum(&trait_def);

    quote! {
        #trait_def

        #function_calls
        #client
        #server
    }
    .into()
}

fn function_call_enum(item: &ItemTrait) -> impl ToTokens {
    let enum_items: Vec<_> = item
        .items
        .iter()
        .filter_map(|item| match item {
            syn::TraitItem::Fn(trait_item_fn) => {
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
                let variant = quote! {
                    #variant_name {
                        #(#params,)*
                    }
                };
                Some(variant)
            }
            _ => None,
        })
        .collect();

    let enum_name = format_ident!("{}Call", item.ident);
    let function_calls = quote! {
        pub enum #enum_name {
            #(#enum_items,)*
        }
    };

    function_calls
}
