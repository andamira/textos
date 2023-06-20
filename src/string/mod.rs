// textas::string
//
//! Strings goodies.
//

mod chars;
mod counter;
mod non_nul;
mod u8string;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{chars::*, non_nul::*, u8string::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::counter::counter_string;
}
