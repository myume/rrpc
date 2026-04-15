pub use rrpc_macros::service;

#[doc(hidden)]
pub mod __internal {
    pub use postcard;
    pub use rrpc_core::stubs::{ClientStub, ServerStub};
    pub use serde;
    pub use tokio;
}
