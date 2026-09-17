# Mesh construction math

The integer and floating-point styles for stroke, outline, and variable-width
stroke all accept `.math(MathMode::Float)` from `i_overlay::mesh::math`. This
selects construction arithmetic, independently of the input coordinate type.

Use `MathMode::Integer` when cross-platform deterministic construction is
required. For end-to-end reproducibility, use `mesh::int` with identical integer
inputs, styles, integer engine, and library version. Otherwise, prefer
`MathMode::Float`: its normalization and arc construction are more accurate and
generally faster than the approximate integer math. Total stroke performance
also depends on the geometry and the boolean operation.

`Integer` remains the default; select `Float` explicitly. The `mesh::float`
namespace selects floating-point input coordinates, not construction arithmetic.
Both input APIs support either math mode, and both modes retain integer
coordinates and boolean operations internally.

Integer miter construction clamps the requested minimum interior angle to at
least 5 degrees and clips sharper corners. Interior angles above 175 degrees
use bevel joins. This uses the turn angle already computed for the join, without
additional normalization or trigonometry. Float mode retains its existing angle
policy, including ordinary miter intersections at nearly straight corners.
For `IntLineJoin::Miter`, the lower limit on the requested minimum is 5 degrees in
Integer mode or 0.01*pi (1.8 degrees) in Float mode; the upper limit is one
`Angle` unit below pi. The floating-point `LineJoin::Miter` style first clamps
its parameter to 0.01*pi..=0.99*pi (1.8..=178.2 degrees), before the selected
math mode applies its construction limits. Angular thresholds are quantized to
the `Angle` representation.

```rust
use i_overlay::mesh::float::{stroke::offset::StrokeOffset, style::StrokeStyle};
use i_overlay::mesh::math::MathMode;

let path = [[0.0, 0.0], [3.0, 4.0]];
let style = StrokeStyle::new(1.0).math(MathMode::Float);
let result = path.stroke(style, false);
```

Construction dispatches once to a generic builder using `IntegerMath` or `FloatMath`. Float math
computes normalization and trigonometry in f64, then stores directions in
`UnitIntVector`. Normalization uses `UnitIntVector::normalize_with_float`, and arc
samples use `UnitIntVector::from_float_unchecked`. Scaling, custom-cap rotation,
miter intersections, integer coordinates, and the boolean engine stay integer.
Point differences are formed before float conversion to preserve local geometry
at large i64 origins; angle calculations form direction cross/dot products before
converting them to f64.

Float arcs cache a rotation and reuse their output allocation. Floating rotation state is retained between samples; quantization of an emitted direction is not fed back into the next rotation. The step reserves angular error for f64 arithmetic and fixed-scale component truncation. The original integer endpoints are retained. `ArcOptions::max_step` still applies, but `rotation_precision` is specific to the Integer mode and ignored by Float.

Conversion truncates fixed-scale components toward zero without checking the
integer squared norm. Float normalization and rotation are approximate: directions
may be slightly longer than one. There is no contraction step or guarantee of an
exact norm bound. Float stroke and outline bounds reserve a margin for numerical drift and
coordinate rounding. Final points still lie on the integer grid, so Float mode
does not eliminate coordinate quantization or rounding-sensitive intersections.

The modes may produce different rounded vertices and arc tessellations. Float
mode does not promise cross-platform bitwise reproducibility. Code using
exhaustive style struct literals must supply the `math` field.

Variable-width strokes also select the tangent-contact calculation through a
private `VariableStrokeMath` extension of `MeshMath`. Integer mode retains the
scaled integer square root and rounded multiply/divide calculation. Float mode
uses the same external-tangent formula in f64 and rounds local offsets before
adding integer centers. The squared-length difference is computed in integers
before conversion, preserving small positive differences near containment.
Containment and orientation predicates remain integer operations.

Variable-width paths are consumed once with a current chain and previous section;
the float adapter converts vertices lazily. There are no intermediate path or
subsegment collections. Arc storage is reused across paths. Debug edge collection
uses the same traversal and records the same emitted geometry. The style-free
`validate_variable_stroke()` uses conservative padding valid for both math modes.

## Validation

- Integer mesh/arc/outline regression tests exercise the default construction mode.
- Miter regressions cover almost straight strokes and shrinking outlines, the 175-degree bevel cutoff, and the 5-degree clipping floor, with separate Float controls.
- Both math modes cover constant-width round-stroke vertex disks, path reversal, coarse caps, empty paths, duplicates, and reused flat output.
- Float arc tests check ordered samples, maximum angular gaps, approximate unit length within a numerical tolerance, and allocation reuse for i16/i32/i64.
- Large-origin i64 tests translate the input by 2^60 and compare the translated-back stroke.

- Both modes cover outline expansion/contraction and variable-width stroke areas for i16/i32/i64, flat output, and large-origin translation.
- A deterministic randomized test compares streaming integer variable-stroke segments with the former partitioned traversal.
- Variable-width stress tests exercise both construction modes, including reversed paths and the i64 engine.
