// textos::string::chars::core_impls

use super::{Char16, Char24, Char32, Char8};
use crate::error::{TextosError, TextosResult as Result};
use core::fmt;
use devela::paste;

/* Default, Display, Debug */

macro_rules! core_impls {
    ($name:ident: $( $bits:literal + $default:expr ),+ ) => {
        $( core_impls![@$name: $bits + $default]; )+
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
        impl fmt::Binary for [<$name $bits>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Binary::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::LowerHex for [<$name $bits>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::UpperHex for [<$name $bits>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::UpperHex::fmt(&self.to_u32(), f)
            }
        }
        impl fmt::Octal for [<$name $bits>] {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Octal::fmt(&self.to_u32(), f)
            }
        }
    }};
}
core_impls![Char: 8+0, 16+0, 24+[0,0,0], 32+'\x00'];

/* From Char8 */

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
impl From<Char8> for Char32 {
    #[inline]
    fn from(c: Char8) -> Char32 {
        Char32(c.0.into())
    }
}
impl From<Char8> for char {
    #[inline]
    fn from(c: Char8) -> char {
        c.0.into()
    }
}

/* From Char16 */

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
impl From<Char16> for Char32 {
    #[inline]
    fn from(c: Char16) -> Char32 {
        #[cfg(feature = "safe")]
        return Char32(char::from_u32(c.0 as u32).unwrap());

        // SAFETY: we've already checked we contain a valid char.
        #[cfg(not(feature = "safe"))]
        return unsafe { Char32(char::from_u32_unchecked(c.0 as u32)) };
    }
}
impl From<Char16> for char {
    #[inline]
    fn from(c: Char16) -> char {
        c.to_char()
    }
}

/* From Char24 */

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
impl From<Char24> for Char32 {
    #[inline]
    fn from(c: Char24) -> Char32 {
        let code_point = (c.0[0] as u32) << 16 | (c.0[1] as u32) << 8 | (c.0[2] as u32);

        #[cfg(feature = "safe")]
        return Char32(char::from_u32(code_point).unwrap());

        // SAFETY: we've already checked we contain a valid char.
        #[cfg(not(feature = "safe"))]
        return unsafe { Char32(char::from_u32_unchecked(code_point)) };
    }
}
impl From<Char24> for char {
    #[inline]
    fn from(c: Char24) -> char {
        c.to_char()
    }
}

/* From Char32 */

impl TryFrom<Char32> for Char8 {
    type Error = TextosError;
    #[inline]
    fn try_from(c: Char32) -> Result<Char8> {
        Char8::try_from_char(c.0)
    }
}
impl TryFrom<Char32> for Char16 {
    type Error = TextosError;
    #[inline]
    fn try_from(c: Char32) -> Result<Char16> {
        Char16::try_from_char(c.0)
    }
}
impl From<Char32> for Char24 {
    #[inline]
    fn from(c: Char32) -> Char24 {
        Char24::from_char(c.0)
    }
}
impl From<Char32> for char {
    #[inline]
    fn from(c: Char32) -> char {
        c.0
    }
}
