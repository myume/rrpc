use serde::{Deserialize, Serialize};

#[rrpc::service]
trait Interface {
    fn test(&self, test: u32) -> u32;
    fn test2(&self, i: String);
    fn test3(&self, i: String, j: usize, k: Option<u32>);
}

#[tokio::main]
async fn main() {
    let client = InterfaceRpcClient::new("127.0.0.1:3000");
    let server = InterfaceRpcServer::new();

    let res = client.test(32).await;
}
