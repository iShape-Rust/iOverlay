use super::offset::IntOutlineError;
use crate::mesh::int::join::Join;
use crate::mesh::int::math::{backend::MeshMath, float::FloatMath, integer::IntegerMath};
use crate::mesh::int::style::IntOutlineStyle;
use crate::mesh::math::MathMode;
use i_float::int::number::int::IntNumber;
use i_float::int::rect::IntRect;
use i_shape::source::int::resource::IntShapeResource;

pub(super) trait OutlineBounds<I: IntNumber>: IntShapeResource<I> {
    fn validate_outline_bounds(&self, style: &IntOutlineStyle<I>) -> Result<(), IntOutlineError> {
        let padding = match style.math {
            MathMode::Integer => outline_padding::<I, IntegerMath>(style),
            MathMode::Float => outline_padding::<I, FloatMath>(style),
        };
        let Some(rect) = IntRect::with_iter(self.iter_paths().flatten()) else {
            return Ok(());
        };

        if crate::mesh::int::bounds::expanded_is_safe(rect, padding) {
            Ok(())
        } else {
            Err(IntOutlineError::CoordinateOutOfRange)
        }
    }
}

impl<I: IntNumber, S: IntShapeResource<I> + ?Sized> OutlineBounds<I> for S {}

fn outline_padding<I: IntNumber, M: MeshMath<I>>(style: &IntOutlineStyle<I>) -> I::Wide {
    M::guard_padding(
        Join::<I, M>::padding(style.join, style.outer_offset)
            .max(Join::<I, M>::padding(style.join, style.inner_offset)),
    )
}
