// textos::unicode::egc::non_nul
//
//!
//

use crate::{
    // error::TextosResult as Result,
    unicode::{
        egc::Egcs,
        string::{StaticNonNulString, Strings},
    },
};
#[cfg(feature = "alloc")]
use alloc::{ffi::CString, str::Chars};
use core::fmt;
// use unicode_segmentation::UnicodeSegmentation;

/// An extended grapheme cluster backed by a [`StaticNonNulString`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StaticNonNulEgc<const CAP: usize>(StaticNonNulString<CAP>);

impl<const CAP: usize> StaticNonNulEgc<CAP> {
    /// Creates a new empty `StaticNonNulString`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the current length is 0.
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Returns the total capacity in bytes.
    #[inline]
    pub const fn capacity() -> usize {
        CAP
    }

    /// Returns the remaining capacity.
    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == CAP
    }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    //

    /// Returns a byte slice of the inner string slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns a mutable byte slice of the inner string slice.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub fn as_array(&self) -> [u8; CAP] {
        self.0.as_array()
    }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub fn into_array(self) -> [u8; CAP] {
        self.0.into_array()
    }

    /// Returns the inner string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the mutable inner string slice.
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        self.0.as_str_mut()
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub fn chars(&self) -> Chars {
        self.0.chars()
    }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString {
        self.0.to_cstring()
    }

    //
}

/* traits */

impl<const CAP: usize> Strings for StaticNonNulEgc<CAP> {}
impl<const CAP: usize> Egcs for StaticNonNulEgc<CAP> {}

mod core_impls {
    use super::*;

    impl<const CAP: usize> Default for StaticNonNulEgc<CAP> {
        /// Returns an empty string.
        #[inline]
        fn default() -> Self {
            Self(StaticNonNulString::default())
        }
    }

    impl<const CAP: usize> fmt::Display for StaticNonNulEgc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<const CAP: usize> fmt::Debug for StaticNonNulEgc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    // impl From<String> for StaticNonNulEgc {
    //     fn from(s: String) -> StaticNonNulEgc {
    //         StaticNonNulEgc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<&str> for StaticNonNulEgc {
    //     fn from(s: &str) -> StaticNonNulEgc {
    //         StaticNonNulEgc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<char> for StaticNonNulEgc {
    //     fn from(s: char) -> StaticNonNulEgc {
    //         StaticNonNulEgc(s.into())
    //     }
    // }
}
