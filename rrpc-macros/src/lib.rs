use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemTrait, parse_macro_input};

mod client;
mod server;

#[proc_macro_attribute]
pub fn service(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(input as ItemTrait);

    let client = client::gen_client_impl(&trait_def);
    let server = server::gen_server_impl(&trait_def);

    quote! {
        #trait_def
        #client
        #server
    }
    .into()
}
