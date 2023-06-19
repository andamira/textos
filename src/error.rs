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
    //
    // ///
    // OutOfBounds,
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

    // MAYBE
    // impl From<EncodeError> for TextosError {
    //     fn from(err: EncodeError) -> Self {
    //         TextosError::RateName(err)
    //     }
    // }

    impl fmt::Display for TextosError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TextosError::NotEnoughCapacity(c) => write!(f, "Not enough capacity. Needed: {c}"),
                TextosError::NotEnoughElements(e) => write!(f, "Not enough elements. Needed: {e}"),
                TextosError::Utf8(e) => fmt::Debug::fmt(e, f),

                // TextosError::RateName(r) => Debug::fmt(r, f),
                #[cfg(feature = "std")]
                TextosError::Error(s) => write!(f, "Error: {s}"),
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
