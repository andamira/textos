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

/// An 8-bit unicode scalar value, a subset of [`char`].
///
/// See also: [`Char16`], [`Char24`], [`Char32`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char8(u8);

/// A 16-bit unicode scalar value, a subset of [`char`].
///
/// See also: [`Char8`], [`Char24`], [`Char32`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char16(u16);

/// A 24-bit unicode scalar value, a subset of [`char`].
///
/// See also: [`Char8`], [`Char16`], [`Char32`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char24([u8; 3]);

/// A 32-bit unicode scalar value.
///
/// See also: [`Char8`], [`Char16`], [`Char24`].
pub type Char32 = char;

/// Common trait for all character types.
pub trait Chars: Strings {
    /// Returns the number of bytes needed to encode in UTF-8.
    fn len_utf8(self) -> usize;

    /// Returns the number of bytes needed to encode in UTF-16.
    fn len_utf16(self) -> usize;

    /* encode */

    /// Encodes this character as UTF-8 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded character.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length four is large enough to encode any char.
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str;

    /// Encodes this character as UTF-16 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded character.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length 2 is large enough to encode any char.
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];

    /* escape */

    /* is_ */
}
