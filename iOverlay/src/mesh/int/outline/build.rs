use super::bounds;
use super::builder::OutlineBuilder;
use super::builder_join::JoinBuilder;
use super::offset::IntOutlineError;
use crate::core::extract::BooleanExtractionBuffer;
use crate::core::fill_rule::FillRule;
use crate::core::integer::OverlayInt;
use crate::core::overlay::{ContourDirection, IntOverlayOptions, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::mesh::int::join::Join;
use crate::mesh::int::style::IntOutlineStyle;
use alloc::vec::Vec;
use i_float::int::number::uint::UIntNumber;
use i_float::int::number::wide_int::WideIntNumber;
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
            bounds::validate(self, style).is_ok(),
            "outline bounds exceed the safe coordinate range"
        );
        Ok(self.build_overlay_with_builders(
            options,
            OutlineBuilder::new(style.outer_offset, Join::new(style.join)),
            OutlineBuilder::new(style.inner_offset, Join::new(style.join)),
        ))
    }

    fn build_overlay_with_builders<J: JoinBuilder<I>>(
        &self,
        options: IntOverlayOptions<I::WideUInt>,
        mut outer_builder: OutlineBuilder<I, J>,
        mut inner_builder: OutlineBuilder<I, J>,
    ) -> Overlay<I> {
        let mut overlay = Overlay::new_custom(0, options, Default::default());
        let mut contour_options = options;

        // Contours below the threshold can join into a larger surviving result.
        contour_options.min_output_area = I::WideUInt::ZERO;

        let mut contour_overlay = Overlay::new_custom(0, contour_options, Default::default());
        let mut segments = Vec::new();
        let mut extraction = BooleanExtractionBuffer::default();
        let mut contours = FlatContoursBuffer::default();

        for path in self.iter_paths() {
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
}

impl<I: OverlayInt, S: IntShapeResource<I> + ?Sized> BuildOutlineOverlay<I> for S {}
