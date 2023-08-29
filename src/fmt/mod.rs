// textos::fmt
//
//! Formatting.
//

mod indent;

pub use all::*;
pub(super) mod all {
    #[doc(inline)]
    pub use super::indent::*;
}
