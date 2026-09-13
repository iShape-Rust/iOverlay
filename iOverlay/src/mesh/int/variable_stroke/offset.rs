use super::{IntVariableStrokeSource, IntVariableStrokeStyle, builder::VariableStrokeBuilder};
use crate::core::{
    fill_rule::FillRule,
    integer::OverlayInt,
    overlay::{IntOverlayOptions, Overlay},
    overlay_rule::OverlayRule,
};
use crate::mesh::int::outline::offset::IntOutlineError;
use alloc::vec::Vec;
use i_float::int::rect::IntRect;
use i_shape::{flat::buffer::FlatContoursBuffer, int::shape::IntShapes};

pub type IntVariableStrokeError = IntOutlineError;

/// Integer variable-width strokes with round caps and joins. Widths and
/// coordinates are in input units. Call validation separately when necessary;
/// release construction trusts the coordinate-range precondition.
///
/// ```
/// use i_float::int::point::IntPoint;
/// use i_overlay::mesh::int::variable_stroke::{IntStrokeVertex, IntVariableStrokeStyle,
///     offset::IntVariableStrokeOffset};
/// let path = [IntStrokeVertex::new(IntPoint::new(0, 0), 2048),
///     IntStrokeVertex::new(IntPoint::new(8192, 0), 4096)];
/// path.validate_variable_stroke().unwrap();
/// assert_eq!(path.variable_stroke(IntVariableStrokeStyle::new()).unwrap().len(), 1);
/// ```
pub trait IntVariableStrokeOffset<I: OverlayInt>: IntVariableStrokeSource<I> {
    fn validate_variable_stroke(&self) -> Result<(), IntVariableStrokeError> {
        let radius = self
            .iter_variable_paths()
            .flatten()
            .map(|v| v.radius())
            .max()
            .unwrap_or(I::ZERO);
        if let Some(rect) = IntRect::with_iter(self.iter_variable_paths().flatten().map(|v| &v.point)) {
            if !crate::mesh::int::bounds::expanded_is_safe(rect, radius.to_wide()) {
                return Err(IntVariableStrokeError::CoordinateOutOfRange);
            }
        }
        Ok(())
    }
    fn variable_stroke(&self, style: IntVariableStrokeStyle) -> Result<IntShapes<I>, IntVariableStrokeError> {
        self.variable_stroke_custom(style, Default::default())
    }
    fn variable_stroke_into(
        &self,
        style: IntVariableStrokeStyle,
        output: &mut FlatContoursBuffer<I>,
    ) -> Result<(), IntVariableStrokeError> {
        self.variable_stroke_custom_into(style, Default::default(), output)
    }
    fn variable_stroke_custom(
        &self,
        style: IntVariableStrokeStyle,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Result<IntShapes<I>, IntVariableStrokeError> {
        Ok(self
            .build_variable_overlay(style, options)
            .overlay(OverlayRule::Subject, FillRule::Positive))
    }
    fn variable_stroke_custom_into(
        &self,
        style: IntVariableStrokeStyle,
        options: IntOverlayOptions<I::WideUInt>,
        output: &mut FlatContoursBuffer<I>,
    ) -> Result<(), IntVariableStrokeError> {
        self.build_variable_overlay(style, options).overlay_into(
            OverlayRule::Subject,
            FillRule::Positive,
            output,
        );
        Ok(())
    }
    #[cfg(feature = "variable_stroke_debug")]
    fn variable_stroke_debug(
        &self,
        style: IntVariableStrokeStyle,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Result<super::debug::IntVariableStrokeDebugResult<I>, IntVariableStrokeError> {
        debug_assert!(
            self.validate_variable_stroke().is_ok(),
            "variable stroke exceeds coordinate range"
        );
        let mut builder = VariableStrokeBuilder::new(style);
        let mut segments = Vec::new();
        let mut edges = Vec::new();
        for (index, path) in self.iter_variable_paths().enumerate() {
            builder.build_debug(path, index, &mut segments, &mut edges);
        }
        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options;
        Ok(super::debug::IntVariableStrokeDebugResult {
            edges,
            shapes: overlay.overlay(OverlayRule::Subject, FillRule::Positive),
        })
    }
}
impl<I: OverlayInt, S: IntVariableStrokeSource<I> + ?Sized> IntVariableStrokeOffset<I> for S {}
trait BuildVariableOverlay<I: OverlayInt>: IntVariableStrokeOffset<I> {
    fn build_variable_overlay(
        &self,
        style: IntVariableStrokeStyle,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Overlay<I> {
        debug_assert!(
            self.validate_variable_stroke().is_ok(),
            "variable stroke exceeds coordinate range"
        );
        let mut builder = VariableStrokeBuilder::new(style);
        let mut segments = Vec::new();
        for path in self.iter_variable_paths() {
            builder.build(path, &mut segments);
        }
        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options;
        overlay
    }
}
impl<I: OverlayInt, S: IntVariableStrokeOffset<I> + ?Sized> BuildVariableOverlay<I> for S {}
