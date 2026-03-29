use rrpc_macros::rrpc_impl;

struct Test {}

#[rrpc_impl]
impl Test {
    pub fn testing(&self, i: u32) {
        println!("we're testing here {i}");
        self.private();
    }

    fn private(&self) {
        println!("doing some private work");
    }
}

fn main() {
    let test = Test {};
    test.testing(1);
    let client = TestRpcClient::new();
    client.testing(2);
    let server = TestRpcServer::new();
}
