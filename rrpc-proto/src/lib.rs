pub mod stubs;

pub struct FunctionCall {
    fn_name: String,
    args: Vec<String>,
}

pub trait Marshallable {
    /// Convert the data structure into bytes
    fn marshall(self) -> Vec<u8>;
}

pub trait Unmarshallable {
    /// Unmarshall the bytes into the data structure
    fn unmarshall(buf: &[u8]) -> Self;
}
