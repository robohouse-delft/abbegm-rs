# abbegm [![docs][docs-badge]][docs] [![tests][tests-badge]][tests]
[docs]: https://docs.rs/abbegm/
[tests]: https://github.com/robohouse-delft/abbegm-rs/actions?query=workflow%3Atests
[docs-badge]: https://docs.rs/abbegm/badge.svg
[tests-badge]: https://github.com/robohouse-delft/abbegm-rs/workflows/CI/badge.svg

This library implements the communication layer of ABB externally-guided motion.

Externally-guided motion (or EGM) is an interface for ABB robots to allow smooth control of a robotic arm from an external PC.
In order to use it, the *Externally Guided Motion* option (689-1) must be installed on the robot controller.

EGM can be used to stream positions to a robot controller in either joint space or cartesian space.
It can also be used to apply corrections to a pre-programmed trajectory.

To communicate with a robot controller in blocking mode, use `sync_peer::EgmPeer`.
Use `tokio_peer::EgmPeer` if you want to communicate with a robot controller asynchronously.

## Warning
Industrial robots are dangerous machines.
Sending poses to the robot using EGM may cause it to perform dangerous motions that could lead to damage, injuries or even death.

Always take appropriate precautions.
Make sure there are no persons or animals in reach of the robot when it is operational and always keep an emergency stop at hand when testing.

## Units and rotation conventions
Unless specified differently, all distances and positions are in millimeters and all angles are in degrees.
This may be somewhat odd, but it is what the robot controller expects.

You should use unit quaternions to represent 3D orientations, not Euler angles or roll-pitch-yaw.
Using unit quaternions avoids the need to specify which Euler angles or roll-pitch-yaw representation is used.
Quaternions also have the added advantage that you don't need to use degrees.

## Features
Some optional features are available.
Note that all features are enabled by default.
They can be disabled by specifying `default-features = false` in your dependency declaration.
Then you can enable only the features you need, to avoid unnecessary dependencies.

The available features are:
  * `tokio`: enable the asynchronous peer.
  * `nalgebra`: implement conversions between `nalgebra` types and EGM messages.

## Re-generating protobuf messages.

The Rust code for the protobuf messages are generated using [`prost`](https://crates.io/crates/prost).
To re-generate the messages, run the following command:

```sh
cargo run --features generate-rust --bin generate-rust
```
