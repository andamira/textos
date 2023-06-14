// textas::string
//
//! Strings goodies.
//

mod array;
mod counter;

pub use all::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array::*, counter::counter_string};
}
