v0.4.2
  * Accept nalgebra 0.21 and 0.22.

v0.4.1
  * Add methods to purge the socket read queue.

v0.4.0
  * Add `has_nan()` to check for NaN values in messages.
  * Check messages for NaN values before sending.

v0.3.0
  * Remove angular velocity from `EgmCartesianSpeed` constructor.
  * Add a few more `From<...>` implementations.

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
