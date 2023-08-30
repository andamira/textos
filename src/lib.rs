// textos
//!
//

// warnings
#![warn(clippy::all)]
#![allow(clippy::wrong_self_convention)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");
// deprecated
deprecate_feature![old: "no-std", new: "no_std", since: "0.0.3"];

pub mod error;

mod ascii;
pub mod fmt;
pub(crate) mod macros;
pub mod unicode;

/// Everything is directly available here.
pub mod all {
    #[doc(inline)]
    #[allow(unused_imports)] // for no_std
    pub use super::{ascii::*, error::*, fmt::*, unicode::all::*};
}
