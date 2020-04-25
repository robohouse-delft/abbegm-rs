use std::net::SocketAddr;

use prost::Message;
use std::net::UdpSocket;

use crate::msg::EgmRobot;
use crate::msg::EgmSensor;
use crate::ReceiveError;
use crate::SendError;

#[derive(Debug)]
/// Blocking EGM peer for sending and receiving messages over UDP.
pub struct EgmPeer {
	socket: UdpSocket,
}

impl EgmPeer {
	/// Wrap an existing UDP socket in a peer.
	///
	/// If you want to use the [`EgmPeer::recv`] and [`EgmPeer::send`] functions,
	/// you should use an already connected socket.
	/// Otherwise, you can only use [`EgmPeer::recv_from`] and [`EgmPeer::send_to`].
	pub fn new(socket: UdpSocket) -> Self {
		Self { socket }
	}

	/// Create an EGM peer on a newly bound UDP socket.
	///
	/// The socket will not be connected to a remote peer,
	/// so you can only use [`EgmPeer::recv_from`] and [`EgmPeer::send_to`].
	pub fn bind(addrs: impl std::net::ToSocketAddrs) -> std::io::Result<Self> {
		Ok(Self::new(UdpSocket::bind(addrs)?))
	}

	/// Get a shared reference to the inner socket.
	pub fn socket(&self) -> &UdpSocket {
		&self.socket
	}

	/// Get an exclusive reference to the inner socket.
	pub fn socket_mut(&mut self) -> &mut UdpSocket {
		&mut self.socket
	}

	/// Consume self and get the inner socket.
	pub fn into_socket(self) -> UdpSocket {
		self.socket
	}

	/// Receive a message from the remote address to which the inner socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub fn recv(&mut self) -> Result<EgmRobot, ReceiveError> {
		let mut buffer = vec![0u8; 1024];
		let bytes_received = self.socket.recv(&mut buffer)?;
		Ok(EgmRobot::decode(&buffer[..bytes_received])?)
	}

	/// Receive a message from any remote address.
	pub fn recv_from(&mut self) -> Result<(EgmRobot, SocketAddr), ReceiveError> {
		let mut buffer = vec![0u8; 1024];
		let (bytes_received, sender) = self.socket.recv_from(&mut buffer)?;
		Ok((EgmRobot::decode(&buffer[..bytes_received])?, sender))
	}

	/// Send a message to the remote address to which the inner socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub fn send(&mut self, msg: &EgmSensor) -> Result<(), SendError> {
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.socket.send(&buffer)?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}

	/// Send a message to the specified address.
	pub fn send_to(&mut self, msg: &EgmSensor, target: &SocketAddr) -> Result<(), SendError> {
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.socket.send_to(&buffer, target)?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}
}
