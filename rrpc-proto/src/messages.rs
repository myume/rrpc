pub struct FunctionCall {}

pub trait RpcRequest {
    fn marshall(&self);
}

pub trait RpcResponse {
    fn unmarshall(&self) -> FunctionCall;
}
