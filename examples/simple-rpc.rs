#[rrpc_macros::service]
trait Interface {
    fn test(&self, test: u32);
    fn test2(&self, i: String);
    fn test3(&self, i: String, j: usize, k: Option<u32>);
}

fn main() {
    let client = InterfaceRpcClient::new();
    let server = InterfaceRpcServer::new();
}
