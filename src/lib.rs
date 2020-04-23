mod generated;

/// protobuf messages used by EGM
pub mod msg {
	pub use super::generated::*;
}

#[cfg(feature = "tokio")]
pub mod tokio_peer;

#[cfg(feature = "nalgebra")]
pub mod nalgebra;
