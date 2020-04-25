# {{crate}}

{{readme}}

## Re-generating protobuf messages.

The Rust code for the protobuf messages are generated using [`prost`](https://crates.io/crates/prost).
To re-generate the messages, run the following command:

```sh
cargo run --features generate-rust --bin generate-rust
```
