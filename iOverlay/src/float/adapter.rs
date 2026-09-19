use super::scale::FixedScaleOverlayError;
use crate::core::integer::OverlayInt;
use i_float::adapter::FloatPointAdapter;
use i_float::float::{compatible::FloatPointCompatible, rect::FloatRect};

// Reserve enough bits for the engine's coordinate differences and products,
// including rounding at the bounds.
pub(crate) fn adapter_with_iter<'a, P, I>(iter: impl Iterator<Item = &'a P>) -> FloatPointAdapter<P, I>
where
    P: FloatPointCompatible + 'a,
    I: OverlayInt,
{
    let rect = FloatRect::with_iter(iter)
        .expect("Invalid overlay bounds")
        .unwrap_or(FloatRect::zero());
    FloatPointAdapter::with_coordinate_bits(rect, I::BITS - 3)
}

pub(crate) fn adapter_with_iter_and_scale<'a, P, I>(
    iter: impl Iterator<Item = &'a P>,
    scale: P::Scalar,
) -> Result<FloatPointAdapter<P, I>, FixedScaleOverlayError>
where
    P: FloatPointCompatible + 'a,
    I: OverlayInt,
{
    let rect = FloatRect::with_iter(iter)?.unwrap_or(FloatRect::zero());
    Ok(FloatPointAdapter::try_with_scale_and_coordinate_bits(
        rect,
        scale,
        I::BITS - 3,
    )?)
}
