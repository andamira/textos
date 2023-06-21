// textos::string::chars
//
//! Character types.
//

use super::Strings;

mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

/* definitions */

/// An 8-bit [unicode scalar value][scalar].
///
/// It can represent every scalar from the [basic latin][0w] and
/// [latin-1 supplement][1w] subsets of the plane 0.
///
/// See also: [`Char16`], [`Char24`], [`Char32`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
/// [1w]: https://en.wikipedia.org/wiki/Latin-1_Supplement
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char8(u8);

/// A 16-bit [unicode scalar value][scalar].
///
/// It can represent every scalar from the [Basic Multilingual Plane][0w] (BMP),
/// the first and most important plane in the Unicode standard (also known as
/// plane 0), containing nearly all commonly used writing systems and symbols.
///
/// See also: [`Char8`], [`Char24`], [`Char32`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Plane_(Unicode)#Basic_Multilingual_Plane
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char16(u16);

/// A 24-bit [unicode scalar value][scalar].
///
/// It can represent each and every scalar the same as [`Char32`],
/// since the maximum value (`\u{10FFFF}`) needs only 21 bits.
///
/// See also: [`Char8`], [`Char16`], [`Char32`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char24([u8; 3]);

/// A 32-bit [unicode scalar value][scalar].
///
/// This is the default unicode scalar type in Rust. It can represent the same
/// range of unicode scalars as [`Char24`].
///
/// See also: [`Char8`], [`Char16`], [`Char24`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
pub type Char32 = char;

/// Common trait for unicode scalars.
pub trait Chars: Strings {
    /// The highest unicode scalar that can be represented by this type.
    const MAX: Self;

    /// Returns the number of bytes needed to encode in UTF-8.
    fn len_utf8(self) -> usize;

    /// Returns the number of bytes needed to encode in UTF-16.
    fn len_utf16(self) -> usize;

    /* encode */

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
}
