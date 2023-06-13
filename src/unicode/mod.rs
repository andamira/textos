// textos::unicode
//
//! Unicode goodies.
//
// - https://www.unicode.org/charts/

/* re-exports */

/// Re-export of the `unicode_blocks` crate.
///
#[doc(inline)]
pub use ::unicode_blocks;

#[doc(inline)]
pub use ::unicode_blocks::find_unicode_block;

pub mod drawing;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{drawing::*, find_unicode_block};
}
