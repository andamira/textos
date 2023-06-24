// textos::unicode
//
//! Unicode-related types and functionality.
//
// - https://www.unicode.org/charts/

/* re-exports */

/// Re-export of the `unicode_blocks` crate.
///
#[doc(inline)]
pub use ::unicode_blocks;

#[doc(inline)]
pub use ::unicode_blocks::find_unicode_block;

pub mod char;
pub mod drawing;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        char::{Char16, Char24, Char32, Char7, Char8, Chars},
        drawing::*,
        find_unicode_block,
    };
}
