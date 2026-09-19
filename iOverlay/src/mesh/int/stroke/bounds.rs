use super::offset::IntStrokeError;
use crate::core::integer::OverlayInt;
use crate::mesh::int::{
    join::Join,
    math::{backend::MeshMath, float::FloatMath, integer::IntegerMath},
    style::{IntLineCap, IntStrokeStyle},
};
use crate::mesh::math::MathMode;
use i_float::int::{
    number::{int::IntNumber, uint::UIntNumber, wide_int::WideIntNumber},
    rect::IntRect,
};
use i_shape::source::int::resource::IntShapeResource;

pub(super) trait StrokeBounds<I: OverlayInt>: IntShapeResource<I> {
    fn validate_stroke_bounds(&self, style: &IntStrokeStyle<I>) -> Result<(), IntStrokeError> {
        if let Some(rect) = IntRect::with_iter(self.iter_paths().flatten())
            && !crate::mesh::int::bounds::expanded_is_safe(rect, stroke_padding(style))
        {
            return Err(IntStrokeError::CoordinateOutOfRange);
        }
        Ok(())
    }
}

impl<I: OverlayInt, S: IntShapeResource<I> + ?Sized> StrokeBounds<I> for S {}

fn stroke_padding<I: OverlayInt>(style: &IntStrokeStyle<I>) -> I::Wide {
    match style.math {
        MathMode::Integer => stroke_padding_with_math::<I, IntegerMath>(style),
        MathMode::Float => stroke_padding_with_math::<I, FloatMath>(style),
    }
}

pub(super) fn stroke_radius<I: IntNumber>(style: &IntStrokeStyle<I>) -> I {
    I::from_wide((style.width.max(I::ZERO).to_wide() + I::Wide::ONE) / I::Wide::TWO)
}
pub(super) fn stroke_padding_with_math<I: IntNumber, M: MeshMath<I>>(style: &IntStrokeStyle<I>) -> I::Wide {
    let radius = stroke_radius(style);
    let cap_padding = |cap: &IntLineCap<I>| match cap {
        IntLineCap::Square => radius.to_wide() * I::Wide::TWO,
        IntLineCap::Custom(points) => points
            .iter()
            .map(|p| {
                let d = i_float::int::vector::IntVector::<I>::new(p.x.to_wide(), p.y.to_wide()).sqr_length();
                let root = d.isqrt();
                I::Wide::from_uint(root)
                    + if root * root < d {
                        I::Wide::ONE
                    } else {
                        I::Wide::ZERO
                    }
            })
            .max()
            .unwrap_or(I::Wide::ZERO),
        _ => radius.to_wide(),
    };
    M::guard_padding(
        Join::<I, M>::padding(style.join, radius)
            .max(cap_padding(&style.start_cap))
            .max(cap_padding(&style.end_cap)),
    )
}
