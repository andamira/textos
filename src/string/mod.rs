// textas::string
//
//! Strings goodies.
//

mod array;

pub use array::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::array::*;
}
