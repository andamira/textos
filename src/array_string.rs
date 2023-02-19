// textas::array_string
//
//!
//

use crate::error::{TextosError as Error, TextosResult as Result};

/// common impls for string types
macro_rules! impl_string {
    (many: $(($bit:expr, $cap:expr)),+) => {
        $( impl_string![$bit, $cap]; )+
    };
    ($bit:expr, $cap:expr) => {
        paste::paste! {
            #[doc = "A " $bit "-bit UTF-8 string, with a fixed capacity of " $cap " bytes."]
            #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub struct [<String$bit>] {
                arr: [u8; $cap],
                len: u8,
            }

            /* String methods */

            impl [<String$bit>] {
                #[doc = "Creates a new empty `" [<String$bit>]"`."]
                pub fn new() -> [<String$bit>] {
                    Self::default()
                }

                // TODO
                // #[doc = "Converts a slice of bytes to a "[<String$bit>]"."]
                // pub fn from_utf8(&self, bytes: &[u8]) -> Result<[<String$bit>]> {
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

                #[doc = "Returns the total capacity of a `" [<String$bit>]"`."]
                pub const fn capacity() -> usize {
                    $cap as usize
                }

                /// Returns the remaining capacity.
                pub const fn remaining_capacity(&self) -> usize {
                    $cap as usize - self.len as usize
                }

                /// Returns the current length.
                pub const fn len(&self) -> usize {
                    self.len as usize
                }

                /// Returns `true` if the current length is 0.
                pub const fn is_empty(&self) -> bool {
                    self.len == 0
                }

                /// Returns `true` if the current remaining capacity is 0.
                pub const fn is_full(&self) -> bool {
                    self.len == $cap
                }

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

                #[doc = "Returns a byte slice of this `" [<String$bit>]"`’s contents."]
                ///
                // /// The inverse of this method is [`from_utf8`][Self::from_utf8].
                pub fn as_bytes(&self) -> &[u8] {
                    #[cfg(not(feature = "safe"))]
                    unsafe { self.arr.get_unchecked(0..self.len as usize) }

                    #[cfg(feature = "safe")]
                    self.arr.get(0..self.len as usize).expect("len must be <= arr.len()")
                }

                #[doc = "Returns a byte slice of this `" [<String$bit>]"`’s contents."]
                pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                    #[cfg(not(feature = "safe"))]
                    unsafe { self.arr.get_unchecked_mut(0..self.len as usize) }

                    #[cfg(feature = "safe")]
                    self.arr.get_mut(0..self.len as usize).expect("len must be <= arr.len()")
                }

                #[doc = "Returns the inner array of this `" [<String$bit>]"`’s."]
                ///
                /// The array contains all the bytes, including those outside the current length.
                pub fn into_array(self) -> [u8; $cap] {
                    self.arr
                }

                #[doc = "Extracts a string slice containing the entire `"[<String$bit>]"`."]
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
                // FIX "Available on non-crate feature `safe` only."
                #[cfg_attr(feature = "nightly", doc(cfg(not(feature = "safe"))))]
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

                #[doc = "Appends the given char to the end of this `" [<String$bit>] "`."]
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

                #[doc = "Tries to append the given char to the end of this `"[<String$bit>]"`."]
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
                        // IMPROVE MAYBE return char_len, or self.len - char_len
                        Err(Error::NotEnoughCapacity)
                    }
                }

            }

            /* traits */

            impl Default for [<String$bit>] {
                fn default() -> Self {
                    Self {
                        arr: [0; $cap],
                        len: 0,
                    }
                }
            }

            #[cfg(feature = "std")]
            impl std::fmt::Display for [<String$bit>] {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.as_str())
                }
            }

            // Deref<Target=str>
            impl core::ops::Deref for [<String$bit>] {
                type Target = str;
                fn deref(&self) -> &Self::Target {
                    self.as_str()
                }
            }


        }
    };
}

impl_string![many:
    (32, 3), (64, 7), (128, 15), (256, 31), (512, 63), (1024, 127), (2048, 255)];

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn sizes() {
        assert_eq![8, size_of::<usize>()];

        assert_eq![8, size_of::<String64>()];
        assert_eq![16, size_of::<String128>()];
        assert_eq![32, size_of::<String256>()];
        assert_eq![64, size_of::<String512>()];
        assert_eq![128, size_of::<String1024>()];
        assert_eq![256, size_of::<String2048>()];
    }

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
