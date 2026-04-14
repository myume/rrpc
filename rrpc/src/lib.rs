pub use rrpc_macros::service;

#[doc(hidden)]
pub mod __internal {
    pub use rrpc_core::stubs::ClientStub;
}
