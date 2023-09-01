# iOverlay

<p align="center">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/balloons.svg" width="250"/>
</p>
The i_overlay is a poly-bool library that supports main operations such as union, intersection, difference, xor, and self-intersection by the even-odd rule. This algorithm is based on Vatti clipping ideas but is an original implementation.

## [Demo](https://ishape-rust.github.io/iShape-js/demo/stars_demo.html)
Try out iShape with an interactive demo. The demo covers operations like union, intersection, difference and exclusion

- [Stars Rotation](https://ishape-rust.github.io/iShape-js/demo/stars_demo.html)
- [Shapes Editor](https://ishape-rust.github.io/iShape-js/demo/editor_demo.html)



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



## Basic Usage

Add the following imports:
```rust

```

### Union
<p align="center">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/union.svg" width="400"/>
</p>

### Difference
<p align="center">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/difference.svg" width="400"/>
</p>

### Intersection
<p align="center">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/intersection.svg" width="400"/>
</p>

### Exclusion (xor)
<p align="center">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/exclusion.svg" width="400"/>
</p>

### Self-intersection
<p align="center">
<img src="https://github.com/iShape-Rust/iOverlay/blob/main/Readme/self-intersecting.svg" width="400"/>
</p>

