use quote::{ToTokens, format_ident, quote};
use syn::ItemTrait;

pub fn gen_server_impl(item: &ItemTrait) -> impl ToTokens {
    let server_ident = format_ident!("{}RpcServer", item.ident);
    quote! {
        pub struct #server_ident {}

        impl #server_ident {
            pub fn new() -> Self {
                Self {}
            }
        }
    }
}
