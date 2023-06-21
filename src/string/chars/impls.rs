// textos::string::chars::impls

use super::{Char16, Char24, Char32, Char8, Chars, Strings};
use crate::error::{TextosError, TextosResult as Result};
use devela::paste;

/* common implementations */

macro_rules! impls {
    ($name:ident: $( $bits:literal ),+ ) => {
        $( impls![@$name: $bits]; )+
    };
    (@$name:ident: $bits:literal) => { paste! {

        impl Strings for [<$name $bits>] {}

        impl Chars for [<$name $bits>] {
            const MAX: Self = Self::MAX;

            #[inline]
            fn len_utf8(self) -> usize { self.len_utf8() }

            #[inline]
            fn len_utf16(self) -> usize { self.len_utf16() }

            #[inline]
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
                self.to_char32().encode_utf8(dst)
            }
            #[inline]
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
                self.to_char32().encode_utf16(dst)
            }

            #[inline]
            fn is_noncharacter(self) -> bool { self.is_noncharacter() }
        }

        // const implementations directly over the type
        impl [<$name $bits>] {

            /// Returns the number of bytes needed to encode in UTF-8.
            #[inline]
            pub const fn len_utf8(self) -> usize { self.to_char32().len_utf8() }

            /// Returns the number of bytes needed to encode in UTF-16.
            #[inline]
            pub const fn len_utf16(self) -> usize { self.to_char32().len_utf16() }
        }
    }};
}
impls![Char: 8, 16, 24];

/* separate implementations */

impl Char8 {
    /* constants */

    /// The highest unicode scalar a `Char8` can represent, `'\u{FF}'`.
    pub const MAX: Char8 = Char8(0xFF);

    /* conversions */

    /// Converts this `Char8` to a `Char32`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char32(self) -> Char32 {
        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(self.0 as u32) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(self.0 as u32) };
    }

    /// Tries to convert a `Char32` to a `Char8`.
    #[inline]
    pub const fn try_from_char32(c: Char32) -> Result<Char8> {
        if c as u32 <= Self::MAX.0 as u32 {
            Ok(Char8(c as u32 as u8))
        } else {
            Err(TextosError::OutOfBounds)
        }
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        is_noncharacter(self.0 as u32)
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }
}

impl Char16 {
    /* constants */

    /// The highest unicode scalar a `Char16` can represent, `'\u{FFFF}'`.
    ///
    /// Note that '\u{FFFF}' is a *noncharacter*.
    pub const MAX: Char16 = Char16(0xFFFF);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char16 = Char16(char::REPLACEMENT_CHARACTER as u32 as u16);

    /* conversions */

    /// Converts this `Char16` to a `Char32`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char32(self) -> Char32 {
        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(self.0 as u32) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(self.0 as u32) };
    }

    /// Tries to convert a `Char32` to a `Char16`.
    #[inline]
    pub const fn try_from_char32(c: Char32) -> Result<Char16> {
        if c as u32 <= Self::MAX.0 as u32 {
            Ok(Char16(c as u32 as u16))
        } else {
            Err(TextosError::OutOfBounds)
        }
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        is_noncharacter(self.0 as u32)
    }
    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }
}

impl Char24 {
    /* constants */

    /// The highest unicode scalar a `Char24` can represent, `'\u{10FFFF}'`.
    pub const MAX: Char24 = Char24::from_char32('\u{10ffff}');

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char24 = Char24::from_char32(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts this `Char24` to a `Char32`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char32(self) -> char {
        let code_point = (self.0[0] as u32) << 16 | (self.0[1] as u32) << 8 | (self.0[2] as u32);

        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(code_point) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(code_point) };
    }

    /// Converts a `Char32` to a `Char24`.
    #[inline]
    pub const fn from_char32(c: Char32) -> Char24 {
        let b0 = ((c as u32 & 0x00FF0000) >> 16) as u8;
        let b1 = ((c as u32 & 0x0000FF00) >> 8) as u8;
        let b2 = (c as u32 & 0x000000FF) as u8;
        Char24([b0, b1, b2])
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        let code = (self.0[0] as u32) << 16 | (self.0[1] as u32) << 8 | (self.0[2] as u32);
        is_noncharacter(code)
    }
    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }
}

impl Strings for Char32 {}

impl Chars for Char32 {
    const MAX: Self = Self::MAX;

    #[inline]
    fn len_utf8(self) -> usize {
        self.len_utf8()
    }

    #[inline]
    fn len_utf16(self) -> usize {
        self.len_utf16()
    }

    #[inline]
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
        self.encode_utf8(dst)
    }
    #[inline]
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
        self.encode_utf16(dst)
    }

    #[inline]
    fn is_noncharacter(self) -> bool {
        is_noncharacter(self as u32)
    }
}

//
#[inline]
const fn is_noncharacter(code: u32) -> bool {
    // sub-block of 32 non-characters:
    (code >= 0xFDD0 && code <= 0xFDEF)
        // 2× non-characters at the end of each plane:
        || (code >= 0xFFFE && (code & 0xFF) == 0xFE)
        || (code >= 0xFFFE && (code & 0xFF) == 0xFF)
        // unallocated range (16 potential non-characters):
        || (code >= 0x2FE0 && code <= 0x2FEF)
    // surrogates (FDDO-FDEF) are already filtered out in `char`.
}
