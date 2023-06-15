// textos::fmt
//
//! Formatting.
//

mod indent;
#[cfg(not(feature = "safe"))]
mod int_buf;

pub use all::*;
pub(super) mod all {
    #[doc(inline)]
    pub use super::indent::indent;

    #[doc(inline)]
    #[cfg(not(feature = "safe"))]
    pub use super::int_buf::{IntBuf, IntBufAble};
}
