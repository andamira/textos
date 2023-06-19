// textas::string
//
//! Strings goodies.
//

mod counter;
mod non_nul;
mod sized;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{non_nul::*, sized::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::counter::counter_string;
}
