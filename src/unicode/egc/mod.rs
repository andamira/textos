// textos::unicode::egc
//
//! Extended grapheme cluster.
//

use crate::macros::impl_sized_alias;

mod non_nul;
#[cfg(feature = "alloc")]
mod string;
mod u8string;

#[cfg(feature = "alloc")]
pub use string::Egc;
pub use {non_nul::StaticNonNulEgc, u8string::StaticU8Egc};

/// Common trait for extended grapheme cluster types.
pub trait Egcs {}

/* type aliases */

impl_sized_alias![
    NonNulEgc, StaticNonNulEgc,
    "extended grapheme cluster, with a fixed capacity of ",
    ", that can't contain nul characters.":
    "An" 8, 1 "";
    "A" 16, 2 "s";
    "A" 24, 3 "s";
    "A" 32, 4 "s";
    "A" 64, 8 "s";
    "A" 128, 16 "s"
];
impl_sized_alias![
    Egc, StaticU8Egc,
    "extended grapheme cluster, with a fixed capacity of ", ".":
    "A" 16, 1 "";
    "A" 24, 2 "s";
    "A" 32, 3 "s";
    "A" 64, 7 "s";
    "A" 128, 15 "s"
];
