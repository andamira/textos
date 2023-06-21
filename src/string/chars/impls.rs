// textos::string::chars::impls

use super::{Char16, Char24, Char32, Char8, Chars, Strings};
use devela::paste;

macro_rules! impls {
    ($name:ident: $( $bits:literal ),+ ) => {
        $( impls![@$name: $bits]; )+
    };
    (@$name:ident: $bits:literal) => { paste! {

        impl Strings for [<$name $bits>] {}

        impl Chars for [<$name $bits>] {
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
        }

        // const implementations directly over the type
        impl [<$name $bits>] {
            /// Returns the number of bytes needed to encode in UTF-8.
            #[inline]
            pub const fn len_utf8(self) -> usize { self.to_char().len_utf8() }

            /// Returns the number of bytes needed to encode in UTF-16.
            #[inline]
            pub const fn len_utf16(self) -> usize { self.to_char().len_utf16() }
        }
    }};
}
impls![Char: 8, 16, 24];

impl Char8 {
    /// Converts this `Char8` to a `char`.
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
}

impl Char16 {
    /// Converts this `Char16` to a `char`.
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
}

impl Char24 {
    /// Converts this `Char24` to a `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        let code_point = (self.0[0] as u32) << 16 | (self.0[1] as u32) << 8 | (self.0[2] as u32);

        // #[cfg(feature = "safe")]
        if let Some(c) = char::from_u32(code_point) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(not(feature = "safe"))]
        // return unsafe { char::from_u32_unchecked(code_point) };
    }
}
