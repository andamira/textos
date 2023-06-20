// textas::error
//
//! Error types.
//

use core::{result, str::Utf8Error};
// use sixbit::EncodeError;

/// `textos` result type.
pub type TextosResult<N> = result::Result<N, TextosError>;

/// `textos` error type.
#[non_exhaustive]
#[derive(Debug)]
pub enum TextosError {
    // /// An error involving the encoding of a rate's name.
    // RateName(EncodeError),

    // https://docs.rs/staticvec/latest/staticvec/enum.StringError.html
    // https://docs.rs/staticvec/latest/staticvec/struct.StaticString.html
    /// The value is out of bounds.
    OutOfBounds,

    /// Not enough capacity for the attempted operation.
    ///
    /// Returns the needed capacity.
    NotEnoughCapacity(usize),

    /// Not enough elements for the attempted operation.
    ///
    /// Returns the needed number of elements.
    NotEnoughElements(usize),

    // ///
    // PushCapacity(String),

    // ///
    // NotCharBoundary,
    /// Errors which can occur when attempting to interpret a sequence of [`u8`]
    /// as a string.
    Utf8(Utf8Error),

    /// A miscelaneous error.
    #[cfg(feature = "std")]
    Error(String),
}

impl TextosError {
    /// Returns `true` if the error is about bytes to UTF-8 conversion.
    pub const fn is_utf8(&self) -> bool {
        matches![self, TextosError::Utf8(_)]
    }

    // /// Returns `true` if the error is about bytes to UTF-8 conversion.
    // pub const fn is_not_char_boundary(&self) -> bool {
    //     matches![self, TextosError::Utf8(_)]
    // }
}

/// allows converting into `Error` from other error types.
mod core_impls {
    use super::TextosError;
    use core::{fmt, str::Utf8Error};

    impl From<Utf8Error> for TextosError {
        fn from(e: Utf8Error) -> Self {
            TextosError::Utf8(e)
        }
    }

    impl fmt::Display for TextosError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use TextosError::*;
            match self {
                OutOfBounds => write!(f, "The value is out of bounds."),
                NotEnoughCapacity(c) => write!(f, "Not enough capacity. Needed: {c}"),
                NotEnoughElements(e) => write!(f, "Not enough elements. Needed: {e}"),
                Utf8(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "std")]
                Error(s) => write!(f, "Error: {s}"),
            }
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::TextosError;
    use std::error::Error as StdError;

    impl StdError for TextosError {}
}
