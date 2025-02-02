## [1.10.0] - 2025-02-02
### Changed
- snap by radius can now grow without limit.
- enum Precision converted to struct
### Added
- New SimplifyShape API `simplify_shape_with_solver` which allow to set Solver.

## [1.9.4] - 2025-01-10
### Fixed
- hole path builder uses clockwise edge priority, which is more topologically natural.
- holes-builder now uses edge orientation and not only its position.

## [1.9.0] - 2024-11-20
### Changed
- new fragment solver for splitting big data set
- multithreading splitting
### Removed
- f32/f64 deprecated api removed
## [1.8.2] - 2024-11-13
### Fixed
- Small fix hole bind for degenerate contours.
## [1.8.1] - 2024-11-12
### Fixed
- Fixed bug bind holes not correct shape index.
## [1.8.0] - 2024-11-11
### Added
- New Float API. A new template-based Float API that uses an iterator, eliminating data cloning. This API can work directly with user-defined Point structures. The previous F32/F64 API is now deprecated. 
- new Single Boolean Operation `overlay`, which work without creating `OverlayGraph`, and can be 10-20% faster in some cases.
### Changed
- The String Line API is now officially supported
- The clip operation now keep the original path order.
### Fixed
- Hole Solver is reworked and connect holes more carefully

## [1.7.4] - 2024-11-06
### Fixed
- Fixed bug bind holes same hole point and contour x_segment.a.
 
## [1.7.3] - 2024-11-05
### Fixed
- Fixed bug min_area filter not work. (thx Azorlogh)

## [1.7.2] - 2024-10-24
### Fixed
- Fixed bug joining holes to shapes when holes were unsorted.

## [1.7.1] - 2024-10-14
### Changed
- Updated `clip_string_lines` methods to output `Vec<IntPath>` instead of `Vec<IntLine>`.

## [1.7.0] - 2024-10-07
### Added
- New `FillRule` options: `Positive` and `Negative`.
- Experimental Line String API:
  - `StringOverlay`, `StringGraph`, `F32StringOverlay`, `F32OverlayGraph`, `F64StringOverlay`,`F64OverlayGraph`
  - `slice` API for slicing polygons and line strings.
  - `clip` API for clipping line strings against shapes.
