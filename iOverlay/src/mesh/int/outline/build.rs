use super::bounds::OutlineBounds;
use super::builder::OutlineBuilder;
use super::builder_join::JoinBuilder;
use super::offset::IntOutlineError;
use crate::core::extract::BooleanExtractionBuffer;
use crate::core::fill_rule::FillRule;
use crate::core::integer::OverlayInt;
use crate::core::overlay::{ContourDirection, IntOverlayOptions, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::mesh::int::bounds::expanded_is_safe;
use crate::mesh::int::join::Join;
use crate::mesh::int::style::IntOutlineStyle;
use alloc::vec::Vec;
use i_float::int::number::uint::UIntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::point::IntPoint;
use i_float::int::rect::IntRect;
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::int::area::Area;
use i_shape::source::int::resource::IntShapeResource;

pub(super) trait BuildOutlineOverlay<I: OverlayInt>: IntShapeResource<I> {
    fn build_overlay(
        &self,
        style: &IntOutlineStyle<I>,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Result<Overlay<I>, IntOutlineError> {
        debug_assert!(
            self.validate_outline_bounds(style).is_ok(),
            "outline bounds exceed the safe coordinate range"
        );
        Ok(build_outline_overlay_iter(
            self.iter_paths().map(|path| path.iter().copied()),
            style,
            options,
        ))
    }
}

impl<I: OverlayInt, S: IntShapeResource<I> + ?Sized> BuildOutlineOverlay<I> for S {}

/// Builds one outline overlay from paths consumed once, retaining only the current path.
pub(crate) fn build_outline_overlay_iter<I, Paths, Path>(
    paths: Paths,
    style: &IntOutlineStyle<I>,
    options: IntOverlayOptions<I::WideUInt>,
) -> Overlay<I>
where
    I: OverlayInt,
    Paths: IntoIterator<Item = Path>,
    Path: IntoIterator<Item = IntPoint<I>>,
{
    #[cfg(debug_assertions)]
    let padding = Join::<I>::padding(style.join, style.outer_offset)
        .max(Join::<I>::padding(style.join, style.inner_offset));

    build_outline_overlay_with_builders(
        paths,
        options,
        OutlineBuilder::new(style.outer_offset, Join::<I>::new(style.join)),
        OutlineBuilder::new(style.inner_offset, Join::<I>::new(style.join)),
        |point| {
            #[cfg(debug_assertions)]
            debug_assert!(
                expanded_is_safe(IntRect::with_point(point), padding),
                "outline bounds exceed the safe coordinate range"
            );
        },
    )
}

fn build_outline_overlay_with_builders<I, Paths, Path, J, F>(
    paths: Paths,
    options: IntOverlayOptions<I::WideUInt>,
    mut outer_builder: OutlineBuilder<I, J>,
    mut inner_builder: OutlineBuilder<I, J>,
    mut validate_point: F,
) -> Overlay<I>
where
    I: OverlayInt,
    Paths: IntoIterator<Item = Path>,
    Path: IntoIterator<Item = IntPoint<I>>,
    J: JoinBuilder<I>,
    F: FnMut(IntPoint<I>),
{
    let mut overlay = Overlay::new_custom(0, options, Default::default());
    let mut contour_options = options;

    // Contours below the threshold can join into a larger surviving result.
    contour_options.min_output_area = I::WideUInt::ZERO;

    let mut contour_overlay = Overlay::new_custom(0, contour_options, Default::default());
    let mut segments = Vec::new();
    let mut extraction = BooleanExtractionBuffer::default();
    let mut contours = FlatContoursBuffer::default();
    let mut path_buffer = Vec::new();

    for path in paths {
        path_buffer.clear();
        path_buffer.extend(path.into_iter().inspect(|point| validate_point(*point)));
        let path = path_buffer.as_slice();
        if path.len() < 3 {
            continue;
        }
        let area = path.area_two();
        if area == I::Wide::ZERO {
            continue;
        }
        let (builder, direction, fill) = if area > I::Wide::ZERO {
            (
                &mut outer_builder,
                ContourDirection::CounterClockwise,
                FillRule::Positive,
            )
        } else {
            (
                &mut inner_builder,
                ContourDirection::Clockwise,
                FillRule::Negative,
            )
        };
        segments.clear();
        builder.build(path, &mut segments);

        contour_overlay.options.output_direction = direction;
        contour_overlay.clear();
        contour_overlay.add_segments(&segments);
        if let Some(graph) = contour_overlay.build_graph_view(fill) {
            graph.extract_contours_into(OverlayRule::Subject, &mut extraction, &mut contours);
            overlay.add_flat_buffer(&contours, ShapeType::Subject);
        }
    }
    overlay
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::int::outline::offset::IntOutlineOffset;
    use alloc::vec;
    use core::cell::Cell;

    #[test]
    fn one_pass_paths_match_resource_outlines() {
        let paths = vec![
            vec![],
            vec![IntPoint::new(0, 0), IntPoint::new(0, 0)],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(0, 0),
                IntPoint::new(8192, 0),
                IntPoint::new(8192, 8192),
                IntPoint::new(0, 8192),
            ],
            vec![
                IntPoint::new(2048, 2048),
                IntPoint::new(2048, 6144),
                IntPoint::new(6144, 6144),
                IntPoint::new(6144, 2048),
            ],
        ];
        let style = IntOutlineStyle::new(1024);
        let expected = paths.outline(&style).unwrap();

        let visited = Cell::new(0);
        let visited_ref = &visited;
        let mut expected_visited = 0;
        let iter = paths.clone().into_iter().map(move |path| {
            assert_eq!(visited_ref.get(), expected_visited);
            expected_visited += path.len();
            let mut points = path.into_iter();
            core::iter::from_fn(move || {
                let point = points.next()?;
                visited_ref.set(visited_ref.get() + 1);
                Some(point)
            })
        });

        let actual = build_outline_overlay_iter(iter, &style, Default::default())
            .overlay(OverlayRule::Subject, FillRule::Positive);
        assert_eq!(actual, expected);
        assert_eq!(visited.get(), paths.iter().map(Vec::len).sum::<usize>());
    }
}
