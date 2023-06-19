// textas::string
//
//! Strings goodies.
//

mod sized;
mod counter;

pub use all::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::sized::*;

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::counter::counter_string;
}
