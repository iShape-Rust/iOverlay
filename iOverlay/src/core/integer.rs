//! Integer types supported by the overlay engine.

use i_float::int::number::int::IntNumber;
use i_key_sort::sort::key::SortKey;
use i_tree::{Expiration, LayoutNumber};

mod private {
    use super::{Expiration, IntNumber, LayoutNumber, SortKey};

    pub trait OverlayIntSealed: IntNumber + Expiration + LayoutNumber + SortKey {}

    impl OverlayIntSealed for i16 {}
    impl OverlayIntSealed for i32 {}
    impl OverlayIntSealed for i64 {}
}

/// An integer type supported by the overlay engine.
///
/// This trait is sealed. The supported integer engines are [`i16`], [`i32`],
/// and [`i64`].
pub trait OverlayInt: private::OverlayIntSealed {}

impl OverlayInt for i16 {}
impl OverlayInt for i32 {}
impl OverlayInt for i64 {}
