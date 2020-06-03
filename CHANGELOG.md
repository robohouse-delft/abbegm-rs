v0.3.0
  * Remove angular velocity from `EgmCartesianSpeed` constructor.

v0.2.2
  * Add constructors to create message with speed reference.
  * Expose function to get a millseconds timestamp from `EgmClock`.
  * Deal with `EgmClock` microseconds overflowing into seconds when converting to Duration.
  * Move code generation program to separate crate.

v0.2.1
  * Add function to create a `tokio_peer::EgmPeer` synchronously.

v0.2.0
  * Add functions to create EGM messages.
  * Implement addition for `EgmClock` and `Duration`.
  * Document which units are used in the library overview.
  * Rename `motors_enabled` and `rapid_running` accessors.

v0.1.1
  * Tweak warning message regarding safety precautions.
  * Add readme to cargo manifest.

v0.1.0
  * Synchronous client using standard library.
  * Asynchronous client using `tokio` with the `tokio` feature.
  * Conversions between `nalgebra` and EGM messages with the `nalgebra` feature.
