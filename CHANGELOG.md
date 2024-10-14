## [1.7.0] - 2024-10-07
### Added
- New `FillRule` options: `Positive` and `Negative`.
- Experimental Line String API:
    - `StringOverlay`, `StringGraph`, `F32StringOverlay`, `F32OverlayGraph`, `F64StringOverlay`,`F64OverlayGraph`
    - `slice` API for slicing polygons and line strings.
    - `clip` API for clipping line strings against shapes.

## [1.7.1] - 2024-10-14
### Changed

Updated `clip_string_lines` methods to output `Vec<IntPath>` instead of `Vec<IntLine>`.