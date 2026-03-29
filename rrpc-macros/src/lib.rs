use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse_macro_input};

mod client;
mod server;
mod utils;

#[proc_macro_attribute]
pub fn rrpc_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(input as ItemImpl);

    let client = client::gen_client_impl(&item_impl);
    let server = server::gen_server_impl(&item_impl);

    quote! {
        #item_impl
        #client
        #server
    }
    .into()
}
