// textas::string::non_nul
//
//! Non-nul `String` backed by an array.
//

#[cfg(feature = "alloc")]
use alloc::ffi::CString;

use crate::error::{TextosError, TextosResult};
use core::fmt;
use devela::paste;

/// The nul character.
const NUL: char = '\0';

/// A string backed by an array that guarantees not to contain nul characters.
///
/// Internally, the first 0 byte in the array indicates the end of the string.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonNulStringStatic<const CAP: usize> {
    arr: [u8; CAP],
}

impl<const CAP: usize> NonNulStringStatic<CAP> {
    /// Creates a new empty `NonNulStrigSized`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total capacity of a in bytes.
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

    //

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.arr = [0; CAP];
    }

    /// Returns a byte slice of the inner string slice.
    ///
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        #[cfg(feature = "safe")]
        return self.arr.get(0..self.len()).unwrap();

        #[cfg(not(feature = "safe"))]
        return unsafe { self.arr.get_unchecked(0..self.len()) };
    }

    /// Returns a mutable byte slice of the inner string slice.
    #[inline]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        self.arr.get_unchecked_mut(0..len)
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub fn as_array(&self) -> [u8; CAP] {
        self.arr
    }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub fn into_array(self) -> [u8; CAP] {
        self.arr
    }

    /// Returns the inner string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        #[cfg(feature = "safe")]
        return core::str::from_utf8(self.arr.get(0..self.len()).unwrap())
            .expect("must be valid utf-8");

        #[cfg(not(feature = "safe"))]
        unsafe {
            return core::str::from_utf8_unchecked(self.arr.get_unchecked(0..self.len()));
        }
    }

    /// Returns the mutable inner string slice.
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        &mut *(self.as_bytes_mut() as *mut [u8] as *mut str)
    }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString {
        CString::new(self.to_string()).unwrap()
    }

    /// Removes the last character and returns it, or `None` if
    /// the string is empty.
    pub fn pop(&mut self) -> Option<char> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            let mut idx_last_char = len - 1;
            while idx_last_char > 0 && !self.as_str().is_char_boundary(idx_last_char) {
                idx_last_char -= 1;
            }
            let last_char = self.as_str()[idx_last_char..len].chars().next().unwrap();
            for i in idx_last_char..len {
                self.arr[i] = 0;
            }
            Some(last_char)
        }
    }

    /// Tries to remove the last character and return it.
    ///
    /// # Errors
    /// Returns an error if the string is empty.
    pub fn try_pop(&mut self) -> TextosResult<char> {
        let len = self.len();
        if len == 0 {
            Err(TextosError::NotEnoughElements(1))
        } else {
            let mut idx_last_char = len - 1;
            while idx_last_char > 0 && !self.as_str().is_char_boundary(idx_last_char) {
                idx_last_char -= 1;
            }
            let last_char = self.as_str()[idx_last_char..len].chars().next().unwrap();
            for i in idx_last_char..len {
                self.arr[i] = 0;
            }
            Ok(last_char)
        }
    }

    /// Appends to the end the given `character`.
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

    /// Tries to append to the end the given `character`.
    ///
    /// Returns the number of bytes written.
    ///
    /// Trying to push a nul character does nothing and returns 0 bytes.
    ///
    /// # Errors
    /// Returns an error if the capacity is not enough to hold the given character.
    pub fn try_push(&mut self, character: char) -> TextosResult<usize> {
        let char_len = character.len_utf8();

        if character == NUL {
            Ok(0)
        } else if self.remaining_capacity() >= char_len {
            let len = self.len();
            let new_len = len + char_len;

            let _ = character.encode_utf8(&mut self.arr[len..new_len]);
            Ok(char_len)
        } else {
            Err(TextosError::NotEnoughCapacity(char_len))
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
    pub fn try_push_str(&mut self, string: &str) -> TextosResult<usize> {
        let first_char_len = string
            .chars()
            .find(|&c| c != NUL)
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        if self.remaining_capacity() < first_char_len {
            Err(TextosError::NotEnoughCapacity(first_char_len))
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
    pub fn try_push_str_complete(&mut self, string: &str) -> TextosResult<usize> {
        let non_nul_len = string.as_bytes().iter().filter(|x| **x != 0).count();

        if self.remaining_capacity() >= non_nul_len {
            Ok(self.push_str(string))
        } else {
            Err(TextosError::NotEnoughCapacity(non_nul_len))
        }
    }
}

/* traits */

impl<const CAP: usize> Default for NonNulStringStatic<CAP> {
    /// Returns an empty string.
    fn default() -> Self {
        Self { arr: [0; CAP] }
    }
}

impl<const CAP: usize> fmt::Display for NonNulStringStatic<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl<const CAP: usize> fmt::Debug for NonNulStringStatic<CAP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

/* specific sizes */

macro_rules! impl_sizes {
    // $bits_det: determinant for the number of bits.
    // $bits: number of bits.
    // $bytes: number of bytes.
    // $byte_plu: plural for the number of bytes.
    ( $($bits_det:literal $bits:literal, $bytes:literal $bytes_plu:literal);+ ) => {
        $(
        impl_sizes![@$bits_det $bits, $bytes $bytes_plu];
        )+
    };
    (@$bits_det:literal $bits:literal, $bytes:literal $bytes_plu:literal) => { paste! {
        #[doc = "" $bits_det " " $bits "-bit non-nul UTF-8 string, with a fixed capacity of "
        $bytes " byte" $bytes_plu "."]
        pub type [<NonNulString$bits>] = NonNulStringStatic<$bytes>;
    }};
}
impl_sizes![
    "An" 8, 1 "";
    "A" 16, 2 "s";
    "A" 24, 3 "s";
    "A" 32, 4 "s";
    "A" 64, 8 "s";
    "A" 128, 16 "s";
    "A" 256, 32 "s";
    "A" 512, 64 "s";
    "A" 1024, 128 "s"
];

macro_rules! impl_from_char {
    // $for_name: `for` type name prefix
    // $bit: size in bits.
    ( $for_name:ident: $( $for_bit:expr ),+ ) => {
        $( impl_from_char![@$for_name: $for_bit]; )+
    };
    ( @$for_name:ident: $for_bit:expr ) => { paste! {
        impl From<char> for [< $for_name $for_bit >] {
            fn from(c: char) -> [< $for_name $for_bit >] {
                let mut s = Self::default();
                let _ = s.push(c);
                s
            }
        }
    }};
    (try $for_name:ident: $( $for_bit:expr ),+ ) => {
        $( impl_from_char![@try $for_name: $for_bit]; )+
    };
    ( @try $for_name:ident: $for_bit:expr ) => { paste! {
        impl TryFrom<char> for [< $for_name $for_bit >] {
            type Error = TextosError;
            fn try_from(c: char) -> TextosResult<[< $for_name $for_bit >]> {
                let mut s = Self::default();
                s.try_push(c)?;
                Ok(s)
            }
        }
    }};
}
impl_from_char![NonNulString: 32, 64, 128, 256, 512, 1024];
impl_from_char![NonNulString: 8, 16];
