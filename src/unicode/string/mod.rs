// textos::unicode::string
//
//! Utf-8 strings.
//

mod counter;
mod non_nul;
mod u8string;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{non_nul::*, u8string::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::counter::counter_string;
}
