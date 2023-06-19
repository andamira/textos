// textas::string::sized
//
//! `String` backed by an array.
//

use crate::error::{TextosError as Error, TextosResult as Result};
use core::{fmt, ops::Deref};
use devela::paste;

macro_rules! impl_string {
    // $name: type name prefix
    // $bit: size in bits.
    // $cap: capacity in bytes.
    ( $name:ident:
      $(
        $doc_det:literal, $doc_suf:literal,
        $bit:expr, $cap:expr );+
    ) => {
        $( impl_string![@$name: $doc_det, $doc_suf, $bit, $cap]; )+
    };
    (@$name:ident:
     $doc_det:literal,
     $doc_suf:literal,
     $bit:expr,
     $cap:expr
     ) => { paste! {
        #[doc = "" $doc_det " " $bit "-bit UTF-8 string, with a fixed capacity of "
        $cap " byte" $doc_suf "."]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<$name $bit>] {
            arr: [u8; $cap],
            len: u8,
        }

        /* String methods */

        impl [<$name $bit>] {
            #[doc = "Creates a new empty `" [<$name $bit>]"`."]
            pub fn new() -> [<$name $bit>] {
                Self::default()
            }

            // TODO
            // #[doc = "Converts a slice of bytes to a "[<$name $bit>]"."]
            // pub fn from_utf8(&self, bytes: &[u8]) -> Result<[<$name $bit>]> {
            //     todo![];
            //
            //     match std::str::from_utf8(input) {
            //         Ok(valid) => {
            //             Self::from_str(valid);
            //         }
            //
            //     }
            // }

            //

            #[doc = "Returns the total capacity of a `" [<$name $bit>]"`."]
            #[inline]
            pub const fn capacity() -> usize { $cap as usize }

            /// Returns the remaining capacity.
            #[inline]
            pub const fn remaining_capacity(&self) -> usize { $cap as usize - self.len as usize }

            /// Returns the current length.
            #[inline]
            pub const fn len(&self) -> usize { self.len as usize }

            /// Returns `true` if the current length is 0.
            #[inline]
            pub const fn is_empty(&self) -> bool { self.len == 0 }

            /// Returns `true` if the current remaining capacity is 0.
            #[inline]
            pub const fn is_full(&self) -> bool { self.len == $cap }

            //

            /// Sets the length to 0.
            pub fn clear(&mut self) {
                self.len = 0;
            }

            /// Don't just set the length to 0, but also set the bytes to 0.
            pub fn really_clear(&mut self) {
                self.arr = [0; $cap];
                self.len = 0;
            }

            #[doc = "Returns a byte slice of this `" [<$name $bit>]"`’s contents."]
            ///
            // /// The inverse of this method is [`from_utf8`][Self::from_utf8].
            pub fn as_bytes(&self) -> &[u8] {
                #[cfg(not(feature = "safe"))]
                unsafe { self.arr.get_unchecked(0..self.len as usize) }

                #[cfg(feature = "safe")]
                self.arr.get(0..self.len as usize).expect("len must be <= arr.len()")
            }

            #[doc = "Returns a byte slice of this `" [<$name $bit>]"`’s contents."]
            pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                #[cfg(not(feature = "safe"))]
                unsafe { self.arr.get_unchecked_mut(0..self.len as usize) }

                #[cfg(feature = "safe")]
                self.arr.get_mut(0..self.len as usize).expect("len must be <= arr.len()")
            }

            #[doc = "Returns the inner array of this `" [<$name $bit>]"`’s."]
            ///
            /// The array contains all the bytes, including those outside the current length.
            pub fn into_array(self) -> [u8; $cap] {
                self.arr
            }

            #[doc = "Extracts a string slice containing the entire `"[<$name $bit>]"`."]
            pub fn as_str(&self) -> &str {
                #[cfg(not(feature = "safe"))]
                unsafe {
                    core::str::from_utf8_unchecked(self.arr.get(0..self.len as usize)
                        .expect("len must be <= arr.len()")
                    )
                }
                #[cfg(feature = "safe")]
                core::str::from_utf8(self.arr.get(0..self.len as usize)
                    .expect("len must be <= arr.len()")
                ).expect("must be valid utf-8")
            }

            /// Converts this string into a mutable string slice.
            ///
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            pub fn as_mut_str(&mut self) -> &mut str {
                unsafe { &mut *(self.as_bytes_mut() as *mut [u8] as *mut str) }
            }

            /// Removes the last character and returns it, or `None` if
            /// the string is empty.
            pub fn pop(&mut self) -> Option<char> {
                self.as_str().chars().last().map(|c| {
                    self.len -= c.len_utf8() as u8;
                    c
                })
            }

            #[doc = "Appends the given char to the end of this `" [<$name $bit>] "`."]
            ///
            /// # Panics
            /// Panics if the capacity is not enough to hold the `character`.
            pub fn push(&mut self, character: char) {
                let char_len = character.len_utf8();
                if self.remaining_capacity() >= char_len {
                    let beg = self.len as usize;
                    let end = beg + char_len;
                    let _ = character.encode_utf8(&mut self.arr[beg..end]);
                    self.len += char_len as u8;
                } else {
                    panic!["not enough capacity"];
                }
            }

            #[doc = "Tries to append the given char to the end of this `"[<$name $bit>]"`."]
            ///
            /// # Errors
            /// Errors if the capacity is not enough to hold the `character`.
            pub fn try_push(&mut self, character: char) -> Result<()> {
                let char_len = character.len_utf8();
                if self.remaining_capacity() >= char_len {
                    let beg = self.len as usize;
                    let end = beg + char_len;
                    let _ = character.encode_utf8(&mut self.arr[beg..end]);
                    self.len += char_len as u8;
                    Ok(())
                } else {
                    Err(Error::NotEnoughCapacity)
                }
            }

        }

        /* traits */

        impl Default for [<$name $bit>] {
            fn default() -> Self {
                Self {
                    arr: [0; $cap],
                    len: 0,
                }
            }
        }

        impl fmt::Display for [<$name $bit>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl Deref for [<$name $bit>] {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }
    }};
}
impl_string![String:
    "A", "", 16, 1; "A", "s", 32, 3; "A", "s", 64, 7; "A", "s", 128, 15];

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
