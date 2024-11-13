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
//! ![Simple Example](https://raw.githubusercontent.com/iShape-Rust/iOverlay/main/readme/example_union.svg)
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
//!println!("result: {:?}", result);
//! ```
//! The result is a vec of shapes:
//! ```text
//! [
//!     // first shape
//!     [
//!         // main contour (clockwise order)
//!         [
//!             [0.0, 2.0], [0.0, 3.0], [1.0, 3.0], [1.0, 5.0], [4.0, 5.0], [4.0, 3.0], [5.0, 3.0], [5.0, 2.0], [4.0, 2.0], [4.0, 0.0], [1.0, 0.0], [1.0, 2.0]
//!         ],
//!         // first hole (counterclockwise order)
//!         [
//!             [2.0, 2.0], [2.0, 1.0], [3.0, 1.0], [3.0, 2.0]
//!         ],
//!         // second hole (counterclockwise order)
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
//! ## Using a Custom Point Type
//!
//! `iOverlay` allows users to define custom point types, as long as they implement the `FloatPointCompatible` trait.
//!
//! Here's an example:
//!
//!```rust
//! use i_float::float::compatible::FloatPointCompatible;
//! use i_overlay::core::fill_rule::FillRule;
//! use i_overlay::core::overlay_rule::OverlayRule;
//! use i_overlay::float::single::SingleFloatOverlay;
//!
//! #[derive(Clone, Copy, Debug)]
//! struct CustomPoint {
//!     x: f32,
//!     y: f32,
//! }
//!
//! // Implement the `FloatPointCompatible` trait for CustomPoint
//! impl FloatPointCompatible<f32> for CustomPoint {
//!     fn from_xy(x: f32, y: f32) -> Self {
//!         Self { x, y }
//!     }
//!
//!     fn x(&self) -> f32 {
//!         self.x
//!     }
//!
//!     fn y(&self) -> f32 {
//!         self.y
//!     }
//! }
//!
//! let subj = [
//!     CustomPoint { x: 0.0, y: 0.0 },
//!     CustomPoint { x: 0.0, y: 3.0 },
//!     CustomPoint { x: 3.0, y: 3.0 },
//!     CustomPoint { x: 3.0, y: 0.0 },
//! ];
//!
//! let clip = [
//!     CustomPoint { x: 1.0, y: 1.0 },
//!     CustomPoint { x: 1.0, y: 2.0 },
//!     CustomPoint { x: 2.0, y: 2.0 },
//!     CustomPoint { x: 2.0, y: 1.0 },
//! ];
//!
//! let result = subj.overlay(&clip, OverlayRule::Difference, FillRule::EvenOdd);
//!
//! println!("result: {:?}", result);
//! ```
//!
//! ## Slicing a Polygon by a String Line
//! ![Slicing Example](https://raw.githubusercontent.com/iShape-Rust/iOverlay/main/readme/example_slice.svg)
//!
//!```rust
//! use i_overlay::core::fill_rule::FillRule;
//! use i_overlay::float::single::SingleFloatOverlay;
//! use i_overlay::float::slice::FloatSlice;
//!
//! let polygon = [
//!     [1.0, 1.0],
//!     [1.0, 4.0],
//!     [4.0, 4.0],
//!     [4.0, 1.0],
//! ];
//!
//! let slicing_line = [
//!     [3.0, 5.0],
//!     [2.0, 2.0],
//!     [3.0, 3.0],
//!     [2.0, 0.0],
//! ];
//!
//! let result = polygon.slice_by(&slicing_line, FillRule::NonZero);
//!
//! println!("result: {:?}", result);
//! ```
//! ## Clip a String Lines by a Polygon
//! ![Clip Example](https://raw.githubusercontent.com/iShape-Rust/iOverlay/main/readme/example_clip.svg)
//!
//!```rust
//! use i_overlay::core::fill_rule::FillRule;
//! use i_overlay::float::clip::FloatClip;
//! use i_overlay::float::single::SingleFloatOverlay;
//! use i_overlay::string::clip::ClipRule;
//!
//! let string_line = [
//!     [3.0, 5.0],
//!     [2.0, 2.0],
//!     [3.0, 3.0],
//!     [2.0, 0.0],
//! ];
//!
//! let polygon = [
//!     [1.0, 1.0],
//!     [1.0, 4.0],
//!     [4.0, 4.0],
//!     [4.0, 1.0],
//! ];
//!
//! let clip_rule = ClipRule { invert: false, boundary_included: false };
//! let result = string_line.clip_by(&polygon, FillRule::NonZero, clip_rule);
//!
//! println!("result: {:?}", result);
//! ```


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