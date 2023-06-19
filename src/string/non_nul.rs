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

macro_rules! impl_string {
    // $name: type name prefix
    // $doc_det: determinant for $bit: "An" | "A".
    // $doc_suf: suffix for plural of `byte`: "s" | "".
    // $bit: size in bits.
    // $cap: capacity in bytes.
    ( $name:ident:
      $(
        $doc_det:literal, $doc_suf:literal,
        $bit:expr, $cap:expr
    );+ ) => {
        $( impl_string![@$name: $doc_det, $doc_suf, $bit, $cap]; )+
    };
    (@$name:ident:
     $doc_det:literal,
     $doc_suf:literal,
     $bit:expr,
     $cap:expr
     ) => { paste! {
        #[doc = "" $doc_det " " $bit "-bit non-nul UTF-8 string, with a fixed capacity of "
        $cap " byte" $doc_suf "."]
        ///
        /// The type can contain nul bytes, but those will never be part of the string content.
        /// The first nul byte will indicate the end of the string.
        #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<$name $bit>] {
            arr: [u8; $cap],
        }

        /* String methods */

        impl [<$name $bit>] {
            #[doc = "Creates a new empty `" [<$name $bit>]"`."]
            #[inline]
            pub fn new() -> [<$name $bit>] {
                Self::default()
            }

            //

            #[doc = "Returns the total capacity of a `" [<$name $bit>]"` in bytes."]
            #[inline]
            pub const fn capacity() -> usize { $cap }

            /// Returns the remaining capacity.
            #[inline]
            pub fn remaining_capacity(&self) -> usize {
                $cap - self.len()
            }

            /// Returns the current length.
            #[inline]
            pub fn len(&self) -> usize {
                self.arr.iter().position(|&x| x == 0).unwrap_or(self.arr.len())
            }

            /// Returns `true` if the current length is 0.
            #[inline]
            pub fn is_empty(&self) -> bool { self.len() == 0 }

            /// Returns `true` if the current remaining capacity is 0.
            #[inline]
            pub fn is_full(&self) -> bool { self.len() == $cap }

            //

            /// Sets the length to 0, by resetting all bytes to 0.
            #[inline]
            pub fn clear(&mut self) {
                self.arr = [0; $cap];
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
            pub fn as_array(&self) -> [u8; $cap] {
                self.arr
            }

            /// Returns the inner array with the full contents.
            ///
            /// The array contains all the bytes, including those outside the current length.
            #[inline]
            pub fn into_array(self) -> [u8; $cap] {
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
                let first_char_len = string.chars().find(|&c| c != NUL).map(|c| c.len_utf8()).unwrap_or(0);
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

        impl Default for [<$name $bit>] {
            /// Returns an empty string.
            fn default() -> Self {
                Self {
                    arr: [0; $cap],
                }
            }
        }

        impl fmt::Display for [<$name $bit>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
        impl fmt::Debug for [<$name $bit>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self.as_str())
            }
        }
    }};
}
impl_string![NonNulString:
    "An", "", 8, 1; "A", "s", 16, 2; "A", "s", 32, 4; "A", "s", 64, 8; "A", "s", 128, 16];

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
impl_from_char![NonNulString: 32, 64, 128];
impl_from_char![NonNulString: 8, 16];

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use core::mem::size_of;
//
//     #[test]
//     fn sizes() {
//         assert_eq![8, size_of::<usize>()];
//
//         assert_eq![8, size_of::<String64>()];
//         assert_eq![16, size_of::<String128>()];
//         assert_eq![32, size_of::<String256>()];
//         assert_eq![64, size_of::<String512>()];
//         assert_eq![128, size_of::<String1024>()];
//         assert_eq![256, size_of::<String2048>()];
//     }
//
//     #[test]
//     fn push() {
//         let mut s = String32::new(); // max capacity == 3
//
//         assert![s.try_push('ñ').is_ok()];
//         assert_eq![2, s.len()];
//         assert![s.try_push('ñ').is_err()];
//         assert_eq![2, s.len()];
//         assert![s.try_push('a').is_ok()];
//         assert_eq![3, s.len()];
//     }
//
//     // TODO
//     #[test]
//     fn pop() {
//         let mut s = String32::new(); // max capacity == 3
//
//         s.push('ñ');
//         s.push('a');
//         assert_eq![Some('a'), s.pop()];
//         assert_eq![Some('ñ'), s.pop()];
//         assert_eq![None, s.pop()];
//     }
// }
