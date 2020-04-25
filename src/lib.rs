mod error;
pub use error::IncompleteTransmissionError;
pub use error::ReceiveError;
pub use error::SendError;

mod generated;

/// protobuf messages used by EGM
pub mod msg {
	pub use super::generated::*;
}

/// Asynchronous EGM peer using `tokio`.
#[cfg(feature = "tokio")]
pub mod tokio_peer;

#[cfg(feature = "nalgebra")]
pub mod nalgebra;

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

	pub fn feedback_cartesion(&self) -> Option<&msg::EgmPose> {
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

	pub fn planned_cartesion(&self) -> Option<&msg::EgmPose> {
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
