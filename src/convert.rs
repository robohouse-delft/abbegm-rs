/// Implement `From<$from> for $for` by delegating to `From<&$from>`.
#[macro_export]
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
#[macro_export]
macro_rules! impl_bidi_through_ref {
	($trait:ident, $a:ty, $b:ty) => {
		impl_through_ref!($trait<$a> for $b);
		impl_through_ref!($trait<$b> for $a);
	}
}

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
