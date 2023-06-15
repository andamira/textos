// textos::fmt
//
//! Formatting.
//

#[cfg(not(feature = "safe"))]
mod int_buf;

pub use all::*;
pub(super) mod all {
    #[doc(inline)]
    #[cfg(not(feature = "safe"))]
    pub use super::int_buf::{IntBuf, IntBufAble};
}
