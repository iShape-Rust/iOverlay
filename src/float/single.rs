use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::FloatOverlay;
use crate::float::source::resource::OverlayResource;

/// Trait `SingleFloatOverlay` provides methods for overlay operations between various geometric entities.
/// This trait supports boolean operations on contours, shapes, and collections of shapes, using customizable overlay and fill rules.
pub trait SingleFloatOverlay<S, P, T>
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// General overlay method that takes an `OverlayResource` to determine the input type.
    ///
    /// - `resource`: A `OverlayResource` specifying the type of geometric entity to overlay with.
    ///   It can be one of the following:
    ///     - `Contour`: A single contour representing a path or boundary.
    ///     - `Contours`: A collection of contours, each defining separate boundaries.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - Returns: A vector of `Shapes<P>` representing the cleaned-up geometric result.
    fn overlay(&self, source: &S, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P>;
}

impl<R, P, T> SingleFloatOverlay<R, P, T> for R
where
    R: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn overlay(&self, resource: &R, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        FloatOverlay::with_subj_and_clip(self, resource).overlay(overlay_rule, fill_rule)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay_rule::OverlayRule;
    use crate::float::overlay::FloatOverlay;
    use crate::float::single::SingleFloatOverlay;

    #[test]
    fn test_contour() {
        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = left_rect.overlay(&right_rect, OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_contours() {
        let r3 = vec![
            vec![
                [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]
            ],
            vec![
                [0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]
            ],
            vec![
                [1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]
            ]
        ];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(&r3, &right_bottom_rect)
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_shapes() {
        let shapes = vec![
            vec![
                vec![
                    [0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]
                ],
                vec![
                    [0.0, 1.0], [0.0, 2.0], [1.0, 2.0], [1.0, 1.0]
                ],
                vec![
                    [1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0]
                ]
            ]
        ];
        let right_bottom_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = FloatOverlay::with_subj_and_clip(&shapes, &right_bottom_rect)
            .overlay(OverlayRule::Union, FillRule::EvenOdd);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }
}