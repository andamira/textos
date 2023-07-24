// textos::unicode::egc::non_nul
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
        string::{StaticNonNulString, Strings},
    },
};
#[cfg(feature = "alloc")]
use alloc::{ffi::CString, str::Chars};
use core::fmt;
use devela::paste;
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

/// An extended grapheme cluster backed by a [`StaticNonNulString`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StaticNonNulEgc<const CAP: usize>(StaticNonNulString<CAP>);

impl_sized_alias![
    NonNulEgc, StaticNonNulEgc,
    "extended grapheme cluster, with a fixed capacity of ",
    ", that can't contain nul characters.":
    "An" 8, 1 "";
    "A" 16, 2 "s";
    "A" 24, 3 "s";
    "A" 32, 4 "s";
    "A" 40, 5 "s";
    "A" 48, 6 "s";
    "A" 56, 7 "s";
    "A" 64, 8 "s";
    "A" 128, 16 "s"
];

/* impls */

impl<const CAP: usize> StaticNonNulEgc<CAP> {
    /// Creates a new empty `StaticNonNulString`.
    #[inline]
    pub const fn new() -> Self {
        Self(StaticNonNulString::new())
    }

    /// Creates a new `StaticNonNulEgc` from a `Char7`.
    ///
    /// If `c`.[`is_nul()`][Char7#method.is_nul] an empty egc will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < 1.
    ///
    /// Will never panic if `CAP` >= 1.
    #[inline]
    #[must_use]
    pub const fn from_char7(c: Char7) -> Self {
        Self(StaticNonNulString::from_char7(c))
    }

    /// Creates a new `StaticNonNulEgc` from a `Char8`.
    ///
    /// If `c`.[`is_nul()`][Char8#method.is_nul] an empty egc will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 2.
    #[inline]
    #[must_use]
    pub const fn from_char8(c: Char8) -> Self {
        Self(StaticNonNulString::from_char8(c))
    }

    /// Creates a new `StaticNonNulEgc` from a `Char16`.
    ///
    /// If `c`.[`is_nul()`][Char16#method.is_nul] an empty egc will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 3
    #[inline]
    #[must_use]
    pub const fn from_char16(c: Char16) -> Self {
        Self(StaticNonNulString::from_char16(c))
    }

    /// Creates a new `StaticNonNulEgc` from a `Char24`.
    ///
    /// If `c`.[`is_nul()`][Char24#method.is_nul] an empty egc will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4.
    #[inline]
    #[must_use]
    pub const fn from_char24(c: Char24) -> Self {
        Self(StaticNonNulString::from_char24(c))
    }

    /// Creates a new `StaticNonNulEgc` from a `Char32`.
    ///
    /// If `c`.[`is_nul()`][Char32#method.is_nul] an empty egc will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4.
    #[inline]
    #[must_use]
    pub const fn from_char32(c: Char32) -> Self {
        Self(StaticNonNulString::from_char32(c))
    }

    /// Creates a new `StaticNonNulEgc` from a `char`.
    ///
    /// If `c`.[`is_nul()`][Chars#method.is_nul] an empty egc will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Self {
        Self::from_char32(Char32(c))
    }

    //

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
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
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

    //
}

/* traits */

impl<const CAP: usize> Strings for StaticNonNulEgc<CAP> {}
impl<const CAP: usize> Egcs for StaticNonNulEgc<CAP> {}

mod core_impls {
    use super::*;

    impl<const CAP: usize> Default for StaticNonNulEgc<CAP> {
        /// Returns an empty extended grapheme character.
        #[inline]
        fn default() -> Self {
            Self::new()
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

    // TODO
    // impl<const CAP: usize> From<String> for StaticNonNulEgc<CAP> {
    //     fn from(s: String) -> StaticNonNulEgc<CAP> {
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

/* conversions */

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
                let mut s = Self::new();
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
                let mut s = Self::new();
                s.0.try_push(c.into())?;
                Ok(s)
            }
        }
    }};
}
impl_from_char![Char7 => NonNulEgc: 8, 16, 24, 32, 40, 48, 56, 64, 128];
impl_from_char![Char8 => NonNulEgc: 16, 24, 32, 40, 48, 56, 64, 128];
impl_from_char![try Char8 => NonNulEgc: 8];
impl_from_char![Char16 => NonNulEgc: 24, 32, 40, 48, 56, 64, 128];
impl_from_char![try Char16 => NonNulEgc: 8, 16];
impl_from_char![Char24 => NonNulEgc: 32, 40, 48, 56, 64, 128];
impl_from_char![try Char24 => NonNulEgc: 8, 16, 24];
impl_from_char![Char32 => NonNulEgc: 32, 40, 48, 56, 64, 128];
impl_from_char![try Char32 => NonNulEgc: 8, 16, 24];
impl_from_char![char => NonNulEgc: 32, 40, 48, 56, 64, 128];
impl_from_char![try char => NonNulEgc: 8, 16, 24];
