# iOverlay

<p align="center">

<img src="Readme/balloons.svg" width="250"/>
</p>

The iOverlay is a fast poly-bool library supporting main operations like union, intersection, difference, and xor, governed by either the even-odd or non-zero rule. This algorithm is based on Vatti clipping ideas but is an original implementation.


## [Documentation](https://ishape-rust.github.io/iShape-js/overlay/stars_demo.html)
Try out iOverlay with an interactive demo. The demo covers operations like union, intersection, difference and exclusion

- [Stars Rotation](https://ishape-rust.github.io/iShape-js/overlay/stars_demo.html)
- [Shapes Editor](https://ishape-rust.github.io/iShape-js/overlay/shapes_editor.html)



## Features

- **Operations**: union, intersection, difference, and exclusion.
- **Polygons**: with holes, self-intersections, and multiple paths.
- **Simplification**: removes degenerate vertices and merges collinear edges.
- **Fill Rules**: even-odd and non-zero.

## Getting Started

Add the following to your Cargo.toml:
```
[dependencies]
i_float
i_shape
i_overlay
```

### Hello world

Let's union two squares
```rust
let mut overlay = Overlay::new(2);

let left_bottom_square = FixShape::new_with_contour([
    FixVec::new_f64(-10.0, -10.0),
    FixVec::new_f64(-10.0, 10.0),
    FixVec::new_f64(10.0, 10.0),
    FixVec::new_f64(10.0, -10.0)
].to_vec());

let right_top_square = FixShape::new_with_contour([
    FixVec::new_f64(-5.0, -5.0),
    FixVec::new_f64(-5.0, 15.0),
    FixVec::new_f64(15.0, 15.0),
    FixVec::new_f64(15.0, -5.0)
].to_vec());

// add new geometry
overlay.add_shape(&left_bottom_square, ShapeType::Subject);
overlay.add_shape(&right_top_square, ShapeType::Clip);

// resolve shapes geometry
let graph = overlay.build_graph(FillRule::EvenOdd);

// apply union operation and get result (in our case it will be only one element)
let shapes = graph.extract_shapes(OverlayRule::Union);

// do something with new shapes...

print!("shapes: {:?}", shapes)
```

# Overlay Rules

## Union, A or B
![Union](Readme/union.svg)

## Intersection, A and B
![Intersection](Readme/intersection.svg)

## Difference, B - A
![Difference](Readme/difference.svg)

## Exclusion, A xor B
![Exclusion](Readme/exclusion.svg)
