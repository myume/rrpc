#[rrpc_macros::service]
trait Interface {
    fn test(&self, test: u32);
    fn test2(&self, i: &str);
}

fn main() {
    let client = InterfaceRpcClient::new();
    let server = InterfaceRpcServer::new();
}
