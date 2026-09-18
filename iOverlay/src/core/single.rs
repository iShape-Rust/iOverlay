//! One-shot Boolean operations on integer shape resources.
use crate::core::fill_rule::FillRule;
use crate::core::integer::OverlayInt;
use crate::core::overlay::Overlay;
use crate::core::overlay_rule::OverlayRule;
use i_shape::int::shape::IntShapes;
use i_shape::source::int::resource::IntShapeResource;

/// Boolean operations between resources with independently chosen storage types.
/// Each input path is interpreted as a closed contour.
///
/// ```
/// use i_overlay::core::{fill_rule::FillRule, overlay_rule::OverlayRule, single::SingleIntOverlay};
/// use i_overlay::i_float::int::point::IntPoint;
///
/// let square = [IntPoint::new(0, 0), IntPoint::new(10, 0),
///               IntPoint::new(10, 10), IntPoint::new(0, 10)];
/// let result = square.overlay(&square[..], OverlayRule::Intersect, FillRule::NonZero);
/// assert_eq!(result.len(), 1);
/// ```
pub trait SingleIntOverlay<R, I>
where
    R: IntShapeResource<I> + ?Sized,
    I: OverlayInt,
{
    /// Applies a Boolean operation using the supplied fill rule for both resources.
    fn overlay(&self, source: &R, overlay_rule: OverlayRule, fill_rule: FillRule) -> IntShapes<I>;
}

impl<R0, R1, I> SingleIntOverlay<R1, I> for R0
where
    R0: IntShapeResource<I> + ?Sized,
    R1: IntShapeResource<I> + ?Sized,
    I: OverlayInt,
{
    #[inline]
    fn overlay(&self, source: &R1, overlay_rule: OverlayRule, fill_rule: FillRule) -> IntShapes<I> {
        Overlay::from_subj_and_clip(self, source).overlay(overlay_rule, fill_rule)
    }
}
