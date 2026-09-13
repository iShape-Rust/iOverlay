//! Integer outline construction used directly and by the float adapter.
use super::bounds;
use super::build::BuildOutlineOverlay;
use crate::core::fill_rule::FillRule;
use crate::core::integer::OverlayInt;
use crate::core::overlay::IntOverlayOptions;
use crate::core::overlay_rule::OverlayRule;
use crate::mesh::int::style::IntOutlineStyle;
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::int::shape::IntShapes;
use i_shape::source::int::resource::IntShapeResource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntOutlineError {
    /// The conservative expanded input bounds fail the coordinate-range check.
    /// Returned by optional mesh validation, not by construction.
    CoordinateOutOfRange,
}

impl core::fmt::Display for IntOutlineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CoordinateOutOfRange => f.write_str("mesh coordinate exceeds the integer engine range"),
        }
    }
}

impl core::error::Error for IntOutlineError {}

/// Offsets integer contours with CCW outer boundaries and CW holes.
///
/// Positive offsets expand filled geometry; negative offsets shrink it. Zero
/// offset still runs contour cleanup and union. Degenerate zero-area contours
/// are ignored. As with float outline, input winding determines outer/hole roles.
/// Input and constructed coordinates must satisfy
/// [`IntRect::is_in_safe_range`](i_float::int::rect::IntRect::is_in_safe_range).
/// Use [`Self::validate_outline`] for an optional conservative bounds check.
/// Construction checks this precondition only in debug builds; release builds
/// trust the caller. Contour signed double areas must also fit in `I::Wide`
/// (including repeated winding).
///
/// Uses [`fast_normalize`](i_float::int::vector::IntVector::fast_normalize)
/// with about 6/14/30 bits of direction precision for i16/i32/i64.
/// Scaling a direction also scales its approximation error.
///
/// ```
/// use i_overlay::i_float::int::point::IntPoint;
/// use i_overlay::mesh::int::style::IntOutlineStyle;
/// use i_overlay::mesh::int::outline::offset::IntOutlineOffset;
///
/// let contour = [
///     IntPoint::new(0_i32, 0), IntPoint::new(16_384, 0),
///     IntPoint::new(16_384, 16_384), IntPoint::new(0, 16_384),
/// ];
/// let style = IntOutlineStyle::new(1_024);
/// contour.validate_outline(&style)?; // Optional bounds check before construction.
/// let result = contour.outline(&style)?;
/// assert_eq!(result.len(), 1);
/// # Ok::<(), i_overlay::mesh::int::outline::offset::IntOutlineError>(())
/// ```
pub trait IntOutlineOffset<I: OverlayInt>: IntShapeResource<I> {
    /// Checks the coordinate range of the prospective operation.
    ///
    /// Computes input bounds, expands by the maximum outer/inner join padding,
    /// and checks `IntRect::is_in_safe_range`. Padding uses absolute offsets even
    /// for shrinking outlines, since temporary edges must also stay in range.
    /// The estimate is conservative and may reject geometry whose actual points
    /// would fit. Empty input passes. This does not validate
    /// winding, topology, or the accumulated area of repeated winding.
    fn validate_outline(&self, style: &IntOutlineStyle<I>) -> Result<(), IntOutlineError> {
        bounds::validate(self, style)
    }

    fn outline(&self, style: &IntOutlineStyle<I>) -> Result<IntShapes<I>, IntOutlineError> {
        self.outline_custom(style, Default::default())
    }

    /// Replaces output on success, including an empty result. Errors leave it unchanged.
    fn outline_into(
        &self,
        style: &IntOutlineStyle<I>,
        output: &mut FlatContoursBuffer<I>,
    ) -> Result<(), IntOutlineError> {
        self.outline_custom_into(style, Default::default(), output)
    }

    fn outline_custom(
        &self,
        style: &IntOutlineStyle<I>,
        options: IntOverlayOptions<I::WideUInt>,
    ) -> Result<IntShapes<I>, IntOutlineError> {
        let mut overlay = self.build_overlay(style, options)?;
        Ok(overlay.overlay(OverlayRule::Subject, FillRule::Positive))
    }

    /// Replaces output on success; errors leave it unchanged.
    /// Area filtering is applied after contour union.
    fn outline_custom_into(
        &self,
        style: &IntOutlineStyle<I>,
        options: IntOverlayOptions<I::WideUInt>,
        output: &mut FlatContoursBuffer<I>,
    ) -> Result<(), IntOutlineError> {
        let mut overlay = self.build_overlay(style, options)?;
        overlay.overlay_into(OverlayRule::Subject, FillRule::Positive, output);
        Ok(())
    }
}

impl<I: OverlayInt, S: IntShapeResource<I> + ?Sized> IntOutlineOffset<I> for S {}
