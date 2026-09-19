//! Floating-point inputs must be finite, with absolute coordinates at most
//! `2^60` for `f32` or `2^500` for `f64`. Stroke and outline bounds, including
//! their padding, must also fit. Infallible APIs panic on invalid bounds;
//! fixed-scale APIs return an error. Empty valid input remains supported.
//!
//! Automatic conversion uses the conservative coordinate budget
//! (`FloatPointAdapter::CONSERVATIVE_COORDINATE_BITS`, or `I::BITS - 3`),
//! reserving an extra bit for rounding inside the arithmetic range. Fixed-scale APIs
//! enforce the same budget and require a positive finite scale with a finite
//! reciprocal in the input scalar type. Custom adapters remain the caller's
//! responsibility; see the integer coordinate contract in [`crate::core::integer`].

pub mod clip;
pub mod graph;
pub mod hierarchy;
pub mod overlay;
pub mod relate;
pub mod scale;
pub mod simplify;
pub mod single;
pub mod slice;
pub mod string_graph;
pub mod string_overlay;
