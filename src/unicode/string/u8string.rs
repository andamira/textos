// textos::unicode::string::static
//
//! `String` backed by an array.
//
// TOC
// - definitions
// - trait impls
// - conversions
// - tests

#[cfg(feature = "alloc")]
use alloc::{ffi::CString, str::Chars};

use crate::{
    error::{TextosError as Error, TextosResult as Result},
    macros::impl_sized_alias,
    unicode::char::*,
};
use core::{fmt, ops::Deref};
use devela::paste;

/* definitions */

/// A UTF-8-encoded string, backed by an array,
/// with a maximum constant capacity of 255 bytes.
///
/// Internally, the current length is stored as a [`u8`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticU8String<const CAP: usize> {
    // WAITING for when we can use CAP: u8 for panic-less const boundary check.
    arr: [u8; CAP],
    len: u8,
}

impl_sized_alias![
    String, StaticU8String,
    "UTF-8-encoded string, with a fixed capacity of ", ".":
    "A" 16, 1 "";
    "A" 24, 2 "s";
    "A" 32, 3 "s";
    "A" 40, 4 "s";
    "A" 48, 5 "s";
    "A" 56, 6 "s";
    "A" 64, 7 "s";
    "A" 128, 15 "s";
    "A" 256, 31 "s";
    "A" 512, 63 "s";
    "A" 1024, 127 "s";
    "A" 2048, 255 "s"
];

impl<const CAP: usize> StaticU8String<CAP> {
    /// Creates a new empty `StaticU8String`.
    ///
    /// # Panics
    /// Panics if `CAP` > 255.
    #[inline]
    pub const fn new() -> Self {
        assert![CAP <= 255];
        Self {
            arr: [0; CAP],
            len: 0,
        }
    }

    /// Creates a new `StaticU8String` from a `Char7`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < 1.
    ///
    /// Will never panic if `CAP` >= 1 and <= 255.
    #[inline]
    pub const fn from_char7(c: Char7) -> Self {
        let mut new = Self::new();
        new.arr[0] = c.to_utf8_bytes()[0];
        new.len = 1;
        new
    }

    /// Creates a new `StaticU8String` from a `Char8`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 2 and <= 255.
    #[inline]
    pub const fn from_char8(c: Char8) -> Self {
        let mut new = Self::new();

        let bytes = c.to_utf8_bytes();
        new.len = char_utf8_2bytes_len(bytes);

        new.arr[0] = bytes[0];
        if new.len > 1 {
            new.arr[1] = bytes[1];
        }
        new
    }

    /// Creates a new `StaticU8String` from a `Char16`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char16#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 3 and <= 255.
    #[inline]
    pub const fn from_char16(c: Char16) -> Self {
        let mut new = Self::new();

        let bytes = c.to_utf8_bytes();
        new.len = char_utf8_3bytes_len(bytes);

        new.arr[0] = bytes[0];
        if new.len > 1 {
            new.arr[1] = bytes[1];
        }
        if new.len > 2 {
            new.arr[2] = bytes[2];
        }
        new
    }

    /// Creates a new `StaticU8String` from a `Char24`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char24#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    pub const fn from_char24(c: Char24) -> Self {
        let mut new = Self::new();

        let bytes = c.to_utf8_bytes();
        new.len = char_utf8_4bytes_len(bytes);

        new.arr[0] = bytes[0];
        if new.len > 1 {
            new.arr[1] = bytes[1];
        }
        if new.len > 2 {
            new.arr[2] = bytes[2];
        }
        if new.len > 3 {
            new.arr[3] = bytes[3];
        }
        new
    }

    /// Creates a new `StaticU8String` from a `Char32`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char32#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    pub const fn from_char32(c: Char32) -> Self {
        let mut new = Self::new();

        let bytes = c.to_utf8_bytes();
        new.len = char_utf8_4bytes_len(bytes);

        new.arr[0] = bytes[0];
        if new.len > 1 {
            new.arr[1] = bytes[1];
        }
        if new.len > 2 {
            new.arr[2] = bytes[2];
        }
        if new.len > 3 {
            new.arr[3] = bytes[3];
        }
        new
    }

    /// Creates a new `StaticU8String` from a `char`.
    ///
    /// # Panic
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    pub const fn from_char(c: char) -> Self {
        Self::from_char32(Char32(c))
    }

    //

    /// Returns the total capacity in bytes.
    #[inline]
    pub const fn capacity() -> usize {
        CAP
    }

    /// Returns the remaining capacity.
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len as usize
    }

    /// Returns the current length.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns `true` if the current length is 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len == CAP as u8
    }

    /// Sets the length to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Sets the length to 0, and resets all the bytes to 0.
    #[inline]
    pub fn reset(&mut self) {
        self.arr = [0; CAP];
        self.len = 0;
    }

    //

    /// Returns a byte slice of the inner string slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        #[cfg(not(feature = "safe"))]
        unsafe {
            self.arr.get_unchecked(0..self.len as usize)
        }

        #[cfg(feature = "safe")]
        self.arr
            .get(0..self.len as usize)
            .expect("len must be <= arr.len()")
    }

    /// Returns a mutable byte slice of the inner string slice.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.arr.get_unchecked_mut(0..self.len as usize)
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub const fn as_array(&self) -> [u8; CAP] {
        self.arr
    }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub const fn into_array(self) -> [u8; CAP] {
        self.arr
    }

    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        #[cfg(not(feature = "safe"))]
        unsafe {
            core::str::from_utf8_unchecked(
                self.arr
                    .get(0..self.len as usize)
                    .expect("len must be <= arr.len()"),
            )
        }
        #[cfg(feature = "safe")]
        core::str::from_utf8(
            self.arr
                .get(0..self.len as usize)
                .expect("len must be <= arr.len()"),
        )
        .expect("must be valid utf-8")
    }

    /// Returns the mutable inner string slice.
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub fn as_str_mut(&mut self) -> &mut str {
        unsafe { &mut *(self.as_bytes_mut() as *mut [u8] as *mut str) }
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub fn chars(&self) -> Chars {
        self.as_str().chars()
    }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString {
        CString::new(self.to_string()).unwrap()
    }

    //

    /// Removes the last character and returns it, or `None` if
    /// the string is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        self.as_str().chars().last().map(|c| {
            self.len -= c.len_utf8() as u8;
            c
        })
    }

    /// Tries to remove the last character and returns it, or `None` if
    /// the string is empty.
    ///
    /// # Errors
    /// Returns an error if the string is empty.
    #[inline]
    pub fn try_pop(&mut self) -> Result<char> {
        self.as_str()
            .chars()
            .last()
            .map(|c| {
                self.len -= c.len_utf8() as u8;
                c
            })
            .ok_or(Error::NotEnoughElements(1))
    }

    /// Appends to the end of the string the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// It will return 0 bytes if the given `character` doesn't fit in
    /// the remaining capacity.
    pub fn push(&mut self, character: char) -> usize {
        let char_len = character.len_utf8();
        if self.remaining_capacity() >= char_len {
            let beg = self.len as usize;
            let end = beg + char_len;
            let _ = character.encode_utf8(&mut self.arr[beg..end]);
            self.len += char_len as u8;
            char_len
        } else {
            0
        }
    }

    /// Tries to append to the end of the string the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Errors if the capacity is not enough to hold the `character`.
    pub fn try_push(&mut self, character: char) -> Result<usize> {
        let char_len = character.len_utf8();
        if self.remaining_capacity() >= char_len {
            let beg = self.len as usize;
            let end = beg + char_len;
            let _ = character.encode_utf8(&mut self.arr[beg..end]);
            self.len += char_len as u8;
            Ok(char_len)
        } else {
            Err(Error::NotEnoughCapacity(char_len))
        }
    }
}

/* traits */

impl<const CAP: usize> Default for StaticU8String<CAP> {
    /// Returns an empty string.
    ///
    /// # Panics
    /// Panics if `CAP` > 255.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize> fmt::Display for StaticU8String<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const CAP: usize> fmt::Debug for StaticU8String<CAP> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl<const CAP: usize> Deref for StaticU8String<CAP> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
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
                let mut s = Self::default();
                let _ = s.push(c.into());
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
                s.try_push(c.into())?;
                Ok(s)
            }
        }
    }};
}
impl_from_char![Char7 => String: 16, 24, 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![Char8 => String: 24, 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char8 => String: 16];
impl_from_char![Char16 => String: 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char16 => String: 16, 24];
impl_from_char![Char24 => String: 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char24 => String: 16, 24, 32];
impl_from_char![Char32 => String: 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char32 => String: 16, 24, 32];
impl_from_char![char => String: 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try char => String: 16, 24, 32];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut s = String32::new(); // max capacity == 3

        assert![s.try_push('ñ').is_ok()];
        assert_eq![2, s.len()];
        assert![s.try_push('ñ').is_err()];
        assert_eq![2, s.len()];
        assert![s.try_push('a').is_ok()];
        assert_eq![3, s.len()];
    }

    // TODO
    #[test]
    fn pop() {
        let mut s = String32::new(); // max capacity == 3

        s.push('ñ');
        s.push('a');
        assert_eq![Some('a'), s.pop()];
        assert_eq![Some('ñ'), s.pop()];
        assert_eq![None, s.pop()];
    }
}
