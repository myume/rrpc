use std::io;

use log::error;
use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error("Failed to serialize RPC call: {0}")]
    BadRequest(#[source] postcard::Error),
    #[error("Failed to deserialize RPC response: {0}")]
    BadResponse(#[source] postcard::Error),
    #[error("Failed to communicate with destination: {0}")]
    CommunicationFailure(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, RpcError>;

pub struct ClientStub {}

impl ClientStub {
    pub async fn send<S: ToSocketAddrs, R: DeserializeOwned, T: Serialize>(
        addr: S,
        call: T,
    ) -> Result<R> {
        let mut stream = TcpStream::connect(addr).await?;

        let bytes = postcard::to_allocvec(&call).map_err(RpcError::BadRequest)?;
        stream.write_u32(bytes.len() as u32).await?;
        stream.write_all(&bytes).await?;

        let len = stream.read_u32().await? as usize;
        let mut buf = vec![0; len];
        stream.read_exact(&mut buf).await?;
        postcard::from_bytes(&buf).map_err(RpcError::BadResponse)
    }
}

pub struct ServerStub {
    listener: TcpListener,
}

impl ServerStub {
    pub async fn bind<S: ToSocketAddrs>(addr: S) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }

    async fn handle_request<T, F>(mut stream: TcpStream, handler: F) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
        F: Fn(T) -> postcard::Result<Vec<u8>> + Send + Clone + 'static,
    {
        let len = stream.read_u32().await? as usize;
        let mut buf = vec![0; len];
        stream.read_exact(&mut buf).await?;
        let req: T = postcard::from_bytes(&buf).map_err(RpcError::BadRequest)?;
        let res = handler(req).map_err(RpcError::BadResponse)?;
        stream.write_u32(res.len() as u32).await?;
        stream.write_all(&res).await?;
        Ok(())
    }

    pub async fn listen_with<T, F>(&self, handler: F) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
        F: Fn(T) -> postcard::Result<Vec<u8>> + Send + Clone + 'static,
    {
        loop {
            let (stream, _) = self.listener.accept().await?;
            let handler = handler.clone();
            tokio::spawn(async move {
                if let Err(e) = ServerStub::handle_request(stream, handler).await {
                    error!("{}", e);
                }
            });
        }
    }
}
