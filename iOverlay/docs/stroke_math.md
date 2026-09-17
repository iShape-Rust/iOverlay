# Stroke math prototype

Both `StrokeStyle` and `IntStrokeStyle` accept `.math(MathMode::Float)` from `i_overlay::mesh::math`. `Integer` remains the default. This prototype selects math for constant-width stroke only; outline and variable-width stroke retain their current integer arithmetic.

Integer miter construction clamps the requested minimum interior angle to at
least 5 degrees and clips sharper corners. Interior angles above 175 degrees
use bevel joins. This uses the turn angle already computed for the join, without
additional normalization or trigonometry. Float mode retains its existing angle
policy, including ordinary miter intersections at nearly straight corners.

```rust
use i_overlay::mesh::float::{stroke::offset::StrokeOffset, style::StrokeStyle};
use i_overlay::mesh::math::MathMode;

let path = [[0.0, 0.0], [3.0, 4.0]];
let style = StrokeStyle::new(1.0).math(MathMode::Float);
let result = path.stroke(style, false);
```

Construction dispatches once to a generic `StrokeBuilder<I, M>`. Float math computes normalization and trigonometry in f64, then stores directions in `UnitIntVector`. It uses the new checked `UnitIntVector::try_from_float` constructor from the local iFloat dependency. Scaling, custom-cap rotation, miter intersections, integer coordinates, and the boolean engine stay integer. Differences and cross/dot products are formed before float conversion to preserve local geometry at large i64 origins.

Float arcs cache a rotation and reuse their output allocation. Floating rotation state is retained between samples; quantization of an emitted direction is not fed back into the next rotation. The step reserves angular error for f64 arithmetic and fixed-scale component truncation. The original integer endpoints are retained. `ArcOptions::max_step` still applies, but `rotation_precision` is specific to the Integer mode and ignored by Float.

The checked constructor truncates components toward zero and verifies the integer squared norm. It does not normalize inputs. Float normalization/rotation is contracted slightly before conversion so rounding cannot create a direction longer than one.

The modes may produce different rounded vertices and arc tessellations. Float mode does not promise cross-platform bitwise reproducibility. Defaults are unchanged; code using exhaustive style struct literals must now supply the new `math` field.

## Preliminary timing

Measured on macOS arm64 with rustc 1.98.1, optimized release build. Each operation uses 16 paths of 96 points. Smooth paths follow a sampled sine; jagged paths alternate height. Stroke width is 512 integer units, with a round start cap and square end cap. The reported value is the median of seven batches of ten calls, after one warmup call. Construction includes allocation; full stroke includes boolean extraction.

These are synthetic cases on one machine, not a guarantee of application speedup. Output tessellations differ slightly: Float emitted 16 fewer segments in each case (one per path), so end-to-end numbers do not isolate arithmetic cost.

| Engine | Paths | Join | Build Integer / Float, µs | Full Integer / Float, µs | Full time reduction |
|---|---|---|---:|---:|---:|
| i32 | smooth | Bevel | 129.96 / 33.80 | 2610.26 / 2588.36 | 0.8% |
| i32 | smooth | Miter | 85.73 / 53.88 | 3522.31 / 3447.71 | 2.1% |
| i32 | smooth | Round | 69.13 / 48.25 | 2640.53 / 2637.67 | 0.1% |
| i32 | jagged | Bevel | 44.28 / 34.40 | 1150.73 / 1143.12 | 0.7% |
| i32 | jagged | Miter | 327.85 / 310.98 | 1794.52 / 1754.57 | 2.2% |
| i32 | jagged | Round | 116.77 / 81.83 | 1706.99 / 1664.77 | 2.5% |
| i64 | smooth | Bevel | 135.62 / 49.92 | 3122.18 / 3014.88 | 3.4% |
| i64 | smooth | Miter | 193.69 / 100.92 | 4106.54 / 3946.51 | 3.9% |
| i64 | smooth | Round | 227.73 / 81.46 | 3189.83 / 3040.92 | 4.7% |
| i64 | jagged | Bevel | 138.93 / 47.17 | 1388.85 / 1293.50 | 6.9% |
| i64 | jagged | Miter | 929.12 / 812.11 | 2581.74 / 2478.71 | 4.0% |
| i64 | jagged | Round | 285.85 / 136.63 | 2112.45 / 1932.25 | 8.5% |

Normalization alone was about 1.8–4.8× faster in these cases. Segment construction improved more than total stroke time; boolean work dominates the full operation. Keep Float opt-in while collecting representative application benchmarks.

Reproduce:

```sh
cargo test --release --lib benchmark_math_modes -- --ignored --nocapture
```

## Validation

- Existing integer mesh/arc/outline regression tests retain the default behavior.
- Both math modes cover constant-width round-stroke vertex disks, path reversal, coarse caps, empty paths, duplicates, and reused flat output.
- Float arc tests check ordered samples, maximum angular gaps, exact integer norm bounds, and allocation reuse for i16/i32/i64.
- Large-origin i64 tests translate the input by 2^60 and compare the translated-back stroke.
- iFloat constructor tests include non-finite/out-of-range input and a norm violation too small to detect by squaring in f64.
