# RRPC

Strongly typed rust to rust rpc.

## Usage

See [examples](./examples) for example usages.

The basic usage is just adding the macro

```rust
#[rrpc::service]
pub trait SharedTrait {
  ...
}
```

above any shared interfaces between your rpc client and server. This will
generated a client and server implementation with the trait name preprended to
`RpcClient`/`RpcServer`.
