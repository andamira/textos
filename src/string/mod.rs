// textas::string
//
//! Strings goodies.
//

mod chars;
mod counter;
mod non_nul;
mod u8string;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{chars::*, non_nul::*, u8string::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::counter::counter_string;
}

/// define type aliases for specific sizes.
macro_rules! impl_sized_alias {
    // $alias: the base name for the type alias.
    // $type: the base name of the original type.
    // $doc1: first doc text.
    // $doc2: second doc text.
    // $(
    //   $bits_det: determinant for the number of bits.
    //   $bits: number of bits.
    //   $bytes: number of bytes.
    //   $byte_plu: plural for the number of bytes.
    // )
    ($alias:ident, $type:ident, $doc1:literal, $doc2:literal:
     $($bits_det:literal $bits:literal, $bytes:literal $bytes_plu:literal);+ ) => {
        $( impl_sized_alias![@$alias, $type, $doc1, $doc2: $bits_det $bits, $bytes $bytes_plu]; )+
    };
    (@$alias:ident, $type:ident, $doc1:literal, $doc2:literal:
     $bits_det:literal $bits:literal, $bytes:literal $bytes_plu:literal) => { devela::paste! {
        #[doc = "" $bits_det " " $bits "-bit " $doc1 $bytes " byte" $bytes_plu $doc2]
        pub type [<$alias $bits>] = $type<$bytes>;
    }};
}
pub(crate) use impl_sized_alias;
