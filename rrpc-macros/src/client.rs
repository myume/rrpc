use quote::{ToTokens, format_ident, quote};
use syn::{ItemTrait, TraitItem};

pub fn gen_client_impl(item: &ItemTrait) -> impl ToTokens {
    let client_ident = format_ident!("{}RpcClient", item.ident);

    let stubs: Vec<_> = item.items.iter().filter_map(gen_stub_method).collect();

    quote! {
        pub struct #client_ident {}

        impl #client_ident {
            pub fn new() -> Self {
                Self {}
            }

            #(#stubs)*
        }
    }
}

fn gen_stub_method(item: &TraitItem) -> Option<impl ToTokens> {
    if let TraitItem::Fn(func) = item {
        let sig = &func.sig;
        for arg in &sig.inputs {
            match arg {
                syn::FnArg::Receiver(receiver) => {
                    println!("receiver: {}", receiver.to_token_stream());
                }
                syn::FnArg::Typed(pat_type) => {
                    println!("typed: {}", pat_type.to_token_stream());
                }
            }
        }

        return Some(quote! {
            pub #sig {
                // 1. create RPC request

                // 2. fire request to server

                // 3. handle response
            }
        });
    }

    None
}
