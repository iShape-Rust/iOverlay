use super::builder::StrokeBuilder;
use crate::core::{
    fill_rule::FillRule,
    integer::OverlayInt,
    overlay::{IntOverlayOptions, Overlay},
    overlay_rule::OverlayRule,
};
use crate::mesh::int::outline::offset::IntOutlineError;
use crate::mesh::int::style::IntStrokeStyle;
use alloc::vec::Vec;
use i_float::int::rect::IntRect;
use i_shape::{
    flat::buffer::FlatContoursBuffer, int::shape::IntShapes, source::int::resource::IntShapeResource,
};

pub type IntStrokeError = IntOutlineError;

/// Strokes integer paths with radius `ceil(max(width, 0) / 2)`.
/// Radii at most one produce no geometry.
/// Coordinates, caps and temporary joins must stay in the engine's safe range.
/// Validation is optional in release and asserted in debug builds.
///
/// ```
/// use i_float::int::point::IntPoint;
/// use i_overlay::mesh::int::{stroke::offset::IntStrokeOffset, style::IntStrokeStyle};
/// let path = [IntPoint::new(0, 0), IntPoint::new(8192, 0)];
/// let style = IntStrokeStyle::new(2048);
/// path.validate_stroke(&style).unwrap();
/// assert_eq!(path.stroke(&style, false).unwrap().len(), 1);
/// ```
pub trait IntStrokeOffset<I: OverlayInt>: IntShapeResource<I> {
    fn validate_stroke(&self, style: &IntStrokeStyle<I>) -> Result<(), IntStrokeError> {
        if let Some(rect) = IntRect::with_iter(self.iter_paths().flatten()) {
            if !crate::mesh::int::bounds::expanded_is_safe(rect, StrokeBuilder::padding(style)) {
                return Err(IntStrokeError::CoordinateOutOfRange);
            }
        }
        Ok(())
    }
    fn stroke(&self, style: &IntStrokeStyle<I>, closed: bool) -> Result<IntShapes<I>, IntStrokeError> {
        self.stroke_custom(style, closed, Default::default())
    }
    fn stroke_into(
        &self,
        style: &IntStrokeStyle<I>,
        closed: bool,
        output: &mut FlatContoursBuffer<I>,
    ) -> Result<(), IntStrokeError> {
        self.stroke_custom_into(style, closed, Default::default(), output)
    }
    fn stroke_custom(
        &self,
        style: &IntStrokeStyle<I>,
        closed: bool,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Result<IntShapes<I>, IntStrokeError> {
        Ok(self
            .build_stroke_overlay(style, closed, options)
            .overlay(OverlayRule::Subject, FillRule::Positive))
    }
    fn stroke_custom_into(
        &self,
        style: &IntStrokeStyle<I>,
        closed: bool,
        options: IntOverlayOptions<I::WideUInt>,
        output: &mut FlatContoursBuffer<I>,
    ) -> Result<(), IntStrokeError> {
        self.build_stroke_overlay(style, closed, options).overlay_into(
            OverlayRule::Subject,
            FillRule::Positive,
            output,
        );
        Ok(())
    }
}
impl<I: OverlayInt, S: IntShapeResource<I> + ?Sized> IntStrokeOffset<I> for S {}

trait BuildStrokeOverlay<I: OverlayInt>: IntStrokeOffset<I> {
    fn build_stroke_overlay(
        &self,
        style: &IntStrokeStyle<I>,
        closed: bool,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Overlay<I> {
        debug_assert!(
            self.validate_stroke(style).is_ok(),
            "stroke bounds exceed the safe coordinate range"
        );
        let mut builder = StrokeBuilder::new(style);
        let mut segments = Vec::new();
        for path in self.iter_paths() {
            builder.build(path, closed, &mut segments);
        }
        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options;
        overlay
    }
}
impl<I: OverlayInt, S: IntStrokeOffset<I> + ?Sized> BuildStrokeOverlay<I> for S {}
