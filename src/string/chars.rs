// textos::string::chars
//
//!
//

/// An 8-bit unicode scalar value, a subset of [`char`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char8(u8);

/// A 16-bit unicode scalar value, a subset of [`char`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char16(u16);

/// A 24-bit unicode scalar value, a subset of [`char`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char24([u8; 3]);

impl Char8 {
    /// Converts a `Char8` to a `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(&self) -> char {
        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(self.0 as u32) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(self.0 as u32) };
    }
}
impl Char16 {
    /// Converts a `Char16` to a `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(&self) -> char {
        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(self.0 as u32) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(self.0 as u32) };
    }
}
impl Char24 {
    /// Converts a `Char24` to a `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(&self) -> char {
        let code_point = (self.0[0] as u32) << 16 | (self.0[1] as u32) << 8 | (self.0[2] as u32);

        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(code_point) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(code_point) };
    }
}

mod core_impls {
    use super::{Char16, Char24, Char8};
    use crate::error::{TextosError, TextosResult as Result};
    use core::fmt;
    use devela::paste;

    macro_rules! common_impls {
        ($name:ident: $( $bits:literal + $default:expr ),+ ) => {
            $( common_impls![@$name: $bits + $default]; )+
        };
        (@$name:ident: $bits:literal + $default:expr) => { paste! {
            /// Returns the default value of `\x00` (nul character).
            impl Default for [<$name $bits>] {
                #[inline]
                fn default() -> Self { Self($default) }
            }
            impl fmt::Display for [<$name $bits>] {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.to_char())
                }
            }
            impl fmt::Debug for [<$name $bits>] {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{:?}", self.to_char())
                }
            }
        }};
    }
    common_impls![Char: 8+0, 16+0, 24+[0,0,0]];

    /* from Char8 */

    impl From<Char8> for Char16 {
        #[inline]
        fn from(c: Char8) -> Char16 {
            Char16(c.0.into())
        }
    }
    impl From<Char8> for Char24 {
        #[inline]
        fn from(c: Char8) -> Char24 {
            Char24([c.0, 0, 0])
        }
    }
    impl From<Char8> for char {
        #[inline]
        fn from(c: Char8) -> char {
            c.0.into()
        }
    }

    /* from Char16 */

    impl TryFrom<Char16> for Char8 {
        type Error = TextosError;
        #[inline]
        fn try_from(c: Char16) -> Result<Char8> {
            if c.0 <= u8::MAX as u16 {
                Ok(Char8(c.0 as u8))
            } else {
                Err(TextosError::OutOfBounds)
            }
        }
    }
    impl From<Char16> for Char24 {
        #[inline]
        fn from(c: Char16) -> Char24 {
            let (b0, b1) = (((c.0 & 0xFF00) >> 8) as u8, (c.0 & 0x00FF) as u8);
            Char24([b0, b1, 0])
        }
    }
    impl From<Char16> for char {
        #[inline]
        fn from(c: Char16) -> char {
            #[cfg(feature = "safe")]
            return char::from_u32(c.0 as u32).unwrap();

            // SAFETY: we've already checked we contain a valid char.
            #[cfg(not(feature = "safe"))]
            return unsafe { char::from_u32_unchecked(c.0 as u32) };
        }
    }

    /* from Char24 */

    impl TryFrom<Char24> for Char8 {
        type Error = TextosError;
        #[inline]
        fn try_from(c: Char24) -> Result<Char8> {
            let code_point = (c.0[0] as u32) << 16 | (c.0[1] as u32) << 8 | (c.0[2] as u32);
            if code_point <= u8::MAX as u32 {
                Ok(Char8(code_point as u8))
            } else {
                Err(TextosError::OutOfBounds)
            }
        }
    }
    impl TryFrom<Char24> for Char16 {
        type Error = TextosError;
        #[inline]
        fn try_from(c: Char24) -> Result<Char16> {
            let code_point = (c.0[0] as u32) << 16 | (c.0[1] as u32) << 8 | (c.0[2] as u32);
            if code_point <= u16::MAX as u32 {
                Ok(Char16(code_point as u16))
            } else {
                Err(TextosError::OutOfBounds)
            }
        }
    }
    impl From<Char24> for char {
        #[inline]
        fn from(c: Char24) -> char {
            let code_point = (c.0[0] as u32) << 16 | (c.0[1] as u32) << 8 | (c.0[2] as u32);

            #[cfg(feature = "safe")]
            return char::from_u32(code_point).unwrap();

            // SAFETY: we've already checked we contain a valid char.
            #[cfg(not(feature = "safe"))]
            return unsafe { char::from_u32_unchecked(code_point) };
        }
    }

    /* from char */

    impl TryFrom<char> for Char8 {
        type Error = TextosError;
        #[inline]
        fn try_from(c: char) -> Result<Char8> {
            if c.len_utf8() == 1 {
                Ok(Char8(c as u8))
            } else {
                Err(TextosError::NotEnoughCapacity(c.len_utf8()))
            }
        }
    }
    impl TryFrom<char> for Char16 {
        type Error = TextosError;
        #[inline]
        fn try_from(c: char) -> Result<Char16> {
            if c.len_utf8() <= 2 {
                Ok(Char16(c as u16))
            } else {
                Err(TextosError::NotEnoughCapacity(c.len_utf8()))
            }
        }
    }
    impl TryFrom<char> for Char24 {
        type Error = TextosError;
        #[inline]
        fn try_from(c: char) -> Result<Char24> {
            if c.len_utf8() <= 3 {
                let mut bytes = [0; 3];
                c.encode_utf8(&mut bytes);
                Ok(Char24(bytes))
            } else {
                Err(TextosError::NotEnoughCapacity(4))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_conversions() {
        let a32 = 'a';
        let a8 = Char8::try_from(a32).unwrap();
        let a16 = Char16::try_from(a32).unwrap();
        let a24 = Char24::try_from(a32).unwrap();

        assert_eq![a16, a8.into()];
        assert_eq![a24, a8.into()];
    }
}
