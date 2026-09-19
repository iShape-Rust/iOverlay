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
//! original floating-point coordinates, which have their own [limits](crate::float).
//! Automatic float APIs and checked fixed-scale APIs use the adapter's
//! conservative coordinate budget, `CONSERVATIVE_COORDINATE_BITS = I::BITS - 3`.
//! This reserves an extra bit for rounding inside the arithmetic range.
//! For a custom adapter with the same budget, use
//! [`FloatPointAdapter::new_conservative`](i_float::adapter::FloatPointAdapter::new_conservative).
//! Converted magnitudes are bounded by `2^(N - 3)` (8,192 for `i16`), including
//! both endpoints. A custom unchecked scale must respect the integer range too.

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
