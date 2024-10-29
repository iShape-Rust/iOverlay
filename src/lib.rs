//! # iOverlay
//!
//! The `iOverlay` library provides high-performance boolean operations on polygons, including union, intersection, difference, and xor. It is designed for applications that require precise polygon operations, such as computer graphics, CAD systems, and geographical information systems (GIS). By supporting both integer (i32) and floating-point (f32, f64) APIs, iOverlay offers flexibility and precision across diverse use cases.
//!
//! ## Features
//! - **Operations**: union, intersection, difference, and exclusion.
//! - **Polygons**: with holes, self-intersections, and multiple paths.
//! - **Simplification**: removes degenerate vertices and merges collinear edges.
//! - **Fill Rules**: even-odd and non-zero.
//! - **Data Types**: Supports i32, f32, and f64 APIs.
//!
//! ## i32 Example
//! Here's an example of performing a union operation between two polygons:
//!
//! ```rust
//!use i_float::int::point::IntPoint;
//!use i_overlay::core::fill_rule::FillRule;
//!use i_overlay::core::overlay::Overlay;
//!use i_overlay::core::overlay_rule::OverlayRule;
//!
//!let subj = [
//!    // Define the subject polygon (a square)
//!    IntPoint::new(-10, -10),
//!    IntPoint::new(-10, 10),
//!    IntPoint::new(10, 10),
//!    IntPoint::new(10, -10),
//!].to_vec();
//!
//!let clip = [
//!    // Define the clip polygon (a slightly shifted square)
//!    IntPoint::new(-5, -5),
//!    IntPoint::new(-5, 15),
//!    IntPoint::new(15, 15),
//!    IntPoint::new(15, -5),
//!].to_vec();
//!
//!let shapes = Overlay::with_paths(&[subj], &[clip])
//!    .into_graph(FillRule::NonZero)
//!    .extract_shapes(OverlayRule::Union);
//!
//!println!("shapes count: {}", shapes.len());
//!
//!if shapes.len() > 0 {
//!    let contour = &shapes[0][0];
//!    println!("shape 0 contour: ");
//!    for p in contour {
//!        let x = p.x;
//!        let y = p.y;
//!        println!("({}, {})", x, y);
//!    }
//!}
//! ```
//! The `extract_shapes` function for `i32` returns a `Vec<IntShapes>`:
//!
//! - `Vec<IntShape>`: A collection of shapes.
//! - `IntShape`: Represents a shape made up of:
//!   - `Vec<IntPath>`: A list of paths (contours).
//!   - The first path is the outer boundary (clockwise), and subsequent paths represent holes (counterclockwise).
//! - `IntPath`: A sequence of points (`Vec<IntPoint>`) forming a closed contour.
//!
//! **Note**: _Outer boundary paths have a clockwise order, and holes have a counterclockwise order. [More information](https://ishape-rust.github.io/iShape-js/overlay/contours/contours.html) about contours._
//! ## f64 Example
//! Same example but with float api:
//!
//! ```rust
//!use i_overlay::core::fill_rule::FillRule;
//!use i_overlay::core::overlay::ShapeType;
//!use i_overlay::core::overlay_rule::OverlayRule;
//!use i_overlay::f64::overlay::F64Overlay;
//!use i_overlay::i_float::f64_point::F64Point;
//!
//! let subj = [
//!    // Define the subject polygon (a square)
//!    F64Point::new(-10.0, -10.0),
//!    F64Point::new(-10.0, 10.0),
//!    F64Point::new(10.0, 10.0),
//!    F64Point::new(10.0, -10.0),
//! ].to_vec();
//!
//! let clip = [
//!    // Define the clip polygon (a slightly shifted square)
//!    F64Point::new(-5.0, -5.0),
//!    F64Point::new(-5.0, 15.0),
//!    F64Point::new(15.0, 15.0),
//!    F64Point::new(15.0, -5.0),
//! ].to_vec();
//!
//! let mut overlay = F64Overlay::new();
//!
//! overlay.add_path(subj, ShapeType::Subject);
//! overlay.add_path(clip, ShapeType::Clip);
//!
//! let graph = overlay.into_graph(FillRule::NonZero);
//! let shapes = graph.extract_shapes(OverlayRule::Union);
//!
//! println!("shapes count: {}", shapes.len());
//!
//! if shapes.len() > 0 {
//!    let contour = &shapes[0][0];
//!    println!("shape 0 contour: ");
//!    for p in contour {
//!        let x = p.x;
//!        let y = p.y;
//!        println!("({}, {})", x, y);
//!    }
//! }
//! ```
//! The result of the `extract_shapes` function for `f64` returns a `Vec<F64Shapes>`:
//!
//! - `Vec<F64Shape>`: A collection of shapes.
//! - `F64Shape`: Represents one shape, consisting of:
//!    - `Vec<F64Path>`: A list of paths (contours).
//!    - The first path is the outer boundary (clockwise), and subsequent paths represent holes (counterclockwise).
//!  - `F64Path`: A series of points (`Vec<F64Point>`) forming a closed contour.
//!
//!  **Note**: _Outer boundary paths have a clockwise order, and holes have a counterclockwise order. [More information](https://ishape-rust.github.io/iShape-js/overlay/contours/contours.html) about contours._
//!
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