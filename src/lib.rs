// textos
//!
//

#![warn(clippy::all)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

pub mod error;

mod ascii;
pub mod string;
pub mod unicode;

/// Everything is directly available here.
pub mod all {
    #[doc(inline)]
    pub use super::{ascii::*, error::*, string::all::*, unicode::all::*};
}
