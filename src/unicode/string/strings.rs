// textos::string::strings
//
//! `Strings` trait.
//

// use crate::error::TextosResult as Result;
use core::fmt::Display;

/// Common trait for all string types.
///
/// It depends on the [`ToString`] trait if the `alloc` feature is enabled.
pub trait Strings: Display {}
