#[cfg(feature = "variable_stroke_debug")]
use super::debug::IntVariableStrokeDebugEdge;
use super::offset::IntVariableStrokeOffset;
use super::{
    IntStrokeVertex, IntVariableStrokeStyle, builder::VariableStrokeBuilder, math::VariableStrokeMath,
};
use crate::core::{
    integer::OverlayInt,
    overlay::{IntOverlayOptions, Overlay},
};
use crate::mesh::{
    int::math::{float::FloatMath, integer::IntegerMath},
    math::MathMode,
};
use alloc::vec::Vec;
pub(super) trait BuildVariableOverlay<I: OverlayInt>: IntVariableStrokeOffset<I> {
    fn build_variable_overlay(
        &self,
        style: IntVariableStrokeStyle,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Overlay<I> {
        build_variable_overlay_iter(
            self.iter_variable_paths().map(|path| path.iter().copied()),
            style,
            options,
            #[cfg(feature = "variable_stroke_debug")]
            None,
        )
    }
}
impl<I: OverlayInt, S: IntVariableStrokeOffset<I> + ?Sized> BuildVariableOverlay<I> for S {}

/// Consumes each path once, including float inputs mapped to integer vertices.
pub(crate) fn build_variable_overlay_iter<I, Paths, Path>(
    paths: Paths,
    style: IntVariableStrokeStyle,
    options: IntOverlayOptions<I::WideUInt>,
    #[cfg(feature = "variable_stroke_debug")] edges: Option<&mut Vec<IntVariableStrokeDebugEdge<I>>>,
) -> Overlay<I>
where
    I: OverlayInt,
    Paths: IntoIterator<Item = Path>,
    Path: IntoIterator<Item = IntStrokeVertex<I>>,
{
    match style.math {
        MathMode::Integer => build_with_math::<I, IntegerMath, _, _>(
            paths,
            style,
            options,
            #[cfg(feature = "variable_stroke_debug")]
            edges,
        ),
        MathMode::Float => build_with_math::<I, FloatMath, _, _>(
            paths,
            style,
            options,
            #[cfg(feature = "variable_stroke_debug")]
            edges,
        ),
    }
}

fn build_with_math<I, M, Paths, Path>(
    paths: Paths,
    style: IntVariableStrokeStyle,
    options: IntOverlayOptions<I::WideUInt>,
    #[cfg(feature = "variable_stroke_debug")] mut edges: Option<&mut Vec<IntVariableStrokeDebugEdge<I>>>,
) -> Overlay<I>
where
    I: OverlayInt,
    M: VariableStrokeMath<I>,
    Paths: IntoIterator<Item = Path>,
    Path: IntoIterator<Item = IntStrokeVertex<I>>,
{
    let mut builder = VariableStrokeBuilder::<I, M>::new(style);
    let mut segments = Vec::new();
    for (_index, path) in paths.into_iter().enumerate() {
        #[cfg(debug_assertions)]
        let path = path.into_iter().inspect(|vertex| {
            debug_assert!(
                crate::mesh::int::bounds::expanded_is_safe(
                    i_float::int::rect::IntRect::with_point(vertex.point),
                    M::guard_padding(vertex.radius().to_wide()),
                ),
                "variable stroke exceeds coordinate range"
            );
        });
        builder.build(
            path,
            &mut segments,
            #[cfg(feature = "variable_stroke_debug")]
            edges.as_deref_mut(),
            #[cfg(feature = "variable_stroke_debug")]
            _index,
        );
    }
    let mut overlay = Overlay::with_segments(segments);
    overlay.options = options;
    overlay
}

#[cfg(test)]
mod tests {
    use super::super::offset::IntVariableStrokeOffset;
    use super::*;
    use crate::core::{fill_rule::FillRule, overlay_rule::OverlayRule};
    use alloc::vec;
    use core::cell::Cell;
    use i_float::int::point::IntPoint;

    #[test]
    fn paths_and_vertices_are_consumed_once_in_order() {
        let vertex = |x, y, width| IntStrokeVertex::new(IntPoint::new(x, y), width);
        let paths = vec![
            vec![],
            vec![vertex(0, 0, 1000)],
            vec![vertex(0, 0, 200), vertex(0, 0, 400), vertex(4000, 3000, 2000)],
            vec![],
            vec![vertex(0, 0, 0)],
        ];
        for math in [MathMode::Integer, MathMode::Float] {
            let style = IntVariableStrokeStyle::new().math(math);
            let expected = paths.variable_stroke(style).unwrap();
            let visited = Cell::new(0);
            let visited_ref = &visited;
            let mut expected_visited = 0;
            let iter = paths.iter().map(|path| {
                assert_eq!(visited.get(), expected_visited);
                expected_visited += path.len();
                let mut vertices = path.iter().copied();
                core::iter::from_fn(move || {
                    let vertex = vertices.next()?;
                    visited_ref.set(visited_ref.get() + 1);
                    Some(vertex)
                })
            });
            let actual = build_variable_overlay_iter(
                iter,
                style,
                Default::default(),
                #[cfg(feature = "variable_stroke_debug")]
                None,
            )
            .overlay(OverlayRule::Subject, FillRule::Positive);
            assert_eq!(actual, expected);
            assert_eq!(visited.get(), paths.iter().map(Vec::len).sum::<usize>());
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "variable stroke exceeds coordinate range")]
    fn invalid_zero_width_vertex_is_checked_before_geometry() {
        build_variable_overlay_iter(
            [[IntStrokeVertex::new(IntPoint::new(i32::MAX, 0), 0)]],
            IntVariableStrokeStyle::new().math(MathMode::Float),
            Default::default(),
            #[cfg(feature = "variable_stroke_debug")]
            None,
        );
    }
}
