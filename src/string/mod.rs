// textas::string
//
//! Strings goodies.
//

mod array;
mod counter;

pub use all::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::array::*;

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::counter::counter_string;
}
