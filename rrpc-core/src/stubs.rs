use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct ClientStub<S: ToSocketAddrs> {
    addr: S,
}

impl<S: ToSocketAddrs> ClientStub<S> {
    pub fn new(conn: S) -> Self {
        ClientStub { addr: conn }
    }

    // TODO: handle errors properly
    pub async fn send<R: DeserializeOwned, T: Serialize>(&self, call: T) -> R {
        let mut stream = TcpStream::connect(&self.addr).await.unwrap();

        let bytes = postcard::to_allocvec(&call).unwrap();
        stream.write_all(&bytes).await.unwrap();

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await.unwrap();
        postcard::from_bytes(&buf).unwrap()
    }
}
