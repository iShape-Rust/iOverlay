# Variable Stroke Debug Editor

Focused desktop debugger for `i_overlay::mesh::variable_stroke`. It reuses the `iced` camera,
sheet, path/shape rendering, point dragging, and fixture navigation approach from
`examples/overlay_editor`.

Run from the repository root:

```bash
cargo run --release --manifest-path examples/variable_stroke_debug/Cargo.toml
```

The editor loads every JSON fixture in `examples/tests/variable_stroke` (including
`test_10`, `test_11`, and `test_12`). Drag a diamond to move a `StrokeVertex`, select a vertex in
the left panel to change its width, and adjust `round_angle` with the slider. Mouse-drag empty
canvas space to pan and use the wheel/trackpad to zoom. Up/down arrow keys switch fixtures.

The layer controls show or hide the centerline and radius guides, tangent section boundaries,
join chords, cap chords, straight closing edges, and the final post-overlay contour. Raw edges
are colored by construction role and can display arrowheads in exact `SegmentBuilder` insertion
direction. The header reports raw edge counts by category.

The instrumentation is compiled only with the `variable_stroke_debug` feature. Normal
`i_overlay` users do not see the debug trait or result types.
