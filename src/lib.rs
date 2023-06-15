// textos
//!
//

#![warn(clippy::all)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

// features safeguarding
#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;

mod ascii;
pub mod fmt;
pub mod string;
pub mod unicode;

/// Everything is directly available here.
pub mod all {
    #[doc(inline)]
    pub use super::{ascii::*, error::*, fmt::*, string::all::*, unicode::all::*};
}
