// textos::string::chars
//
//! Unicode scalar types.
//

use super::Strings;
use devela::NonSpecificU8;

mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

pub(crate) type NonMaxU8 = NonSpecificU8<{ u8::MAX }>;

/* definitions */

/// A 7-bit [unicode scalar value][scalar] (ASCII).
///
/// It can represent every scalar from the [basic latin][0w] subset of the plane 0.
///
/// `Option<Char7>` is the same size as `Char7`.
///
/// See also: [`Char8`], [`Char16`], [`Char24`], [`Char32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char7(NonMaxU8);

/// An 8-bit [unicode scalar value][scalar].
///
/// It can represent every scalar from the [basic latin][0w] and
/// [latin-1 supplement][1w] subsets of the plane 0.
///
/// See also: [`Char7`], [`Char16`], [`Char24`], [`Char32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
/// [1w]: https://en.wikipedia.org/wiki/Latin-1_Supplement
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char8(u8);

/// A 16-bit [unicode scalar value][scalar].
///
/// It can represent every scalar from the [Basic Multilingual Plane][0w] (BMP),
/// the first and most important plane in the Unicode standard (also known as
/// plane 0), containing nearly all commonly used writing systems and symbols.
///
/// See also: [`Char7`], [`Char8`], [`Char24`], [`Char32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Plane_(Unicode)#Basic_Multilingual_Plane
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char16(u16);

/// A 24-bit [unicode scalar value][scalar].
///
/// It can represent each and every scalar the same as [`Char32`],
/// since the maximum value (`\u{10FFFF}`) needs only 21 bits.
///
/// See also: [`Char7`], [`Char8`], [`Char16`], [`Char32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char24 {
    hi: u8, // highest byte
    mi: u8, // middle byte
    lo: u8, // lowest byte
}

/// A 32-bit [unicode scalar value][scalar].
///
/// This is the default unicode scalar type in Rust. It can represent the same
/// range of unicode scalars as [`Char24`].
///
/// See also: [`Char7`], [`Char8`], [`Char16`], [`Char24`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char32(pub char);

/// Common trait for unicode scalars.
pub trait Chars: Strings {
    /// The highest unicode scalar that can be represented by this type.
    const MAX: Self;

    /* encode */

    /// Returns the number of bytes needed to represent the scalar value.
    fn byte_len(self) -> usize;

    /// Returns the number of bytes needed to encode in UTF-8.
    fn len_utf8(self) -> usize;

    /// Returns the number of bytes needed to encode in UTF-16.
    fn len_utf16(self) -> usize;

    /// Encodes this scalar as UTF-8 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length four is large enough to encode any char.
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str;

    /// Encodes this scalar as UTF-16 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length 2 is large enough to encode any char.
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];

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
    fn to_digit(self, radix: u32) -> Option<u32>;

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    fn to_ascii_uppercase(self) -> Self
    where
        Self: Sized;

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    fn to_ascii_lowercase(self) -> Self
    where
        Self: Sized;

    /* escape */

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    fn is_noncharacter(self) -> bool;

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    fn is_character(self) -> bool
    where
        Self: Sized,
    {
        !self.is_noncharacter()
    }

    /// Checks if the unicode scalar is a digit in the given radix.
    ///
    /// See also [`to_digit`][Self#method.to_digit].
    fn is_digit(self, radix: u32) -> bool;

    /// Returns `true` if this unicode scalar has the general category for
    /// control codes.
    fn is_control(self) -> bool;

    /// Returns `true` if this unicode scalar has the `Alphabetic` property.
    fn is_alphabetic(self) -> bool;

    /// Returns `true` if this unicode scalar has one of the general categories
    /// for numbers.
    ///
    /// If you want to parse ASCII decimal digits (0-9) or ASCII base-N,
    /// use [`is_ascii_digit`][Self#method.is_ascii_digit] or
    /// [`is_digit`][Self#method.is_digit] instead.
    fn is_numeric(self) -> bool;

    /// Returns `true` if this unicode scalar satisfies either
    /// [`is_alphabetic()`][Self#method.is_alphabetic] or
    /// [`is_numeric()`][Self#method.is_numeric].
    fn is_alphanumeric(self) -> bool;

    /// Returns `true` if this unicode scalar has the `Lowercase` property.
    fn is_lowercase(self) -> bool;

    /// Returns `true` if this unicode scalar has the `Lowercase` property.
    fn is_uppercase(self) -> bool;

    /// Returns `true` if this unicode scalar has the `White_Space` property.
    fn is_whitespace(self) -> bool;

    /* ascii */

    /// Checks if the value is within the ASCII range.
    fn is_ascii(self) -> bool;
}
