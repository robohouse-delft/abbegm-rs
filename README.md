# abbegm

This library implements the communication layer of ABB externally-guided motion.

Externally-guided motion (or EGM) is an interface for ABB robots to allow smooth control of a robotic arm from an external PC.
In order to use it, the *Externally Guided Motion* option (689-1) must be installed on the robot.

EGM can be used to stream positions to a robot in either joint space or cartesian space.
It can also be used to apply corrections to a pre-programmed trajectory.

A blocking peer is available as `sync_peer::EgmPeer`.

## Warning
Industrial robots are dangerous machines.
Sending poses to the robot using EGM may cause it to perform dangerous motions that could lead to damage, injuries or even death.

Always take appropriate precautions.
Stay out of reach of the robot when it is operational and always keep an emergency stop at hand when testing.

## Features
Some optional features are available.
Note that all features are enabled by default.
They can be disabled by specifying `default-features = false` in your dependency declaration.
Then you can enable only the features you need, to avoid unnecessary dependencies.

The available features are:
  * `tokio`: enables an asynchronous peer: `tokio_peer::EgmPeer`.
  * `nalgebra`: implements conversions to/from `nalgebra` types.

## Re-generating protobuf messages.

The Rust code for protobuf messages are generated using [`prost`](https://crates.io/crates/prost).
To re-generate the messages, run `cargo run --features generate-rust --bin generate-rust`.
