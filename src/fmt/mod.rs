// textos::fmt
//
//! Formatting.
//

mod indent;

pub use all::*;
pub(super) mod all {
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::indent::indent;
}
