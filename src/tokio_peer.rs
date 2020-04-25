use std::net::SocketAddr;

use prost::Message;
use tokio::net::UdpSocket;
use tokio::net::udp;

use crate::msg::EgmSensor;
use crate::msg::EgmRobot;

#[derive(Debug)]
pub struct EgmPeer {
	rx: EgmReceiver,
	tx: EgmSender,
}

#[derive(Debug)]
pub struct EgmReceiver {
	inner: udp::RecvHalf,
}

#[derive(Debug)]
pub struct EgmSender {
	inner: udp::SendHalf,
}

#[derive(Debug)]
pub enum ReceiveError {
	Io(std::io::Error),
	Decode(prost::DecodeError),
}

#[derive(Debug)]
pub enum SendError {
	Io(std::io::Error),
	Encode(prost::EncodeError),
	IncompleteTransmission(IncompleteTransmissionError),
}

#[derive(Clone, Debug)]
pub struct IncompleteTransmissionError {
	pub transferred: usize,
	pub total: usize,
}

/// An EGM peer capable of sending and receiving messages.
impl EgmPeer {
	/// Wrap an existing UDP socket in a peer.
	///
	/// If you want to use the [`EgmPeer::recv`] and [`EgmPeer::send`] functions,
	/// you should use an already connected socket.
	/// Otherwise, you can only use [`EgmPeer::recv_from`] and [`EgmPeer::send_to`].
	pub fn new(peer: UdpSocket) -> Self {
		let (rx, tx) = peer.split();
		Self {
			rx: EgmReceiver::new(rx),
			tx: EgmSender::new(tx),
		}
	}

	/// Consume self and return the original socket.
	pub fn into_inert(self) -> UdpSocket {
		let rx = self.rx.into_inner();
		let tx = self.tx.into_inner();
		rx.reunite(tx).unwrap()
	}

	/// Split the peer into a receiver and a sender.
	///
	/// This can be useful if you want to split receiving and sending into separate tasks.
	pub fn split(self) -> (EgmReceiver, EgmSender) {
		(self.rx, self.tx)
	}

	/// Receive a message from the remote address to which the unerlying socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn recv(&mut self) -> Result<EgmRobot, ReceiveError> {
		self.rx.recv().await
	}

	/// Receive a message from any remote address.
	pub async fn recv_from(&mut self, ) -> Result<(EgmRobot, SocketAddr), ReceiveError> {
		self.rx.recv_from().await
	}

	/// Send a message to the remote address to which the unerlying socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn send(&mut self, msg: &EgmSensor) -> Result<(), SendError> {
		self.tx.send(msg).await
	}

	/// Send a message to the specified address.
	pub async fn send_to(&mut self, msg: &EgmSensor, target: &SocketAddr) -> Result<(), SendError> {
		self.tx.send_to(msg, target).await
	}
}

impl EgmReceiver {
	/// Create an EGM receiver from the receive half of a UDP socket.
	pub fn new(inner: udp::RecvHalf) -> Self {
		Self { inner }
	}

	/// Consume self and return the original half of the UDP socket.
	pub fn into_inner(self) -> udp::RecvHalf {
		self.inner
	}

	/// Receive a message from the remote address to which the unerlying socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn recv(&mut self) -> Result<EgmRobot, ReceiveError> {
		let mut buffer = vec![0u8; 256];
		let bytes_received = self.inner.recv(&mut buffer).await?;
		Ok(EgmRobot::decode(&buffer[..bytes_received])?)
	}

	/// Receive a message from any remote address.
	pub async fn recv_from(&mut self) -> Result<(EgmRobot, SocketAddr), ReceiveError> {
		let mut buffer = vec![0u8; 256];
		let (bytes_received, sender) = self.inner.recv_from(&mut buffer).await?;
		Ok((EgmRobot::decode(&buffer[..bytes_received])?, sender))
	}
}

impl EgmSender {
	/// Create an EGM sender from the send half of a UDP socket.
	pub fn new(inner: udp::SendHalf) -> Self {
		Self { inner }
	}

	/// Consume self and return the original half of the UDP socket.
	pub fn into_inner(self) -> udp::SendHalf {
		self.inner
	}

	/// Send a message to the remote address to which the unerlying socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn send(&mut self, msg: &EgmSensor) -> Result<(), SendError> {
		let buffer = encode_to_vec(msg)?;
		let bytes_sent = self.inner.send(&buffer).await?;
		check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}

	/// Send a message to the specified address.
	pub async fn send_to(&mut self, msg: &EgmSensor, target: &SocketAddr) -> Result<(), SendError> {
		let buffer = encode_to_vec(msg)?;
		let bytes_sent = self.inner.send_to(&buffer, target).await?;
		check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}
}

/// Encode a protocol buffers message to a byte vector.
fn encode_to_vec(msg: &impl Message) -> Result<Vec<u8>, prost::EncodeError> {
	let encoded_len = msg.encoded_len();
	let mut buffer = Vec::<u8>::with_capacity(encoded_len);
	msg.encode(&mut buffer)?;
	Ok(buffer)
}

/// Check if a whole buffer was sucessfully transferred.
fn check_transfer(transferred: usize, total: usize) -> Result<(), IncompleteTransmissionError> {
	if transferred == total {
		Ok(())
	} else {
		Err(IncompleteTransmissionError {
			transferred,
			total
		})
	}
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
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "incomplete transmission: transferred only {} of {} bytes", self.transferred, self.total)
	}
}

impl std::error::Error for ReceiveError {}
impl std::error::Error for SendError {}
impl std::error::Error for IncompleteTransmissionError {}

#[cfg(test)]
mod test {
	use super::*;
	use assert2::assert;

	#[test]
	fn test_check_transfer() {
		assert!(let Err(IncompleteTransmissionError { transferred: 1, total: 2}) = check_transfer(1, 2));
		assert!(let Err(IncompleteTransmissionError { transferred: 2, total: 1}) = check_transfer(2, 1));
		assert!(let Ok(()) = check_transfer(3, 3));
	}

	#[test]
	fn test_encode_to_vec() {
		assert!(encode_to_vec(&true).unwrap().len() == true.encoded_len());
		assert!(encode_to_vec(&10).unwrap().len() == 10.encoded_len());
		assert!(encode_to_vec(&String::from("aap noot mies")).unwrap().len() == String::from("aap noot mies").encoded_len());
	}
}
