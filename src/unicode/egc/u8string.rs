// textos::unicode::egc::u8string
//
//!
//
// TOC
// - definitions
// - trait impls
// - conversions

use crate::{
    error::{TextosError as Error, TextosResult as Result},
    macros::impl_sized_alias,
    unicode::{
        char::*,
        egc::Egcs,
        string::{StaticU8String, Strings},
    },
};
#[cfg(feature = "alloc")]
use alloc::{ffi::CString, str::Chars};
use core::fmt;
use devela::codegen::paste;
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

/// An extended grapheme cluster backed by a [`StaticU8String`].
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StaticU8Egc<const CAP: usize>(StaticU8String<CAP>);

impl_sized_alias![
    Egc, StaticU8Egc,
    "extended grapheme cluster, with a fixed capacity of ", ".":
    "A" 16, 1 "";
    "A" 24, 2 "s";
    "A" 32, 3 "s";
    "A" 40, 4 "s";
    "A" 48, 5 "s";
    "A" 56, 6 "s";
    "A" 64, 7 "s";
    "A" 128, 15 "s"
];

impl<const CAP: usize> StaticU8Egc<CAP> {
    /// Creates a new empty `StaticU8Egc`.
    ///
    /// # Panics
    /// Panics if `CAP` > 255.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(StaticU8String::new())
    }

    /// Creates a new `StaticU8Egc` from a `Char7`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < 1.
    ///
    /// Will never panic if `CAP` >= 1 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char7(c: Char7) -> Self {
        Self(StaticU8String::from_char7(c))
    }

    /// Creates a new `StaticU8Egc` from a `Char8`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 2 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char8(c: Char8) -> Self {
        Self(StaticU8String::from_char8(c))
    }

    /// Creates a new `StaticU8Egc` from a `Char16`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char16#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 3 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char16(c: Char16) -> Self {
        Self(StaticU8String::from_char16(c))
    }

    /// Creates a new `StaticU8Egc` from a `Char24`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char24#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char24(c: Char24) -> Self {
        Self(StaticU8String::from_char24(c))
    }

    /// Creates a new `StaticU8Egc` from a `Char32`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char32#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char32(c: Char32) -> Self {
        Self(StaticU8String::from_char32(c))
    }

    /// Creates a new `StaticU8Egc` from a `char`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Self {
        Self::from_char32(Char32(c))
    }

    //

    /// Returns the length in bytes.
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the current length is 0.
    pub const fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Returns the total capacity in bytes.
    #[inline]
    pub const fn capacity() -> usize {
        CAP
    }

    /// Returns the remaining capacity.
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline]
    pub const fn is_full(&self) -> bool {
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
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub const fn as_array(&self) -> [u8; CAP] {
        self.0.as_array()
    }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub const fn into_array(self) -> [u8; CAP] {
        self.0.into_array()
    }

    /// Returns the inner string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the mutable inner string slice.
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
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
}

/* traits */

impl<const CAP: usize> Strings for StaticU8Egc<CAP> {}
impl<const CAP: usize> Egcs for StaticU8Egc<CAP> {}

mod core_impls {
    use super::*;

    impl<const CAP: usize> Default for StaticU8Egc<CAP> {
        /// Returns an empty extended grapheme character.
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl<const CAP: usize> fmt::Display for StaticU8Egc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<const CAP: usize> fmt::Debug for StaticU8Egc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    // impl From<String> for StaticU8Egc {
    //     fn from(s: String) -> StaticU8Egc {
    //         StaticU8Egc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<&str> for StaticU8Egc {
    //     fn from(s: &str) -> StaticU8Egc {
    //         StaticU8Egc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<char> for StaticU8Egc {
    //     fn from(s: char) -> StaticU8Egc {
    //         StaticU8Egc(s.into())
    //     }
    // }
}

macro_rules! impl_from_char {
    // $char:ty char type
    // $for_name: `for` type name prefix
    // $bit: size in bits.
    ( $char:ty => $for_name:ident: $( $for_bit:expr ),+ ) => {
        $( impl_from_char![@$char => $for_name: $for_bit]; )+
    };
    ( @$char:ty => $for_name:ident: $for_bit:expr ) => { paste! {
        impl From<$char> for [< $for_name $for_bit >] {
            fn from(c: $char) -> [< $for_name $for_bit >] {
                let mut s = Self::default();
                let _ = s.0.push(c.into());
                s
            }
        }
    }};
    ( try $char:ty => $for_name:ident: $( $for_bit:expr ),+ ) => {
        $( impl_from_char![@try $char => $for_name: $for_bit]; )+
    };
    ( @try $char:ty => $for_name:ident: $for_bit:expr ) => { paste! {
        impl TryFrom<$char> for [< $for_name $for_bit >] {
            type Error = Error;
            fn try_from(c: $char) -> Result<[< $for_name $for_bit >]> {
                let mut s = Self::default();
                s.0.try_push(c.into())?;
                Ok(s)
            }
        }
    }};
}
impl_from_char![Char7 => Egc: 16, 24, 32, 40, 48, 56, 64, 128];
impl_from_char![Char8 => Egc: 24, 32, 40, 48, 56, 64, 128];
impl_from_char![try Char8 => Egc: 16];
impl_from_char![Char16 => Egc: 32, 40, 48, 56, 64, 128];
impl_from_char![try Char16 => Egc: 16, 24];
impl_from_char![Char24 => Egc: 40, 48, 56, 64, 128];
impl_from_char![try Char24 => Egc: 16, 24, 32];
impl_from_char![Char32 => Egc: 40, 48, 56, 64, 128];
impl_from_char![try Char32 => Egc: 16, 24, 32];
impl_from_char![char => Egc: 40, 48, 56, 64, 128];
impl_from_char![try char => Egc: 16, 24, 32];
