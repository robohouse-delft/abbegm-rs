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
