// textos::unicode::egc
//
//! Extended grapheme cluster.
//

mod non_nul;
#[cfg(feature = "alloc")]
mod string;
mod u8string;

/// Common trait for extended grapheme cluster types.
pub trait Egc {}

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{non_nul::*, u8string::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::string::*;
}
