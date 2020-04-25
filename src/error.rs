/// Check if a whole buffer was successfully transferred.
pub fn check_transfer(transferred: usize, total: usize) -> Result<(), IncompleteTransmissionError> {
	if transferred == total {
		Ok(())
	} else {
		Err(IncompleteTransmissionError { transferred, total })
	}
}

#[cfg(test)]
#[test]
fn test_check_transfer() {
	use assert2::assert;
	assert!(let Err(IncompleteTransmissionError { transferred: 1, total: 2}) = check_transfer(1, 2));
	assert!(let Err(IncompleteTransmissionError { transferred: 2, total: 1}) = check_transfer(2, 1));
	assert!(let Ok(()) = check_transfer(3, 3));
}

/// Error that may occur when receiving a message.
#[derive(Debug)]
pub enum ReceiveError {
	Io(std::io::Error),
	Decode(prost::DecodeError),
}

/// Error that may occur when sending a message.
#[derive(Debug)]
pub enum SendError {
	Io(std::io::Error),
	Encode(prost::EncodeError),
	IncompleteTransmission(IncompleteTransmissionError),
}

/// Error indicating that a message was only partially transmitted.
#[derive(Clone, Debug)]
pub struct IncompleteTransmissionError {
	/// The number of bytes that were transmitted.
	pub transferred: usize,

	/// The total number of bytes that should have been transmitted.
	pub total: usize,
}

impl From<std::io::Error> for ReceiveError {
	fn from(other: std::io::Error) -> Self {
		Self::Io(other)
	}
}

impl From<prost::DecodeError> for ReceiveError {
	fn from(other: prost::DecodeError) -> Self {
		Self::Decode(other)
	}
}

impl From<std::io::Error> for SendError {
	fn from(other: std::io::Error) -> Self {
		Self::Io(other)
	}
}

impl From<prost::EncodeError> for SendError {
	fn from(other: prost::EncodeError) -> Self {
		Self::Encode(other)
	}
}

impl From<IncompleteTransmissionError> for SendError {
	fn from(other: IncompleteTransmissionError) -> Self {
		Self::IncompleteTransmission(other)
	}
}

impl std::fmt::Display for ReceiveError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Io(e) => e.fmt(f),
			Self::Decode(e) => e.fmt(f),
		}
	}
}

impl std::fmt::Display for SendError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Io(e) => e.fmt(f),
			Self::Encode(e) => e.fmt(f),
			Self::IncompleteTransmission(e) => e.fmt(f),
		}
	}
}

impl std::fmt::Display for IncompleteTransmissionError {
	#[rustfmt::skip]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "incomplete transmission: transferred only {} of {} bytes",
			self.transferred,
			self.total
		)
	}
}

impl std::error::Error for ReceiveError {}
impl std::error::Error for SendError {}
impl std::error::Error for IncompleteTransmissionError {}
