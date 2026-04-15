use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct ClientStub {}

impl ClientStub {
    // TODO: handle errors properly
    pub async fn send<S: ToSocketAddrs, R: DeserializeOwned, T: Serialize>(addr: S, call: T) -> R {
        let mut stream = TcpStream::connect(addr).await.unwrap();

        let bytes = postcard::to_allocvec(&call).unwrap();
        stream.write_u32(bytes.len() as u32).await.unwrap();
        stream.write_all(&bytes).await.unwrap();

        let len = stream.read_u32().await.unwrap() as usize;
        let mut buf = vec![0; len];
        stream.read_exact(&mut buf).await.unwrap();
        postcard::from_bytes(&buf).unwrap()
    }
}

pub struct ServerStub {
    listener: TcpListener,
}

impl ServerStub {
    pub async fn bind<S: ToSocketAddrs>(addr: S) -> Self {
        let listener = TcpListener::bind(addr).await.unwrap();
        Self { listener }
    }

    pub async fn listen_with<T, F>(&self, handler: F)
    where
        T: DeserializeOwned + Send + 'static,
        F: Fn(T) -> Vec<u8> + Send + Clone + 'static,
    {
        loop {
            let (mut stream, _) = self.listener.accept().await.unwrap();
            let handler = handler.clone();
            tokio::spawn(async move {
                let len = stream.read_u32().await.unwrap() as usize;
                let mut buf = vec![0; len];
                stream.read_exact(&mut buf).await.unwrap();
                let req: T = postcard::from_bytes(&buf).unwrap();
                let res = handler(req);
                stream.write_u32(res.len() as u32).await.unwrap();
                stream.write_all(&res).await.unwrap();
            });
        }
    }
}
