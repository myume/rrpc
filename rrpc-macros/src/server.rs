use quote::{ToTokens, format_ident, quote};
use syn::ItemImpl;

use crate::utils::type_ident;

pub fn gen_server_impl(item: &ItemImpl) -> impl ToTokens {
    let ident = type_ident(&item.self_ty).expect("unable to extract interface name");
    let server_ident = format_ident!("{ident}RpcServer");
    quote! {
        pub struct #server_ident {}

        impl #server_ident {
            pub fn new() -> Self {
                Self {}
            }
        }
    }
}
