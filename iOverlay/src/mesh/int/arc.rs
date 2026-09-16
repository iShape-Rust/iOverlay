//! Directed unit-circle arcs with reusable storage.
//!
//! Integer and floating-point implementations share traversal and angular-gap
//! guarantees, but their error budgets and intermediate directions differ.

mod float;
mod integer;

#[cfg(test)]
mod tests;

pub(crate) use float::FloatArc;
pub(crate) use integer::IntegerArc;
// Preserve the public integer builder API.
pub use integer::IntegerArc as ArcBuilder;

use i_float::int::angle::Angle;

/// Direction of traversal in Cartesian coordinates (y increases upward).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcDirection {
    Clockwise,
    Counterclockwise,
}

/// Settings for directed arcs built with integer or floating-point arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArcOptions {
    /// Maximum angular gap, including the final gap to the original endpoint.
    /// Clamped to [`Self::MIN_STEP`]..=[`Self::MAX_STEP`] on construction.
    pub max_step: Angle,
    /// Relative matrix-angle accuracy exponent: 4 allows 1/16 error, 5 allows
    /// 1/32, and 6 allows 1/64. Clamped to 4..=32; 32 selects full precision.
    /// This changes integer matrix construction cost, not the coefficient format.
    /// Ignored by the floating-point implementation.
    pub rotation_precision: u32,
}

impl ArcOptions {
    /// 1/1024 turn, or 0.3515625 degrees.
    pub const MIN_STEP: Angle = Angle::from_bits(1 << 22);
    /// 1/8 turn, or 45 degrees.
    pub const MAX_STEP: Angle = Angle::from_bits(1 << 29);

    pub(crate) fn clamped(self) -> Self {
        Self {
            max_step: Angle::from_bits(
                self.max_step
                    .bits()
                    .clamp(Self::MIN_STEP.bits(), Self::MAX_STEP.bits()),
            ),
            rotation_precision: self.rotation_precision.clamp(4, 32),
        }
    }
}

impl Default for ArcOptions {
    fn default() -> Self {
        Self {
            max_step: Self::MAX_STEP,
            rotation_precision: 5,
        }
    }
}

/// Reusable directed arcs, excluding both input endpoints.
/// Equal rays produce an empty arc; opposite rays and major arcs are supported.
/// Each implementation budgets its own numerical error to keep all angular
/// gaps, including the final gap to the endpoint, within the clamped max_step.
/// Point counts and coordinates need not match between implementations.
pub(crate) trait ArcMath<D> {
    fn new(options: ArcOptions) -> Self;
    fn build(&mut self, from: D, to: D, direction: ArcDirection) -> &[D];
}
