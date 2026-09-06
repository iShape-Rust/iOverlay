//! Integer types supported by the overlay engine.
//!
//! # Coordinate range
//!
//! For an `N`-bit engine, keep **every x and y coordinate**, across all inputs to
//! an operation, in the inclusive range `-2^(N - 2)..=2^(N - 2) - 1`:
//!
//! | Engine | Minimum | Maximum |
//! | --- | ---: | ---: |
//! | `i16` | -16,384 | 16,383 |
//! | `i32` | -1,073,741,824 | 1,073,741,823 |
//! | `i64` | -4,611,686,018,427,387,904 | 4,611,686,018,427,387,903 |
//!
//! The maximum coordinate difference is `D = I::MAX`; sums of two products
//! fit in `I::Wide` because `2 * D^2 < 2^(2*N - 1)`. Intersection numerators
//! use extended-width products. Area accumulation must allow partial sums to
//! wrap even though the final contour area fits. Integer input bounds are not
//! checked at runtime.
//!
//! ## Floating-point conversion
//!
//! These limits concern the integer coordinates after conversion, not the
//! original floating-point coordinates. For an explicit conservative bound,
//! use [`FloatPointAdapter::with_coordinate_bits`](i_float::adapter::FloatPointAdapter::with_coordinate_bits)
//! with `coordinate_bits = I::BITS - 3`. This bounds the converted magnitude by
//! `2^(N - 3)` (8,192 for `i16`), with both endpoints included. A custom unchecked
//! scale must respect the integer range too.

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
/// See the [coordinate range](self) required by the integer APIs.
pub trait OverlayInt: private::OverlayIntSealed {}

impl OverlayInt for i16 {}
impl OverlayInt for i32 {}
impl OverlayInt for i64 {}
