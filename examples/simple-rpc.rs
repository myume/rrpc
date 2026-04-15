#[rrpc::service]
pub trait Interface: Send + Sync {
    fn test(&self, test: u32) -> u32;
    fn test2(&self, i: String) -> String;
    fn test3(&self, a: u32, b: u32) -> u32;
}

#[derive(Default)]
pub struct InterfaceImpl {}
impl Interface for InterfaceImpl {
    fn test(&self, i: u32) -> u32 {
        i + 1
    }

    fn test2(&self, _: String) -> String {
        "World".into()
    }

    fn test3(&self, a: u32, b: u32) -> u32 {
        a * a + b * b
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let client = InterfaceRpcClient::new(addr);
    let mut server = InterfaceRpcServer::bind(addr).await.unwrap();

    tokio::spawn(async move {
        server.listen(InterfaceImpl::default()).await;
    });

    assert_eq!(client.test(3).await, 4);
    assert_eq!(client.test2("Hello".into()).await, "World");
    assert_eq!(client.test3(2, 3).await, 13);
}
