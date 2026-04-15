use quote::{ToTokens, format_ident, quote};
use syn::{Ident, ItemTrait, TraitItemFn};

pub fn gen_server_impl(item: &ItemTrait) -> impl ToTokens {
    let server_ident = format_ident!("{}RpcServer", item.ident);
    let trait_name = &item.ident;
    let dispatcher = call_dispatcher(item);
    quote! {
        pub struct #server_ident<T: #trait_name + Send + Sync + 'static> {
            imp: ::std::sync::Arc<T>,
        }
        impl<T: #trait_name + Send + Sync + 'static> #server_ident<T> {
            pub fn new(imp: T) -> Self {
                Self {
                    imp: ::std::sync::Arc::new(imp),
                }
            }
            pub async fn listen(&self, addr: &str) {
                let imp = ::std::sync::Arc::clone(&self.imp);
                let stub = ::rrpc::__internal::ServerStub::default();
                stub.listen(addr.to_owned(), move |call| {
                    #dispatcher
                }).await;
            }
        }
    }
}

fn call_dispatcher(item: &ItemTrait) -> impl ToTokens {
    let enum_ident = format_ident!("{}Call", item.ident);
    let handlers: Vec<_> = item
        .items
        .iter()
        .filter_map(|item| match item {
            syn::TraitItem::Fn(f) => Some(variant_handler(f, &enum_ident)),
            _ => None,
        })
        .collect();
    quote! {
        match call {
            #(#handlers,)*
        }
    }
}

fn variant_handler(trait_item_fn: &TraitItemFn, enum_ident: &Ident) -> impl ToTokens {
    let params: Vec<_> = trait_item_fn
        .sig
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(i, input)| match input {
            syn::FnArg::Typed(_) => {
                let ident = format_ident!("param_{i}");
                Some(ident)
            }
            _ => None, // don't handle static methods (does it make sense to rpc call)
        })
        .collect();
    let func = &trait_item_fn.sig.ident;
    let variant_name = format_ident!("Variant{}", func);
    quote! {
        #enum_ident::#variant_name { #(#params,)* } => {
            let res = imp.#func(#(#params,)*);
            ::rrpc::__internal::postcard::to_allocvec(&res).unwrap()
        }
    }
}
