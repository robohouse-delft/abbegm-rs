use std::net::SocketAddr;

use prost::Message;
use tokio::net::udp;
use tokio::net::UdpSocket;

use crate::InvalidMessageError;
use crate::ReceiveError;
use crate::SendError;
use crate::msg::EgmRobot;
use crate::msg::EgmSensor;

#[derive(Debug)]
/// Asynchronous EGM peer capable of sending and receiving messages.
pub struct EgmPeer {
	socket: UdpSocket,
}

#[derive(Debug)]
/// Receiving half of an [`EgmPeer`].
pub struct EgmReceiver {
	inner: udp::RecvHalf,
}

#[derive(Debug)]
/// Sending half of an [`EgmPeer`].
pub struct EgmSender {
	inner: udp::SendHalf,
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
	pub async fn bind(addrs: impl tokio::net::ToSocketAddrs) -> std::io::Result<Self> {
		Ok(Self::new(UdpSocket::bind(addrs).await?))
	}

	/// Synchronously create an EGM peer on a newly bound UDP socket.
	///
	/// This function allows you to create the peer synchronously,
	/// but use an asynchronous API for communicating with the robot.
	/// This can be useful if you want to perform initialization of your application synchronously.
	///
	/// The socket will not be connected to a remote peer,
	/// so you can only use [`EgmPeer::recv_from`] and [`EgmPeer::send_to`].
	pub fn bind_sync(addrs: impl std::net::ToSocketAddrs) -> std::io::Result<Self> {
		let socket = std::net::UdpSocket::bind(addrs)?;
		let socket = tokio::net::UdpSocket::from_std(socket)?;
		Ok(Self::new(socket))
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

	/// Split the peer into a receiver and a sender.
	///
	/// This can be useful if you want to split receiving and sending into separate tasks.
	pub fn split(self) -> (EgmReceiver, EgmSender) {
		let (rx, tx) = self.socket.split();
		(EgmReceiver::new(rx), EgmSender::new(tx))
	}

	/// Receive a message from the remote address to which the inner socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn recv(&mut self) -> Result<EgmRobot, ReceiveError> {
		let mut buffer = vec![0u8; 1024];
		let bytes_received = self.socket.recv(&mut buffer).await?;
		Ok(EgmRobot::decode(&buffer[..bytes_received])?)
	}

	/// Receive a message from any remote address.
	pub async fn recv_from(&mut self) -> Result<(EgmRobot, SocketAddr), ReceiveError> {
		let mut buffer = vec![0u8; 1024];
		let (bytes_received, sender) = self.socket.recv_from(&mut buffer).await?;
		Ok((EgmRobot::decode(&buffer[..bytes_received])?, sender))
	}

	/// Purge all messages from the socket read queue.
	pub async fn purge_read_queue(&mut self) -> std::io::Result<()> {
		let mut buffer = vec![0; 1024];
		loop {
			match poll_once(self.socket.recv_from(&mut buffer)).await {
				std::task::Poll::Ready(Ok(_)) => (),
				std::task::Poll::Ready(Err(e)) => return Err(e),
				std::task::Poll::Pending => return Ok(()),
			}
		}
	}

	/// Send a message to the remote address to which the inner socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn send(&mut self, msg: &EgmSensor) -> Result<(), SendError> {
		InvalidMessageError::check_sensor_msg(msg)?;
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.socket.send(&buffer).await?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}

	/// Send a message to the specified address.
	pub async fn send_to(&mut self, msg: &EgmSensor, target: &SocketAddr) -> Result<(), SendError> {
		InvalidMessageError::check_sensor_msg(msg)?;
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.socket.send_to(&buffer, target).await?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
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

	/// Receive a message from the remote address to which the inner socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn recv(&mut self) -> Result<EgmRobot, ReceiveError> {
		let mut buffer = vec![0u8; 1024];
		let bytes_received = self.inner.recv(&mut buffer).await?;
		Ok(EgmRobot::decode(&buffer[..bytes_received])?)
	}

	/// Receive a message from any remote address.
	pub async fn recv_from(&mut self) -> Result<(EgmRobot, SocketAddr), ReceiveError> {
		let mut buffer = vec![0u8; 1024];
		let (bytes_received, sender) = self.inner.recv_from(&mut buffer).await?;
		Ok((EgmRobot::decode(&buffer[..bytes_received])?, sender))
	}

	/// Purge all messages from the socket read queue.
	pub async fn purge_read_queue(&mut self) -> std::io::Result<()> {
		let mut buffer = vec![0; 1024];
		loop {
			match poll_once(self.inner.recv_from(&mut buffer)).await {
				std::task::Poll::Ready(Ok(_)) => (),
				std::task::Poll::Ready(Err(e)) => return Err(e),
				std::task::Poll::Pending => return Ok(()),
			}
		}
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

	/// Send a message to the remote address to which the inner socket is connected.
	///
	/// To use this function, you must pass an already connected socket to [`EgmPeer::new`].
	/// If the peer was created with an unconnected socket, this function will panic.
	pub async fn send(&mut self, msg: &EgmSensor) -> Result<(), SendError> {
		InvalidMessageError::check_sensor_msg(msg)?;
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.inner.send(&buffer).await?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}

	/// Send a message to the specified address.
	pub async fn send_to(&mut self, msg: &EgmSensor, target: &SocketAddr) -> Result<(), SendError> {
		InvalidMessageError::check_sensor_msg(msg)?;
		let buffer = crate::encode_to_vec(msg)?;
		let bytes_sent = self.inner.send_to(&buffer, target).await?;
		crate::error::check_transfer(bytes_sent, buffer.len())?;
		Ok(())
	}
}

struct PollOnce<F> {
	future: F,
}

impl<F: std::future::Future> std::future::Future for PollOnce<F> {
	type Output = std::task::Poll<F::Output>;

	fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context) -> std::task::Poll<Self::Output> {
		let pin = unsafe { std::pin::Pin::new_unchecked(&mut self.get_unchecked_mut().future) };
		std::task::Poll::Ready(pin.poll(context))
	}
}

async fn poll_once<F: std::future::Future>(future: F) -> std::task::Poll<F::Output> {
	PollOnce { future }.await
}
