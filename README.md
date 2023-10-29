# iOverlay

<p align="center">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/balloons.svg" width="250"/>
</p>
The i_overlay is a poly-bool library that supports main operations such as union, intersection, difference, xor, and self-intersection by the even-odd rule. This algorithm is based on Vatti clipping ideas but is an original implementation.

## [Demo](https://ishape-rust.github.io/iShape-js/demo/stars_demo.html)
Try out iOverlay with an interactive demo. The demo covers operations like union, intersection, difference and exclusion

- [Stars Rotation](https://ishape-rust.github.io/iShape-js/overlay/stars_demo.html)
- [Shapes Editor](https://ishape-rust.github.io/iShape-js/overlay/shapes_editor.html)



## Features

- Supports all basic set operations such as union, intersection, difference, exclusion and self-intersection.
- Capable of handling various types of polygons, including self-intersecting polygons, multiple paths and polygons with holes.
- Optimizes by removing unnecessary vertices and merging parallel edges.
- Effectively handles an arbitrary number of overlaps, resolving them using the even-odd rule.
- Employs integer arithmetic for computations.



## Working Range and Precision
The i_overlay library operates within the following ranges and precision levels:

Extended Range: From -1,000,000 to 1,000,000 with a precision of 0.001.
Recommended Range: From -100,000 to 100,000 with a precision of 0.01 for more accurate results.
Utilizing the library within the recommended range ensures optimal accuracy in computations and is advised for most use cases.



## Getting Started

Add the following to your Cargo.toml:
```
[dependencies]
i_float = "^0.1.0"
i_shape = "^0.1.0"
i_overlay = "^0.2.0"
```

### Example

Here is a simple example that demonstrates how to use the iOverlay library for polygon union operations.
```rust
use i_float::fix_vec::FixVec;
use i_overlay::{layout::overlay::Overlay, fill::shape_type::ShapeType, bool::fill_rule::FillRule};

fn main() {
    let mut overlay = Overlay::new(1);
        
    let subj = [
        FixVec::new_number(-10, -10),
        FixVec::new_number(-10,  10),
        FixVec::new_number( 10,  10),
        FixVec::new_number( 10, -10)
    ];

    let clip = [
        FixVec::new_number(-5, -5),
        FixVec::new_number(-5, 15),
        FixVec::new_number(15, 15),
        FixVec::new_number(15, -5)
    ];

    overlay.add_path(subj.to_vec(), ShapeType::SUBJECT);
    overlay.add_path(clip.to_vec(), ShapeType::CLIP);

    let graph = overlay.build_graph();

    let shapes = graph.extract_shapes(FillRule::Union);

    println!("shapes count: {}", shapes.len());

    if shapes.len() > 0 {
        let contour = shapes[0].contour();
        println!("shape 0 contour: ");
        for p in contour {
            let x = p.x.float();
            let y = p.x.float();
            println!("({}, {})", x, y);
        }
    }
}
```



### Union
<p align="left">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/union.svg" width="250"/>
</p>

### Difference
<p align="left">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/difference.svg" width="250"/>
</p>

### Intersection
<p align="left">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/intersection.svg" width="250"/>
</p>

### Exclusion (xor)
<p align="left">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/exclusion.svg" width="250"/>
</p>

### Self-intersection
<p align="left">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/self-intersecting.svg" width="250"/>
</p>

