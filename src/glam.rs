use crate::msg;

use std::convert::TryFrom;

// Vector3

impl From<&msg::EgmCartesian> for glam::DVec3 {
	fn from(other: &msg::EgmCartesian) -> Self {
		Self::new(other.x, other.y, other.z)
	}
}

impl From<&glam::DVec3> for msg::EgmCartesian {
	fn from(other: &glam::DVec3) -> Self {
		Self::from_mm(other.x, other.y, other.z)
	}
}

impl TryFrom<&msg::EgmCartesianSpeed> for glam::DVec3 {
	type Error = TryFromEgmCartesianSpeedError;

	fn try_from(other: &msg::EgmCartesianSpeed) -> Result<Self, Self::Error> {
		if other.value.len() == 3 {
			Ok(Self::new(other.value[0], other.value[1], other.value[2]))
		} else {
			Err(TryFromEgmCartesianSpeedError::WrongNumberOfValues(other.value.len()))
		}
	}
}

impl From<&glam::DVec3> for msg::EgmCartesianSpeed {
	fn from(other: &glam::DVec3) -> Self {
		Self::from_xyz_mm(other.x, other.y, other.z)
	}
}

impl_bidi_through_ref!(From, msg::EgmCartesian, glam::DVec3);
impl_through_ref!(From<glam::DVec3> for msg::EgmCartesianSpeed);
impl_through_ref!(TryFrom<msg::EgmCartesianSpeed> for glam::DVec3);

// Quaternion

impl From<&msg::EgmQuaternion> for glam::DQuat{
	fn from(other: &msg::EgmQuaternion) -> Self {
		Self::from_xyzw(other.u0, other.u1, other.u2, other.u3)
	}
}

impl From<&glam::DQuat> for msg::EgmQuaternion {
	fn from(other: &glam::DQuat) -> Self {
		Self::from_wxyz(other.w, other.x, other.y, other.z)
	}
}

impl_bidi_through_ref!(From, msg::EgmQuaternion, glam::DQuat);

// UnitQuaternion

// impl From<&msg::EgmQuaternion> for glam::UnitQuaternion {
// 	fn from(other: &msg::EgmQuaternion) -> Self {
// 		Self::from_quaternion(other.into())
// 	}
// }

// impl From<&glam::UnitQuaternion> for msg::EgmQuaternion {
// 	fn from(other: &glam::UnitQuaternion) -> Self {
// 		other.as_ref().into()
// 	}
// }

// impl_bidi_through_ref!(From, msg::EgmQuaternion, glam::UnitQuaternion);

// Rotation3

impl From<&msg::EgmQuaternion> for glam::DAffine3 {
	fn from(other: &msg::EgmQuaternion) -> Self {
        glam::DAffine3::from_quat(glam::DQuat::from(other))
	}
}

impl From<&glam::DAffine3> for msg::EgmQuaternion {
	fn from(other: &glam::DAffine3) -> Self {
		glam::DQuat::from_affine3(other).into()
	}
}

impl_bidi_through_ref!(From, msg::EgmQuaternion, glam::DAffine3);

// Isometry3

impl TryFrom<&msg::EgmPose> for glam::DAffine3 {
	type Error = TryFromEgmPoseError;

	fn try_from(other: &msg::EgmPose) -> Result<Self, Self::Error> {
		let position = other.pos.as_ref().ok_or(Self::Error::MissingPosition)?;
		let orientation = other.orient.as_ref().ok_or(Self::Error::MissingOrientation)?;

		Ok(glam::DAffine3::from_rotation_translation(
            orientation.into(),
			glam::DVec3::from(position).into(),
		))
	}
}

impl From<&glam::DAffine3> for msg::EgmPose {
	fn from(other: &glam::DAffine3) -> Self {
        let (_, rotation, translation) = other.to_scale_rotation_translation();
		Self::new(translation, rotation)
	}
}

impl_through_ref!(From<glam::DAffine3> for msg::EgmPose);
impl_through_ref!(TryFrom<msg::EgmPose> for glam::DAffine3);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TryFromEgmCartesianSpeedError {
	WrongNumberOfValues(usize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TryFromEgmPoseError {
	MissingPosition,
	MissingOrientation,
}

impl std::fmt::Display for TryFromEgmCartesianSpeedError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::WrongNumberOfValues(x) => write!(f, "wrong number of values, expected 3, got {}", x),
		}
	}
}

impl std::fmt::Display for TryFromEgmPoseError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::MissingPosition => write!(f, "missing field: pos"),
			Self::MissingOrientation => write!(f, "missing field: orient"),
		}
	}
}

impl std::error::Error for TryFromEgmCartesianSpeedError {}
impl std::error::Error for TryFromEgmPoseError {}
