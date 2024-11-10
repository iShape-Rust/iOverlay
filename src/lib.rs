//! # iOverlay
//!
//! The `iOverlay` library provides high-performance boolean operations on polygons, including union, intersection, difference, and xor. It is designed for applications that require precise polygon operations, such as computer graphics, CAD systems, and geographical information systems (GIS). By supporting both integer (i32) and floating-point (f32, f64) APIs, iOverlay offers flexibility and precision across diverse use cases.
//!
//! ## Features
//! - **Boolean Operations**: union, intersection, difference, and exclusion.
//! - **String Line Operations**: clip and slice.
//! - **Polygons**: with holes, self-intersections, and multiple contours.
//! - **Simplification**: removes degenerate vertices and merges collinear edges.
//! - **Fill Rules**: even-odd, non-zero, positive and negative.
//! - **Data Types**: Supports i32, f32, and f64 APIs.
//!
//! ## Simple Example
//! ![iOverlay Logo](https://raw.githubusercontent.com/iShape-Rust/iOverlay/main/readme/example_union.svg)
//! Here's an example of performing a union operation between two polygons:
//!
//! ```rust
//!use i_overlay::core::fill_rule::FillRule;
//!use i_overlay::core::overlay_rule::OverlayRule;
//!use i_overlay::float::single::SingleFloatOverlay;
//!
//! // Define the subject "O"
//!let subj = [
//!    // main contour
//!    vec![
//!         [1.0, 0.0],
//!         [1.0, 5.0],
//!         [4.0, 5.0],
//!         [4.0, 0.0], // the contour is auto closed!
//!    ],
//!    // hole contour
//!    vec![
//!         [2.0, 1.0],
//!         [3.0, 1.0],
//!         [3.0, 4.0],
//!         [2.0, 4.0], // the contour is auto closed!
//!    ],
//!];
//!
//! // Define the clip "-"
//!let clip = [
//!    // main contour
//!    [0.0, 2.0],
//!    [5.0, 2.0],
//!    [5.0, 3.0],
//!    [0.0, 3.0], // the contour is auto closed!
//!];
//!
//!let result = subj.overlay(&clip, OverlayRule::Union, FillRule::EvenOdd);
//!
//!println!("result: {}", result);
//! ```
//! The result is a vec of shapes:
//! ```text
//! [
//!     // first shape
//!     [
//!         // main contour
//!         [
//!             [0.0, 2.0], [0.0, 3.0], [1.0, 3.0], [1.0, 5.0], [4.0, 5.0], [4.0, 3.0], [5.0, 3.0], [5.0, 2.0], [4.0, 2.0], [4.0, 0.0], [1.0, 0.0], [1.0, 2.0]
//!         ],
//!         // first hole
//!         [
//!             [2.0, 2.0], [2.0, 1.0], [3.0, 1.0], [3.0, 2.0]
//!         ],
//!         // second hole
//!         [
//!             [2.0, 4.0], [2.0, 3.0], [3.0, 3.0], [3.0, 4.0]
//!         ]
//!     ]
//!     // ... other shapes if present
//! ]
//! ```
//! The `overlay` function returns a `Vec<Shapes>`:
//!
//! - `Vec<Shape>`: A collection of shapes.
//! - `Shape`: Represents a shape made up of:
//!   - `Vec<Contour>`: A list of contours.
//!   - The first contour is the outer boundary (clockwise), and subsequent contours represent holes (counterclockwise).
//! - `Contour`: A sequence of points (`Vec<P: FloatPointCompatible>`) forming a closed contour.
//!
//! **Note**: Outer boundary contours have a clockwise order, and holes have a counterclockwise order. [More information](https://ishape-rust.github.io/iShape-js/overlay/contours/contours.html) about contours.





pub mod fill;
pub mod core;
pub mod vector;
pub mod f64;
pub mod f32;
pub mod float;
pub mod string;
pub mod segm;

pub(crate) mod split;
pub(crate) mod bind;
pub(crate) mod geom;
pub(crate) mod util;

pub use i_float;
pub use i_shape;