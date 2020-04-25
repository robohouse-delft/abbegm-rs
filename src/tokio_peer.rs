use std::net::SocketAddr;

use prost::Message;
use tokio::net::UdpSocket;
use tokio::net::udp;

use crate::ReceiveError;
use crate::SendError;
use crate::msg::EgmRobot;
use crate::msg::EgmSensor;

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

	/// Create an EGM peer on a newly bound UDP socket.
	///
	/// The socket will not be connected to a remote peer,
	/// so you can only use [`EgmPeer::recv_from`] and [`EgmPeer::send_to`].
	pub async fn bind(addrs: impl tokio::net::ToSocketAddrs) -> std::io::Result<Self> {
		Ok(Self::new(UdpSocket::bind(addrs).await?))
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
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.inner.send(&buffer).await?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}

	/// Send a message to the specified address.
	pub async fn send_to(&mut self, msg: &EgmSensor, target: &SocketAddr) -> Result<(), SendError> {
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.inner.send_to(&buffer, target).await?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}
}
