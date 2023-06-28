// textos::unicode::egc::string
//
//!
//

use core::fmt;
use crate::unicode::{char::*};
use unicode_segmentation::UnicodeSegmentation;

use alloc::str::Chars;

/// An extended grapheme cluster backed by a [`String`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub struct Egc(String);

impl Egc {
    /// Creates a new empty `StaticNonNulString`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the length in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the current length is 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn chars(&self) -> Chars {
        self.0.chars()
    }
}

/* traits */

impl Default for Egc {
    /// Returns an empty string.
    #[inline]
    fn default() -> Self {
        Self(String::default())
    }
}

impl fmt::Display for Egc {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Egc {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl From<String> for Egc {
    #[inline]
    fn from(s: String) -> Egc {
        Egc(s.graphemes(true).take(1).collect())
    }
}
impl From<&str> for Egc {
    #[inline]
    fn from(s: &str) -> Egc {
        Egc(s.graphemes(true).take(1).collect())
    }
}
impl From<char> for Egc {
    #[inline]
    fn from(s: char) -> Egc {
        Egc(s.into())
    }
}
