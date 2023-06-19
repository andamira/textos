// textas::string
//
//! Strings goodies.
//

mod counter;
mod non_nul;
mod string_u8;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{non_nul::*, string_u8::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::counter::counter_string;
}
