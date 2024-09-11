# iOverlay
![Balloons](readme/balloons.svg)

The iOverlay is a fast poly-bool library supporting main operations like union, intersection, difference, and xor, governed by either the even-odd or non-zero rule.  
This library is optimized for different scenarios, ensuring high performance across various use cases. For detailed performance benchmarks, check out the [Performance Comparison](https://ishape-rust.github.io/iShape-js/overlay/performance.html)


## [Documentation](https://ishape-rust.github.io/iShape-js/overlay/stars_demo.html)
Try out iOverlay with an interactive demo:

- [Stars Rotation](https://ishape-rust.github.io/iShape-js/overlay/stars_demo.html)
- [Shapes Editor](https://ishape-rust.github.io/iShape-js/overlay/shapes_editor.html)



## Features

- **Operations**: union, intersection, difference, and exclusion.
- **Polygons**: with holes, self-intersections, and multiple paths.
- **Simplification**: removes degenerate vertices and merges collinear edges.
- **Fill Rules**: even-odd and non-zero.
- **Data Types**: Supports i32, f32, and f64 APIs.

## Getting Started

Add the following to your Cargo.toml:
```
[dependencies]
i_overlay = "^1.5"
```

### Hello world

Let's union two squares
```rust
let subj = [
    F64Point::new(-10.0, -10.0),
    F64Point::new(-10.0, 10.0),
    F64Point::new(10.0, 10.0),
    F64Point::new(10.0, -10.0),
].to_vec();

let clip = [
    F64Point::new(-5.0, -5.0),
    F64Point::new(-5.0, 15.0),
    F64Point::new(15.0, 15.0),
    F64Point::new(15.0, -5.0),
].to_vec();

let mut overlay = F64Overlay::new();

overlay.add_path(subj, ShapeType::Subject);
overlay.add_path(clip, ShapeType::Clip);

let graph = overlay.into_graph(FillRule::NonZero);
let shapes = graph.extract_shapes(OverlayRule::Union);

println!("shapes count: {}", shapes.len());

if shapes.len() > 0 {
    let contour = &shapes[0][0];
    println!("shape 0 contour: ");
    for p in contour {
        let x = p.x;
        let y = p.y;
        println!("({}, {})", x, y);
    }
}
```

### Shapes result

The output of the `extract_shapes` function is a `Vec<Vec<Vec<F64Point>>>`, where:

- The outer `Vec<F64Shape>` represents a set of shapes.
- Each shape `Vec<F64Path>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
- Each path `Vec<F64Point>` is a sequence of points, forming a closed path.

**Note**: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.

# Overlay Rules

<img src="readme/ab.svg" alt="AB" style="width:50%;">

## Union, A or B
<img src="readme/union.svg" alt="Union" style="width:50%;">

## Intersection, A and B
<img src="readme/intersection.svg" alt="Intersection" style="width:50%;">

## Difference, A - B
<img src="readme/difference_ab.svg" alt="Difference" style="width:50%;">

## Inverse Difference, B - A
<img src="readme/difference_ba.svg" alt="Inverse Difference" style="width:50%;">

## Exclusion, A xor B
<img src="readme/exclusion.svg" alt="Exclusion" style="width:50%;">