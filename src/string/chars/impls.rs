// textos::string::chars::impls
//
//!
//
// TOC
// - common implementations
//   - traits
//   - const fns
// - separate implementations
//   - Char8
//   - Char16
//   - Char24
//   - Char32
//   - traits for char
// - helper fns

use super::{Char16, Char24, Char32, Char8, Chars, Strings};
use crate::error::{TextosError, TextosResult as Result};
use devela::paste;

/* common implementations */

macro_rules! impls {
    ($name:ident: $( $bits:literal ),+ ) => {
        $( impls![@$name: $bits]; )+
    };
    (@$name:ident: $bits:literal) => { paste! {

        /* impl traits */

        impl Strings for [<$name $bits>] {}

        impl Chars for [<$name $bits>] {
            const MAX: Self = Self::MAX;

            /* encode */

            #[inline]
            fn len_utf8(self) -> usize { self.len_utf8() }
            #[inline]
            fn len_utf16(self) -> usize { self.len_utf16() }
            #[inline]
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
                self.to_char().encode_utf8(dst)
            }
            #[inline]
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
                self.to_char().encode_utf16(dst)
            }
            #[inline]
            fn to_digit(self, radix: u32) -> Option<u32> { self.to_digit(radix) }
            #[inline]
            fn to_ascii_uppercase(self) -> Self { self.to_ascii_uppercase() }
            #[inline]
            fn to_ascii_lowercase(self) -> Self { self.to_ascii_lowercase() }

            /* queries */

            #[inline]
            fn is_noncharacter(self) -> bool { self.is_noncharacter() }
            #[inline]
            fn is_digit(self, radix: u32) -> bool { self.is_digit(radix) }
            //
            #[inline]
            fn is_control(self) -> bool { self.to_char().is_control() }
            #[inline]
            fn is_alphabetic(self) -> bool { self.to_char().is_alphabetic() }
            #[inline]
            fn is_numeric(self) -> bool { self.to_char().is_numeric() }
            #[inline]
            fn is_alphanumeric(self) -> bool { self.to_char().is_alphanumeric() }
            #[inline]
            fn is_lowercase(self) -> bool { self.to_char().is_lowercase() }
            #[inline]
            fn is_uppercase(self) -> bool { self.to_char().is_uppercase() }
            #[inline]
            fn is_whitespace(self) -> bool { self.to_char().is_whitespace() }
            //
            #[inline]
            fn is_ascii(self) -> bool { self.is_ascii() }
        }

        /* impl const fns */

        impl [<$name $bits>] {

            /* encode */

            /// Returns the number of bytes needed to encode in UTF-8.
            #[inline]
            pub const fn len_utf8(self) -> usize { self.to_char().len_utf8() }

            /// Returns the number of bytes needed to encode in UTF-16.
            #[inline]
            pub const fn len_utf16(self) -> usize { self.to_char().len_utf16() }

            /// Converts the scalar to a digit in the given radix.
            ///
            /// ‘Digit’ is defined to be only the following characters:
            /// `0-9`, `a-z`, `A-Z`.
            ///
            /// # Errors
            /// Returns None if the char does not refer to a digit in the given radix.
            ///
            /// # Panics
            /// Panics if given a radix larger than 36.
            pub const fn to_digit(self, radix: u32) -> Option<u32> {
                self.to_char().to_digit(radix)
            }

            /* queries */

            /// Checks if the unicode scalar is a digit in the given radix.
            ///
            /// See also [`to_digit`][Self#method.to_digit].
            pub const fn is_digit(self, radix: u32) -> bool {
                if let Some(_) = self.to_digit(radix) { true } else { false }
            }
        }
    }};
}
impls![Char: 8, 16, 24, 32];

/* separate implementations */

impl Char8 {
    /* constants */

    /// The highest unicode scalar a `Char8` can represent, `'\u{FF}'`.
    pub const MAX: Char8 = Char8(0xFF);

    /* conversions */

    /// Converts this `Char8` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `Char8` to `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        self.0 as char
    }

    /// Tries to convert a `Char32` to a `Char8`.
    #[inline]
    pub const fn try_from_char(c: char) -> Result<Char8> {
        if c as u32 <= Self::MAX.0 as u32 {
            Ok(Char8(c as u32 as u8))
        } else {
            Err(TextosError::OutOfBounds)
        }
    }
    const fn from_char_unchecked(c: char) -> Char8 {
        Char8(c as u32 as u8)
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

    /// Checks if the value is within the ASCII range.
    #[inline]
    pub const fn is_ascii(self) -> bool {
        self.0 <= 0x7F
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char8 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char8 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}

impl Char16 {
    /* constants */

    /// The highest unicode scalar a `Char16` can represent, `'\u{FFFF}'`.
    ///
    /// Note that `'\u{FFFF}'` is a *noncharacter*.
    pub const MAX: Char16 = Char16(0xFFFF);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char16 = Char16(char::REPLACEMENT_CHARACTER as u32 as u16);

    /* conversions */

    /// Converts this `Char16` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `Char16` to `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(self.0 as u32) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(self.0 as u32) };
    }

    /// Tries to convert a `Char32` to a `Char16`.
    #[inline]
    pub const fn try_from_char(c: char) -> Result<Char16> {
        if c as u32 <= Self::MAX.0 as u32 {
            Ok(Char16(c as u32 as u16))
        } else {
            Err(TextosError::OutOfBounds)
        }
    }
    const fn from_char_unchecked(c: char) -> Char16 {
        Char16(c as u32 as u16)
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char16 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char16 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
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

    /// Checks if the value is within the ASCII range.
    #[inline]
    pub const fn is_ascii(self) -> bool {
        self.0 <= 0x7F
    }
}

impl Char24 {
    /* constants */

    /// The highest unicode scalar a `Char24` can represent, `'\u{10FFFF}'`.
    pub const MAX: Char24 = Char24::from_char('\u{10ffff}');

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char24 = Char24::from_char(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts this `Char24` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        (self.0[0] as u32) << 16 | (self.0[1] as u32) << 8 | (self.0[2] as u32)
    }

    /// Converts this `Char24` to `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(self.to_u32()) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(code_point) };
    }

    /// Converts a `Char32` to a `Char24`.
    #[inline]
    pub const fn from_char(c: char) -> Char24 {
        let b0 = ((c as u32 & 0x00FF0000) >> 16) as u8;
        let b1 = ((c as u32 & 0x0000FF00) >> 8) as u8;
        let b2 = (c as u32 & 0x000000FF) as u8;
        Char24([b0, b1, b2])
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char24 {
        Self::from_char(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char24 {
        Self::from_char(char::to_ascii_lowercase(&self.to_char()))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        is_noncharacter(self.to_u32())
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }

    /// Checks if the value is within the ASCII range.
    #[inline]
    pub const fn is_ascii(self) -> bool {
        self.to_u32() <= 0x7F
    }
}

impl Char32 {
    /* constants */

    /// The highest unicode scalar a `Char32` can represent, `'\u{10FFFF}'`.
    pub const MAX: Char32 = Char32(char::MAX);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char32 = Char32(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts this `Char32` to `u32`.
    #[inline]
    pub(crate) const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `Char32` to `char`.
    #[inline]
    pub const fn to_char(self) -> char {
        self.0
    }

    /// Converts a `char` to a `Char32`.
    #[inline]
    pub const fn from_char(c: char) -> Char32 {
        Char32(c)
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char32 {
        Char32(char::to_ascii_uppercase(&self.0))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char32 {
        Char32(char::to_ascii_lowercase(&self.0))
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

    /// Checks if the value is within the ASCII range.
    #[inline]
    pub const fn is_ascii(self) -> bool {
        char::is_ascii(&self.0)
    }
}

/* traits for char */

impl Strings for char {}

impl Chars for char {
    const MAX: Self = Self::MAX;

    /* encode */

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
    fn to_digit(self, radix: u32) -> Option<u32> {
        self.to_digit(radix)
    }
    #[inline]
    fn to_ascii_uppercase(self) -> char {
        char::to_ascii_uppercase(&self)
    }
    #[inline]
    fn to_ascii_lowercase(self) -> char {
        char::to_ascii_lowercase(&self)
    }

    /* queries */

    #[inline]
    fn is_noncharacter(self) -> bool {
        is_noncharacter(self as u32)
    }
    #[inline]
    fn is_digit(self, radix: u32) -> bool {
        self.is_digit(radix)
    }
    #[inline]
    fn is_control(self) -> bool {
        self.is_control()
    }
    #[inline]
    fn is_alphabetic(self) -> bool {
        self.is_alphabetic()
    }
    #[inline]
    fn is_numeric(self) -> bool {
        self.is_numeric()
    }
    #[inline]
    fn is_alphanumeric(self) -> bool {
        self.is_alphanumeric()
    }
    #[inline]
    fn is_lowercase(self) -> bool {
        self.is_lowercase()
    }
    #[inline]
    fn is_uppercase(self) -> bool {
        self.is_uppercase()
    }
    #[inline]
    fn is_whitespace(self) -> bool {
        self.is_whitespace()
    }

    /* ascii queries*/

    #[inline]
    fn is_ascii(self) -> bool {
        (self as u32) <= 0x7F
    }
}

/* helper fns */

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
