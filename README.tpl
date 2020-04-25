# {{crate}} [![docs][docs-badge]][docs] [![tests][tests-badge]][tests]
[docs]: https://docs.rs/abbegm/
[tests]: https://github.com/robohouse-delft/abbegm-rs/actions?query=workflow%3Atests
[docs-badge]: https://docs.rs/abbegm/badge.svg
[tests-badge]: https://github.com/robohouse-delft/abbegm-rs/workflows/CI/badge.svg

{{readme}}

## Re-generating protobuf messages.

The Rust code for the protobuf messages are generated using [`prost`](https://crates.io/crates/prost).
To re-generate the messages, run the following command:

```sh
cargo run --features generate-rust --bin generate-rust
```
