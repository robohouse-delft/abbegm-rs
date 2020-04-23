use crate::msg;
use std::convert::TryFrom;

/// Implement `From<$from> for $for` by delegating to `From<&$from>`.
macro_rules! impl_through_ref {
	(From<$from:ty> for $for:ty) => {
		impl From<$from> for $for {
			fn from(other: $from) -> Self {
				(&other).into()
			}
		}
	};

	(TryFrom<$from:ty> for $for:ty) => {
		impl ::core::convert::TryFrom<$from> for $for {
			type Error = <Self as ::core::convert::TryFrom<&'static $from>>::Error;

			fn try_from(other: $from) -> Result<Self, Self::Error> {
				Self::try_from(&other)
			}
		}
	};
}

/// Implement `$trait<$a> for $b` and `$trait<$b> for $a` by delegating to `$trait<&_>
macro_rules! impl_bidi_through_ref {
	($trait:ident, $a:ty, $b:ty) => {
		impl_through_ref!($trait<$a> for $b);
		impl_through_ref!($trait<$b> for $a);
	}
}

// Vector3

impl From<&msg::EgmCartesian> for nalgebra::Vector3<f64> {
	fn from(other: &msg::EgmCartesian) -> Self {
		Self::new(other.x, other.y, other.z)
	}
}

impl From<&nalgebra::Vector3<f64>> for msg::EgmCartesian {
	fn from(other: &nalgebra::Vector3<f64>) -> Self {
		Self {
			x: other.x,
			y: other.y,
			z: other.z,
		}
	}
}

impl_bidi_through_ref!(From, msg::EgmCartesian, nalgebra::Vector3<f64>);

// Quaternion

impl From<&msg::EgmQuaternion> for nalgebra::Quaternion<f64> {
	fn from(other: &msg::EgmQuaternion) -> Self {
		Self::new(other.u0, other.u1, other.u2, other.u3)
	}
}

impl From<&nalgebra::Quaternion<f64>> for msg::EgmQuaternion {
	fn from(other: &nalgebra::Quaternion<f64>) -> Self {
		Self {
			u0: other.scalar(),
			u1: other.imag().x,
			u2: other.imag().y,
			u3: other.imag().z,
		}
	}
}

impl_bidi_through_ref!(From, msg::EgmQuaternion, nalgebra::Quaternion<f64>);

// UnitQuaternion

impl From<&msg::EgmQuaternion> for nalgebra::UnitQuaternion<f64> {
	fn from(other: &msg::EgmQuaternion) -> Self {
		Self::from_quaternion(other.into())
	}
}

impl From<&nalgebra::UnitQuaternion<f64>> for msg::EgmQuaternion {
	fn from(other: &nalgebra::UnitQuaternion<f64>) -> Self {
		other.as_ref().into()
	}
}

impl_bidi_through_ref!(From, msg::EgmQuaternion, nalgebra::UnitQuaternion<f64>);

// Rotation3

impl From<&msg::EgmQuaternion> for nalgebra::Rotation3<f64> {
	fn from(other: &msg::EgmQuaternion) -> Self {
		nalgebra::UnitQuaternion::from(other).into()
	}
}

impl From<&nalgebra::Rotation3<f64>> for msg::EgmQuaternion {
	fn from(other: &nalgebra::Rotation3<f64>) -> Self {
		nalgebra::UnitQuaternion::from_rotation_matrix(other).into()
	}
}

impl_bidi_through_ref!(From, msg::EgmQuaternion, nalgebra::Rotation3<f64>);

// Isometry3

impl TryFrom<&msg::EgmPose> for nalgebra::Isometry3<f64> {
	type Error = TryFromEgmPoseError;

	fn try_from(other: &msg::EgmPose) -> Result<Self, Self::Error> {
		let position    = other.pos.as_ref().ok_or(Self::Error::MissingPosition)?;
		let orientation = other.orient.as_ref().ok_or(Self::Error::MissingOrientation)?;

		Ok(nalgebra::Isometry3::from_parts(
			nalgebra::Vector3::from(position).into(),
			orientation.into(),
		))
	}
}

impl From<&nalgebra::Isometry3<f64>> for msg::EgmPose {
	fn from(other: &nalgebra::Isometry3<f64>) -> Self {
		Self {
			pos: Some(other.translation.vector.into()),
			orient: Some(other.rotation.into()),
			euler: None,
		}
	}
}

impl_through_ref!(From<nalgebra::Isometry3<f64>> for msg::EgmPose);
impl_through_ref!(TryFrom<msg::EgmPose> for nalgebra::Isometry3<f64>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TryFromEgmPoseError {
	MissingPosition,
	MissingOrientation,
}

impl std::fmt::Display for TryFromEgmPoseError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::MissingPosition => write!(f, "missing field: pos"),
			Self::MissingOrientation => write!(f, "missing field: orient"),
		}
	}
}

impl std::error::Error for TryFromEgmPoseError {}
