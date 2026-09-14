use super::offset::IntOutlineError;
use crate::mesh::int::join::Join;
use crate::mesh::int::style::IntOutlineStyle;
use i_float::int::number::int::IntNumber;
use i_float::int::rect::IntRect;
use i_shape::source::int::resource::IntShapeResource;

pub(super) trait OutlineBounds<I: IntNumber>: IntShapeResource<I> {
    fn validate_outline_bounds(&self, style: &IntOutlineStyle<I>) -> Result<(), IntOutlineError> {
        let padding =
            Join::padding(style.join, style.outer_offset).max(Join::padding(style.join, style.inner_offset));
        let Some(rect) = IntRect::with_iter(self.iter_paths().flatten()) else {
            return Ok(());
        };

        // Expand in the wide type, including when an offset is I::MIN. Saturating
        // at storage limits prevents wrapping during narrowing: these limits are
        // themselves outside is_in_safe_range, so an oversized bound stays invalid.
        let min = I::MIN.to_wide();
        let max = I::MAX.to_wide();
        let expanded = IntRect::new(
            I::from_wide((rect.min_x.to_wide() - padding).max(min)),
            I::from_wide((rect.max_x.to_wide() + padding).min(max)),
            I::from_wide((rect.min_y.to_wide() - padding).max(min)),
            I::from_wide((rect.max_y.to_wide() + padding).min(max)),
        );
        if expanded.is_in_safe_range() {
            Ok(())
        } else {
            Err(IntOutlineError::CoordinateOutOfRange)
        }
    }
}

impl<I: IntNumber, S: IntShapeResource<I> + ?Sized> OutlineBounds<I> for S {}
