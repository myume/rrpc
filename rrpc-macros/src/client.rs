use quote::{ToTokens, format_ident, quote};
use syn::{ImplItem, ItemImpl, Visibility};

use crate::utils::type_ident;

pub fn gen_client_impl(item: &ItemImpl) -> impl ToTokens {
    let ident = type_ident(&item.self_ty).expect("unable to extract interface name");
    let client_ident = format_ident!("{ident}RpcClient");

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

fn gen_stub_method(item: &ImplItem) -> Option<impl ToTokens> {
    if let ImplItem::Fn(func) = item
        && matches!(func.vis, Visibility::Public(_))
    {
        let sig = &func.sig;
        let sig_tokens = sig.to_token_stream().to_string();
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
                println!("{}", #sig_tokens);
                // self.stub();
            }
        });
    }

    None
}
