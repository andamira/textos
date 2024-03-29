// textos::unicode::string::non_nul
//
//! Non-nul `String` backed by an array.
//
// TOC
// - definitions
// - trait impls
// - conversions

#[cfg(feature = "alloc")]
use alloc::{ffi::CString, str::Chars, string::ToString};

use crate::{
    error::{TextosError as Error, TextosResult as Result},
    macros::impl_sized_alias,
    unicode::char::*,
};
use core::fmt;
use devela::codegen::paste;

/* definitions */

/// The nul character.
const NUL: char = '\0';

/// A UTF-8-encoded string, backed by an array of constant capacity.
/// Can't contain nul chars.
///
/// Internally, the first 0 byte in the array indicates the end of the string.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticNonNulString<const CAP: usize> {
    arr: [u8; CAP],
}

impl_sized_alias![
    NonNulString, StaticNonNulString,
    "UTF-8-encoded string, with fixed capacity of ",
    ", no nul chars.":
    "An" 8, 1 "";
    "A" 16, 2 "s";
    "A" 24, 3 "s";
    "A" 32, 4 "s";
    "A" 40, 5 "s";
    "A" 48, 6 "s";
    "A" 56, 7 "s";
    "A" 64, 8 "s";
    "A" 128, 16 "s";
    "A" 256, 32 "s";
    "A" 512, 64 "s";
    "A" 1024, 128 "s";
    "A" 2048, 256 "s"
];

impl<const CAP: usize> StaticNonNulString<CAP> {
    /// Creates a new empty `StaticNonNulString`.
    #[inline]
    pub const fn new() -> Self {
        Self { arr: [0; CAP] }
    }

    /// Creates a new `StaticNonNulString` from a `Char7`.
    ///
    /// If `c`.[`is_nul()`][Char7#method.is_nul] an empty string will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < 1
    ///
    /// Will never panic if `CAP` >= 1.
    #[inline]
    pub const fn from_char7(c: Char7) -> Self {
        let mut new = Self::new();
        if !c.is_nul() {
            new.arr[0] = c.to_utf8_bytes()[0];
        }
        new
    }

    /// Creates a new `StaticU8String` from a `Char8`.
    ///
    /// If `c`.[`is_nul()`][Char8#method.is_nul] an empty string will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 2.
    #[inline]
    pub const fn from_char8(c: Char8) -> Self {
        let mut new = Self::new();
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = char_utf8_2bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
        }
        new
    }

    /// Creates a new `StaticU8String` from a `Char16`.
    ///
    /// If `c`.[`is_nul()`][Char16#method.is_nul] an empty string will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 3.
    #[inline]
    pub const fn from_char16(c: Char16) -> Self {
        let mut new = Self::new();
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = char_utf8_3bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
            if len > 2 {
                new.arr[2] = bytes[2];
            }
        }
        new
    }

    /// Creates a new `StaticU8String` from a `Char24`.
    ///
    /// If `c`.[`is_nul()`][Char24#method.is_nul] an empty string will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4.
    #[inline]
    pub const fn from_char24(c: Char24) -> Self {
        let mut new = Self::new();
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = char_utf8_4bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
            if len > 2 {
                new.arr[2] = bytes[2];
            }
            if len > 3 {
                new.arr[3] = bytes[3];
            }
        }
        new
    }

    /// Creates a new `StaticU8String` from a `Char32`.
    ///
    /// If `c`.[`is_nul()`][Char32#method.is_nul] an empty string will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will never panic if `CAP` is >= 4.
    #[inline]
    pub const fn from_char32(c: Char32) -> Self {
        let mut new = Self::new();
        if !c.is_nul() {
            let bytes = c.to_utf8_bytes();
            let len = char_utf8_4bytes_len(bytes);

            new.arr[0] = bytes[0];
            if len > 1 {
                new.arr[1] = bytes[1];
            }
            if len > 2 {
                new.arr[2] = bytes[2];
            }
            if len > 3 {
                new.arr[3] = bytes[3];
            }
        }
        new
    }

    /// Creates a new `StaticU8String` from a `char`.
    ///
    /// If `c`.[`is_nul()`][Chars#method.is_nul] an empty string will be returned.
    ///
    /// # Panic
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` is >= 4.
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
    pub fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    /// Returns the current length.
    #[inline]
    pub fn len(&self) -> usize {
        self.arr
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(self.arr.len())
    }

    /// Returns `true` if the current length is 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len() == CAP
    }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.arr = [0; CAP];
    }

    //

    /// Returns a byte slice of the inner string slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        #[cfg(not(feature = "unsafe"))]
        return self.arr.get(0..self.len()).unwrap();

        #[cfg(feature = "unsafe")]
        return unsafe { self.arr.get_unchecked(0..self.len()) };
    }

    /// Returns a mutable byte slice of the inner string slice.
    #[inline]
    #[cfg(feature = "unsafe")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        self.arr.get_unchecked_mut(0..len)
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
    #[inline]
    pub fn as_str(&self) -> &str {
        #[cfg(not(feature = "unsafe"))]
        return core::str::from_utf8(self.arr.get(0..self.len()).unwrap())
            .expect("must be valid utf-8");

        // SAFETY
        #[cfg(feature = "unsafe")]
        unsafe {
            return core::str::from_utf8_unchecked(self.arr.get_unchecked(0..self.len()));
        }
    }

    /// Returns the mutable inner string slice.
    #[cfg(feature = "unsafe")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        &mut *(self.as_bytes_mut() as *mut [u8] as *mut str)
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
        if self.is_empty() {
            None
        } else {
            Some(self.pop_unchecked())
        }
    }

    /// Tries to remove the last character and return it.
    ///
    /// # Errors
    /// Returns an error if the string is empty.
    #[inline]
    pub fn try_pop(&mut self) -> Result<char> {
        if self.is_empty() {
            Err(Error::NotEnoughElements(1))
        } else {
            Ok(self.pop_unchecked())
        }
    }

    /// Removes the last character and returns it.
    ///
    /// # Panics
    /// Panics if the string is empty.
    #[inline]
    pub fn pop_unchecked(&mut self) -> char {
        let len = self.len();
        let mut idx_last_char = len - 1;
        while idx_last_char > 0 && !self.as_str().is_char_boundary(idx_last_char) {
            idx_last_char -= 1;
        }
        let last_char = self.as_str()[idx_last_char..len].chars().next().unwrap();
        for i in idx_last_char..len {
            self.arr[i] = 0;
        }
        last_char
    }

    /// Appends to the end of the string the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// It will return 0 bytes if the given `character` doesn't fit in
    /// the remaining capacity, or if it is the nul character.
    pub fn push(&mut self, character: char) -> usize {
        let char_len = character.len_utf8();

        if character != NUL && self.remaining_capacity() >= char_len {
            let len = self.len();
            let new_len = len + char_len;

            let _ = character.encode_utf8(&mut self.arr[len..new_len]);
            char_len
        } else {
            0
        }
    }

    /// Tries to append to the end of the string the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// Trying to push a nul character does nothing and returns 0 bytes.
    ///
    /// # Errors
    /// Returns an error if the capacity is not enough to hold the given character.
    pub fn try_push(&mut self, character: char) -> Result<usize> {
        let char_len = character.len_utf8();

        if character == NUL {
            Ok(0)
        } else if self.remaining_capacity() >= char_len {
            let len = self.len();
            let new_len = len + char_len;

            let _ = character.encode_utf8(&mut self.arr[len..new_len]);
            Ok(char_len)
        } else {
            Err(Error::NotEnoughCapacity(char_len))
        }
    }

    /// Appends to the end the fitting characters from the given `string` slice.
    ///
    /// Nul characters will be stripped out.
    ///
    /// Returns the number of bytes written, which will be 0 if not even the first
    /// non-nul character can fit.
    pub fn push_str(&mut self, string: &str) -> usize {
        let mut rem_cap = self.remaining_capacity();
        let mut bytes_written = 0;

        for c in string.chars() {
            if c != NUL {
                let char_len = c.len_utf8();

                if char_len <= rem_cap {
                    self.push(c);
                    rem_cap -= char_len;
                    bytes_written += char_len;
                } else {
                    break;
                }
            }
        }
        bytes_written
    }

    /// Tries to append to the end the fitting characters from the given `string` slice.
    ///
    /// Nul characters will be stripped out.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Errors
    /// Returns an error if the capacity is not enough to hold even the
    /// first non-nul character.
    pub fn try_push_str(&mut self, string: &str) -> Result<usize> {
        let first_char_len = string
            .chars()
            .find(|&c| c != NUL)
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        if self.remaining_capacity() < first_char_len {
            Err(Error::NotEnoughCapacity(first_char_len))
        } else {
            Ok(self.push_str(string))
        }
    }

    /// Tries to append the complete `string` slice to the end.
    ///
    /// Returns the number of bytes written in success.
    ///
    /// Nul characters will not be taken into account.
    ///
    /// # Errors
    /// Returns an error if the slice wont completely fit.
    #[inline]
    pub fn try_push_str_complete(&mut self, string: &str) -> Result<usize> {
        let non_nul_len = string.as_bytes().iter().filter(|x| **x != 0).count();

        if self.remaining_capacity() >= non_nul_len {
            Ok(self.push_str(string))
        } else {
            Err(Error::NotEnoughCapacity(non_nul_len))
        }
    }
}

/* traits */

impl<const CAP: usize> Default for StaticNonNulString<CAP> {
    /// Returns an empty string.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize> fmt::Display for StaticNonNulString<CAP> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl<const CAP: usize> fmt::Debug for StaticNonNulString<CAP> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
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
impl_from_char![Char7 => NonNulString: 8, 16, 24, 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![Char8 => NonNulString: 16, 24, 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char8 => NonNulString: 8];
impl_from_char![Char16 => NonNulString: 24, 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char16 => NonNulString: 8, 16];
impl_from_char![Char24 => NonNulString: 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char24 => NonNulString: 8, 16, 24];
impl_from_char![Char32 => NonNulString: 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try Char32 => NonNulString: 8, 16, 24];
impl_from_char![char => NonNulString: 32, 40, 48, 56, 64, 128, 256, 512, 1024, 2048];
impl_from_char![try char => NonNulString: 8, 16, 24];
