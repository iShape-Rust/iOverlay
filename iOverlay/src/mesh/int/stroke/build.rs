#[cfg(debug_assertions)]
use super::bounds::stroke_padding_with_math;
use super::{builder::StrokeBuilder, offset::IntStrokeOffset};
use crate::core::{
    integer::OverlayInt,
    overlay::{IntOverlayOptions, Overlay},
};
use crate::mesh::int::{
    math::{backend::MeshMath, float::FloatMath, integer::IntegerMath},
    style::IntStrokeStyle,
};
use crate::mesh::math::MathMode;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
#[cfg(debug_assertions)]
use i_float::int::rect::IntRect;

pub(super) trait BuildStrokeOverlay<I: OverlayInt>: IntStrokeOffset<I> {
    fn build_stroke_overlay(
        &self,
        style: &IntStrokeStyle<I>,
        closed: bool,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Overlay<I> {
        build_stroke_overlay_iter(
            self.iter_paths().map(|path| path.iter().copied()),
            style,
            closed,
            options,
        )
    }
}
impl<I: OverlayInt, S: IntStrokeOffset<I> + ?Sized> BuildStrokeOverlay<I> for S {}

/// Builds one overlay from paths consumed once, without retaining input points.
pub(crate) fn build_stroke_overlay_iter<I, Paths, Path>(
    paths: Paths,
    style: &IntStrokeStyle<I>,
    closed: bool,
    options: IntOverlayOptions<I::WideUInt>,
) -> Overlay<I>
where
    I: OverlayInt,
    Paths: IntoIterator<Item = Path>,
    Path: IntoIterator<Item = IntPoint<I>>,
{
    match style.math {
        MathMode::Integer => {
            build_stroke_overlay_with_math::<I, IntegerMath, _, _>(paths, style, closed, options)
        }
        MathMode::Float => {
            build_stroke_overlay_with_math::<I, FloatMath, _, _>(paths, style, closed, options)
        }
    }
}

fn build_stroke_overlay_with_math<I, M, Paths, Path>(
    paths: Paths,
    style: &IntStrokeStyle<I>,
    closed: bool,
    options: IntOverlayOptions<I::WideUInt>,
) -> Overlay<I>
where
    I: OverlayInt,
    M: MeshMath<I>,
    Paths: IntoIterator<Item = Path>,
    Path: IntoIterator<Item = IntPoint<I>>,
{
    let mut builder = StrokeBuilder::<I, M>::new(style);
    let mut segments = Vec::new();
    #[cfg(debug_assertions)]
    let padding = stroke_padding_with_math::<I, M>(style);
    for path in paths {
        #[cfg(debug_assertions)]
        let path = path.into_iter().inspect(|point| {
            // Checking each expanded point is equivalent to checking the expanded
            // input rectangle, and happens before the point enters geometry math.
            debug_assert!(
                crate::mesh::int::bounds::expanded_is_safe(IntRect::with_point(*point), padding),
                "stroke bounds exceed the safe coordinate range"
            );
        });
        builder.build(path, closed, &mut segments);
    }
    let mut overlay = Overlay::with_segments(segments);
    overlay.options = options;
    overlay
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{fill_rule::FillRule, overlay_rule::OverlayRule};
    use alloc::vec;
    use core::cell::Cell;

    #[test]
    fn one_pass_paths_match_resource_strokes() {
        let paths = vec![
            vec![],
            vec![IntPoint::new(0, 0)],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(0, 0),
                IntPoint::new(8192, 0),
                IntPoint::new(8192, 8192),
            ],
            vec![IntPoint::new(4096, -4096), IntPoint::new(4096, 4096)],
            vec![],
        ];
        for math in [MathMode::Integer, MathMode::Float] {
            let style = IntStrokeStyle::new(1024).math(math);
            for closed in [false, true] {
                let expected = paths.stroke(&style, closed).unwrap();
                let visited = Cell::new(0);
                let visited_ref = &visited;
                let mut expected_visited = 0;
                let iter = paths.clone().into_iter().map(move |path| {
                    // Finish each path before requesting the next one.
                    assert_eq!(visited_ref.get(), expected_visited);
                    expected_visited += path.len();
                    let mut points = path.into_iter();
                    // A single-pass point iterator; no Clone or slice access required.
                    core::iter::from_fn(move || {
                        let point = points.next()?;
                        visited_ref.set(visited_ref.get() + 1);
                        Some(point)
                    })
                });
                let actual = build_stroke_overlay_iter(iter, &style, closed, Default::default())
                    .overlay(OverlayRule::Subject, FillRule::Positive);
                assert_eq!(actual, expected);
                assert_eq!(visited.get(), paths.iter().map(Vec::len).sum::<usize>());
            }
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "stroke bounds exceed the safe coordinate range")]
    fn iterator_checks_range_before_building_section() {
        let paths = [[IntPoint::new(0, 0), IntPoint::new(i32::MAX, 0)]];
        build_stroke_overlay_iter(paths, &IntStrokeStyle::new(1024), false, Default::default());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "stroke bounds exceed the safe coordinate range")]
    fn zero_radius_still_checks_range() {
        let paths = [[IntPoint::new(i32::MAX, 0)]];
        build_stroke_overlay_iter(paths, &IntStrokeStyle::new(0), false, Default::default());
    }
}
