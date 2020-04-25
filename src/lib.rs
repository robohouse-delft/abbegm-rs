//! This library implements the communication layer of ABB externally-guided motion.
//!
//! Externally-guided motion (or EGM) is an interface for ABB robots to allow smooth control of a robotic arm from an external PC.
//! In order to use it, the *Externally Guided Motion* option (689-1) must be installed on the robot controller.
//!
//! EGM can be used to stream positions to a robot controller in either joint space or cartesian space.
//! It can also be used to apply corrections to a pre-programmed trajectory.
//!
//! To communicate with a robot controller in blocking mode, use [`sync_peer::EgmPeer`].
//! Use [`tokio_peer::EgmPeer`] if you want to communicate with a robot controller asynchronously.
//!
//! # Warning
//! Industrial robots are dangerous machines.
//! Sending poses to the robot using EGM may cause it to perform dangerous motions that could lead to damage, injuries or even death.
//!
//! Always take appropriate precautions.
//! Make sure there are no persons or animals in reach of the robot when it is operational and always keep an emergency stop at hand when testing.
//!
//! # Units and rotation conventions
//! Unless specified differently, all distances and positions are in millimeters and all angles are in degrees.
//! This may be somewhat odd, but it is what the robot controller expects.
//!
//! You should use unit quaternions to represent 3D orientations, not Euler angles or roll-pitch-yaw.
//! Using unit quaternions avoids the need to specify which Euler angles or roll-pitch-yaw representation is used.
//! Quaternions come with the added advantage that you do not have to use degrees.
//!
//! # Features
//! Some optional features are available.
//! Note that all features are enabled by default.
//! To avoid unnecessary dependencies you can disable the default features and select only the ones you need:
//!
//! ```toml
//! [dependencies]
//! abbegm = { version = "...", default-features = false, features = ["nalgebra"] }
//! ```
//!
//! The available features are:
//!   * `tokio`: enable the asynchronous peer.
//!   * `nalgebra`: implement conversions between `nalgebra` types and EGM messages.

use std::time::Duration;

mod error;
pub use error::IncompleteTransmissionError;
pub use error::ReceiveError;
pub use error::SendError;

mod generated;

/// Generated protobuf messages used by EGM.
pub mod msg {
	pub use super::generated::*;
}

/// Synchronous (blocking) EGM peer.
pub mod sync_peer;

/// Asynchronous EGM peer using `tokio`.
#[cfg(feature = "tokio")]
pub mod tokio_peer;

/// Conversions to/from nalgebra types.
#[cfg(feature = "nalgebra")]
mod nalgebra;

impl msg::EgmHeader {
	pub fn new(seqno: u32, timestamp_ms: u32, kind: msg::egm_header::MessageType) -> Self {
		Self {
			seqno: Some(seqno),
			tm: Some(timestamp_ms),
			mtype: Some(kind as i32),
		}
	}

	/// Make a new command header.
	pub fn command(seqno: u32, timestamp_ms: u32) -> Self {
		Self::new(seqno, timestamp_ms, msg::egm_header::MessageType::MsgtypeCommand)
	}

	/// Make a new data header.
	pub fn data(seqno: u32, timestamp_ms: u32) -> Self {
		Self::new(seqno, timestamp_ms, msg::egm_header::MessageType::MsgtypeData)
	}

	/// Make a new correction header.
	pub fn correction(seqno: u32, timestamp_ms: u32) -> Self {
		Self::new(seqno, timestamp_ms, msg::egm_header::MessageType::MsgtypeCorrection)
	}

	/// Make a new path correction header.
	pub fn path_correction(seqno: u32, timestamp_ms: u32) -> Self {
		Self::new(seqno, timestamp_ms, msg::egm_header::MessageType::MsgtypePathCorrection)
	}
}

impl msg::EgmCartesian {
	/// Create a cartesian position from x, y and z components in millemeters.
	pub fn from_mm(x: f64, y: f64, z: f64) -> Self {
		Self { x, y, z }
	}

	/// Get the cartesion position as [x, y, z] array in millimeters.
	pub fn as_mm(&self) -> [f64; 3] {
		[self.x, self.y, self.z]
	}
}

impl From<[f64; 3]> for msg::EgmCartesian {
	/// Create a cartesian position from x, y and z components in millemeters.
	fn from(other: [f64; 3]) -> Self {
		let [x, y, z] = other;
		Self::from_mm(x, y, z)
	}
}

impl From<(f64, f64, f64)> for msg::EgmCartesian {
	/// Create a cartesian position from x, y and z components in millemeters.
	fn from(other: (f64, f64, f64)) -> Self {
		let (x, y, z) = other;
		Self::from_mm(x, y, z)
	}
}

impl msg::EgmQuaternion {
	/// Create a new quaternion from w, x, y and z components.
	pub fn from_wxyz(w: f64, x: f64, y: f64, z: f64) -> Self {
		Self {
			u0: w,
			u1: x,
			u2: y,
			u3: z,
		}
	}

	/// Get the quaternion as [w, x, y, z] array.
	pub fn as_wxyz(&self) -> [f64; 4] {
		[self.u0, self.u1, self.u2, self.u3]
	}
}

impl msg::EgmEuler {
	/// Create a new rotation from X, Y and Z rotations specified in degrees.
	pub fn from_xyz_degrees(x: f64, y: f64, z: f64) -> Self {
		Self { x, y, z }
	}

	/// Get the rotation as [x, y, z] array in degrees.
	pub fn as_xyz_degrees(&self) -> [f64; 3] {
		[self.x, self.y, self.z]
	}
}

impl msg::EgmClock {
	/// Create a new time point from seconds and microseconds.
	pub fn new(sec: u64, usec: u64) -> Self {
		Self { sec, usec }
	}

	/// Get the elapsed time since the epoch as [`Duration`].
	///
	/// Note that the duration will have only a microsecond resolution.
	pub fn elapsed_since_epoch(&self) -> Duration {
		Duration::new(self.sec, (self.usec) as u32 * 1000)
	}
}

impl Copy for msg::EgmClock {}

impl std::ops::Add<&Duration> for &msg::EgmClock {
	type Output = msg::EgmClock;

	fn add(self, right: &Duration) -> Self::Output {
		let usec = self.usec + u64::from(right.subsec_micros());
		msg::EgmClock {
			sec: self.sec + right.as_secs() + usec / 1_000_000,
			usec: usec % 1_000_000,
		}
	}
}

impl std::ops::Add<&msg::EgmClock> for &Duration {
	type Output = msg::EgmClock;

	fn add(self, right: &msg::EgmClock) -> Self::Output {
		right + self
	}
}

impl std::ops::Add<Duration> for msg::EgmClock {
	type Output = Self;

	fn add(self, right: Duration) -> Self::Output {
		&self + &right
	}
}

impl std::ops::Add<msg::EgmClock> for Duration {
	type Output = msg::EgmClock;

	fn add(self, right: msg::EgmClock) -> Self::Output {
		&self + &right
	}
}

impl std::ops::AddAssign<&Duration> for msg::EgmClock {
	fn add_assign(&mut self, right: &Duration) {
		*self = &*self + right
	}
}

impl std::ops::AddAssign<Duration> for msg::EgmClock {
	fn add_assign(&mut self, right: Duration) {
		*self += &right
	}
}

#[cfg(test)]
#[test]
fn test_add_duration() {
	use assert2::assert;
	use msg::EgmClock;
	assert!(EgmClock::new(1, 500_000) + Duration::from_secs(1) == EgmClock::new(2, 500_000));
	assert!(EgmClock::new(1, 500_000) + Duration::from_millis(600) == EgmClock::new(2, 100_000));
	assert!(&EgmClock::new(1, 500_000) + &Duration::from_secs(1) == EgmClock::new(2, 500_000));
	assert!(&EgmClock::new(1, 500_000) + &Duration::from_millis(600) == EgmClock::new(2, 100_000));
	assert!(Duration::from_secs(1) + EgmClock::new(1, 500_000)  == EgmClock::new(2, 500_000));
	assert!(Duration::from_millis(600) + EgmClock::new(1, 500_000)  == EgmClock::new(2, 100_000));
	assert!(&Duration::from_secs(1) + &EgmClock::new(1, 500_000)  == EgmClock::new(2, 500_000));
	assert!(&Duration::from_millis(600) + &EgmClock::new(1, 500_000)  == EgmClock::new(2, 100_000));

	let mut clock = EgmClock::new(10, 999_999);
	clock += Duration::from_micros(1);
	assert!(clock == EgmClock::new(11, 0));
	clock += Duration::from_micros(999_999);
	assert!(clock == EgmClock::new(11, 999_999));
	clock += Duration::from_micros(2);
	assert!(clock == EgmClock::new(12, 1));
}

impl msg::EgmPose {
	/// Create a new 6-DOF pose from a position and orientation.
	pub fn new(position: impl Into<msg::EgmCartesian>, orientation: impl Into<msg::EgmQuaternion>) -> Self {
		Self {
			pos: Some(position.into()),
			orient: Some(orientation.into()),
			euler: None,
		}
	}
}

impl msg::EgmJoints {
	/// Create a new joint list from a vector of joint values in degrees.
	pub fn from_degrees(joints: impl Into<Vec<f64>>) -> Self {
		Self { joints: joints.into() }
	}
}

impl From<Vec<f64>> for msg::EgmJoints {
	/// Create a new joint list from a vector of joint values in degrees.
	fn from(other: Vec<f64>) -> Self {
		Self::from_degrees(other)
	}
}

impl From<&[f64]> for msg::EgmJoints {
	/// Create a new joint list from a slice of joint values in degrees.
	fn from(other: &[f64]) -> Self {
		Self::from_degrees(other)
	}
}

impl From<&[f64; 6]> for msg::EgmJoints {
	/// Create a new joint list from an array of joint values in degrees.
	fn from(other: &[f64; 6]) -> Self {
		Self::from_degrees(*other)
	}
}

impl msg::EgmExternalJoints {
	/// Create a new external joint list from a vector of joint values in degrees.
	pub fn from_degrees(joints: impl Into<Vec<f64>>) -> Self {
		Self { joints: joints.into() }
	}
}

impl From<Vec<f64>> for msg::EgmExternalJoints {
	/// Create a new external joint list from a vector of joint values in degrees.
	fn from(other: Vec<f64>) -> Self {
		Self::from_degrees(other)
	}
}

impl From<&[f64]> for msg::EgmExternalJoints {
	/// Create a new external joint list from a slice of joint values in degrees.
	fn from(other: &[f64]) -> Self {
		Self::from_degrees(other)
	}
}

impl msg::EgmPlanned {
	/// Create a new joint target.
	pub fn joints(joints: impl Into<msg::EgmJoints>, time: impl Into<msg::EgmClock>) -> Self {
		Self {
			time: Some(time.into()),
			joints: Some(joints.into()),
			cartesian: None,
			external_joints: None,
		}
	}

	/// Create a new 6-DOF pose target.
	pub fn pose(pose: impl Into<msg::EgmPose>, time: impl Into<msg::EgmClock>) -> Self {
		Self {
			time: Some(time.into()),
			cartesian: Some(pose.into()),
			joints: None,
			external_joints: None,
		}
	}
}

impl msg::EgmPathCorr {
	/// Create a new path correction.
	pub fn new(position: impl Into<msg::EgmCartesian>, age_ms: u32) -> Self {
		Self {
			pos: position.into(),
			age: age_ms,
		}
	}
}

impl msg::EgmSensor {
	/// Create a sensor message containing a joint target.
	///
	/// The header timestamp is created from the `time` parameter.
	pub fn joint_target(sequence_number: u32, joints: impl Into<msg::EgmJoints>, time: impl Into<msg::EgmClock>) -> Self {
		let time = time.into();
		let timestamp_ms = (time.sec.wrapping_mul(1000) + time.usec / 1000) as u32;
		Self {
			header: Some(msg::EgmHeader::correction(sequence_number, timestamp_ms)),
			planned: Some(msg::EgmPlanned::joints(joints, time)),
			speed_ref: None,
		}
	}

	/// Create a sensor message containing a 6-DOF pose target.
	///
	/// The header timestamp is created from the `time` parameter.
	pub fn pose_target(sequence_number: u32, pose: impl Into<msg::EgmPose>, time: impl Into<msg::EgmClock>) -> Self {
		let time = time.into();
		let timestamp_ms = (time.sec.wrapping_mul(1000) + time.usec / 1000) as u32;
		Self {
			header: Some(msg::EgmHeader::correction(sequence_number, timestamp_ms)),
			planned: Some(msg::EgmPlanned::pose(pose, time)),
			speed_ref: None,
		}
	}
}

impl msg::EgmSensorPathCorr {
	/// Create a sensor message containing a path correction.
	pub fn new(sequence_number: u32, timestamp_ms: u32, correction: impl Into<msg::EgmCartesian>, age_ms: u32) -> Self {
		Self {
			header: Some(msg::EgmHeader::path_correction(sequence_number, timestamp_ms)),
			path_corr: Some(msg::EgmPathCorr::new(correction, age_ms)),
		}
	}
}

impl msg::EgmRobot {
	pub fn sequence_number(&self) -> Option<u32> {
		self.header.as_ref()?.seqno
	}

	pub fn timestamp_ms(&self) -> Option<u32> {
		self.header.as_ref()?.tm
	}

	pub fn feedback_joints(&self) -> Option<&Vec<f64>> {
		Some(&self.feed_back.as_ref()?.joints.as_ref()?.joints)
	}

	pub fn feedback_pose(&self) -> Option<&msg::EgmPose> {
		self.feed_back.as_ref()?.cartesian.as_ref()
	}

	pub fn feedback_extenal_joints(&self) -> Option<&Vec<f64>> {
		Some(&self.feed_back.as_ref()?.external_joints.as_ref()?.joints)
	}

	pub fn feedback_time(&self) -> Option<msg::EgmClock> {
		self.feed_back.as_ref()?.time.clone()
	}

	pub fn planned_joints(&self) -> Option<&Vec<f64>> {
		Some(&self.planned.as_ref()?.joints.as_ref()?.joints)
	}

	pub fn planned_pose(&self) -> Option<&msg::EgmPose> {
		self.planned.as_ref()?.cartesian.as_ref()
	}

	pub fn planned_extenal_joints(&self) -> Option<&Vec<f64>> {
		Some(&self.planned.as_ref()?.external_joints.as_ref()?.joints)
	}

	pub fn planned_time(&self) -> Option<msg::EgmClock> {
		self.planned.as_ref()?.time.clone()
	}

	pub fn is_motors_on(&self) -> Option<bool> {
		use msg::egm_motor_state::MotorStateType;
		match self.motor_state.as_ref()?.state() {
			MotorStateType::MotorsUndefined => None,
			MotorStateType::MotorsOn => Some(true),
			MotorStateType::MotorsOff => Some(false),
		}
	}

	pub fn is_rapid_running(&self) -> Option<bool> {
		use msg::egm_rapid_ctrl_exec_state::RapidCtrlExecStateType;
		match self.rapid_exec_state.as_ref()?.state() {
			RapidCtrlExecStateType::RapidUndefined => None,
			RapidCtrlExecStateType::RapidRunning => Some(true),
			RapidCtrlExecStateType::RapidStopped => Some(false),
		}
	}

	pub fn test_signals(&self) -> Option<&Vec<f64>> {
		Some(&self.test_signals.as_ref()?.signals)
	}

	pub fn measured_force(&self) -> Option<&Vec<f64>> {
		Some(&self.measured_force.as_ref()?.force)
	}
}

/// Encode a protocol buffers message to a byte vector.
fn encode_to_vec(msg: &impl prost::Message) -> Result<Vec<u8>, prost::EncodeError> {
	let encoded_len = msg.encoded_len();
	let mut buffer = Vec::<u8>::with_capacity(encoded_len);
	msg.encode(&mut buffer)?;
	Ok(buffer)
}

#[cfg(test)]
#[test]
fn test_encode_to_vec() {
	use assert2::assert;
	use prost::Message;

	assert!(encode_to_vec(&true).unwrap().len() == true.encoded_len());
	assert!(encode_to_vec(&10).unwrap().len() == 10.encoded_len());
	assert!(encode_to_vec(&String::from("aap noot mies")).unwrap().len() == String::from("aap noot mies").encoded_len());
}
