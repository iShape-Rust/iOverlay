# Integer coordinate range audit

The coordinate interval is `-2^(N - 2)..=2^(N - 2) - 1` for an `N`-bit
engine. Both endpoints are included. Let `D = 2^(N - 1) - 1 = I::MAX`.
Every coordinate difference then has magnitude at most `D`.

This is a bound on coordinate arithmetic for polygon, string, predicate, and
vector overlays. It does not bound allocation sizes, winding counts, or
arithmetic in user-provided edge-data callbacks. Floating-point inputs must
respect the interval **after** conversion; custom scales are the caller's
responsibility.

## Point and vector operations

Point subtraction widens to `I::Wide`. Each product of differences has magnitude
at most `D^2`; a dot product, cross product, or squared distance is bounded by
`2 * D^2`. For all three engines:

```text
2 * D^2 = 2^(2*N - 1) - 2^(N + 1) + 2 < 2^(2*N - 1)
```

Thus these expressions, including negation of cross products, fit in the signed
wide type (`i32`, `i64`, or `i128`). The same calculation covers input collinearity
filtering, segment ordering, hole binding, point queries, and snapping distances.

## Intersection coordinates

`CrossSolver::cross_point` translates endpoints relative to one endpoint. The
translated coordinates and the other segment's direction still have magnitude
at most `D`: subtracting two translated coordinates cancels the common offset.
The determinants `xy_b` and `div` are bounded by `2 * D^2`.

The general intersection numerator multiplies a determinant by a direction
component. It uses `UIntProduct`, with **4*N bits**, rather than `I::Wide`:
its magnitude is at most `2 * D^3 < 2^(3*N - 2)`. The unsigned divisor is
positive and below `2^(2*N - 1)`, satisfying `divide_with_rounding`'s precondition.
The quotient is an intersection displacement and has magnitude at most `D`.

A true segment intersection is within both segments' coordinate bounds.
Rounding or truncating a coordinate relative to an integer endpoint cannot
move it outside those integer bounds. Snapping selects an existing endpoint.
Consequently, every repair iteration preserves the global input coordinate box.

## Area accumulation

Twice the final area of an output contour is at most `2 * D^2`. A partial
shoelace sum has no such bound: a simple spiral can wind around the origin many
times before the return path cancels most of the accumulated area.

Both ordinary and vector contours therefore accumulate area with wrapping
arithmetic. This computes the exact sum modulo `2^(2*N)`; since the final signed
area fits, its final representation is exact. Reducing the coordinate range by
a fixed number of bits would not solve arbitrary partial-sum overflow.

## Boundary coverage

- All engines and solver strategies: inclusive endpoints, one-unit features,
  holes, steep edges, collinear overlaps, exact and rounded intersections.
- A simple spiral: vector area filtering at its exact area and one unit above.
- Growing and oversized snapping thresholds, including shift and addition limits.
- Boundary intersection coordinates checked against an independent `i128`
  rational calculation, including `i64` intersections using extended products.
- Fragment enclosures checked against exact rational segment heights for steep,
  shallow, increasing, and decreasing edges at several grid resolutions.
- Seeded `i16` and `i32` overlays compared with the same input in the `i64` engine.

Adding one more unit to the interval allows a positive difference `D + 1`
that does not fit in `I`, and `2 * (D + 1)^2` does not fit in `I::Wide`.
Larger intervals therefore need a different arithmetic contract.
