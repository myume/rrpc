use quote::{ToTokens, format_ident, quote};
use syn::{Ident, ItemTrait, PatType, TraitItem};

pub fn gen_client_impl(item: &ItemTrait) -> impl ToTokens {
    let client_ident = format_ident!("{}RpcClient", item.ident);

    let enum_ident = format_ident!("{}Call", item.ident);
    let stubs: Vec<_> = item
        .items
        .iter()
        .filter_map(|func| gen_stub_method(func, &enum_ident))
        .collect();

    let generics = quote! { <S: ::rrpc::__internal::tokio::net::ToSocketAddrs> };

    quote! {
        pub struct #client_ident #generics{
            server_addr: S
        }

        impl #generics #client_ident<S>{
            pub fn new(addr: S) -> Self {
                Self {
                    server_addr: addr
                }
            }


            #(#stubs)*
        }
    }
}

fn gen_stub_method(item: &TraitItem, enum_ident: &Ident) -> Option<impl ToTokens> {
    let TraitItem::Fn(func) = item else {
        return None;
    };

    let mut sig = func.sig.clone();
    sig.asyncness = Some(syn::token::Async::default());

    let params: Vec<_> = sig
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(i, arg)| match arg {
            syn::FnArg::Typed(PatType { pat, .. }) => match pat.as_ref() {
                syn::Pat::Ident(pat_ident) => {
                    let param_ident = format_ident!("param_{i}");
                    let param = quote! {
                        #param_ident: #pat_ident
                    };
                    Some(param)
                }
                _ => None,
            },
            _ => None,
        })
        .collect();

    let variant_name = format_ident!("Variant{}", sig.ident);
    let variant = quote! {
        #enum_ident::#variant_name { #(#params),* }
    };

    Some(quote! {
        pub #sig {
            // 1. create RPC request
            let call = #variant;

            // 2. fire request to server
            ::rrpc::__internal::ClientStub::send(&self.server_addr, call).await

            // 3. handle response
        }
    })
}
