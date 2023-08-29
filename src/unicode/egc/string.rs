// textos::unicode::egc::string
//
//!
//

use crate::unicode::char::*;
use core::fmt;
use unicode_segmentation::UnicodeSegmentation;

use alloc::{
    str::{self, Chars},
    string::String,
};

/// An extended grapheme cluster backed by a [`String`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub struct Egc(String);

impl Egc {
    /// Creates a new empty extended grapheme cluster.
    #[inline]
    pub const fn new() -> Egc {
        Self(String::new())
    }

    /// Creates a new `Egc` from a `Char7`.
    #[inline]
    pub fn from_char7(c: Char7) -> Egc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `Egc` from a `Char8`.
    #[inline]
    pub fn from_char8(c: Char8) -> Egc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `Egc` from a `Char16`.
    #[inline]
    pub fn from_char16(c: Char16) -> Egc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `Egc` from a `Char24`.
    #[inline]
    pub fn from_char24(c: Char24) -> Egc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `Egc` from a `Char32`.
    #[inline]
    pub fn from_char32(c: Char32) -> Egc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `Egc` from a `char`.
    #[inline]
    pub fn from_char(c: char) -> Egc {
        Self::from_char32(Char32(c))
    }

    //

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
    /// Returns a new empty extended grapheme cluster.
    #[inline]
    fn default() -> Self {
        Self::new()
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
