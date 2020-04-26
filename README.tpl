# {{crate}} [![docs][docs-badge]][docs] [![tests][tests-badge]][tests]
[docs]: https://docs.rs/abbegm/
[tests]: https://github.com/robohouse-delft/abbegm-rs/actions?query=workflow%3Atests
[docs-badge]: https://docs.rs/abbegm/badge.svg
[tests-badge]: https://github.com/robohouse-delft/abbegm-rs/workflows/tests/badge.svg

{{readme}}

## Re-generating protobuf messages.

The Rust code for the protobuf messages are generated using [`prost`](https://crates.io/crates/prost).
Normal users do not need to worry about this, but during development it may be necessary to re-generate the messages.
To do so, run the following command:

```sh
cargo run --features generate-rust --bin generate-rust
```
