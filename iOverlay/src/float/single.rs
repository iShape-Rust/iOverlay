use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use i_shape::source::resource::ShapeResource;
use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::FloatOverlay;

/// Trait `SingleFloatOverlay` provides methods for overlay operations between various geometric entities.
/// This trait supports boolean operations on contours, shapes, and collections of shapes, using customizable overlay and build rules.
pub trait SingleFloatOverlay<R0, R1, P, T>
where
    R0: ShapeResource<P, T>,
    R1: ShapeResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// General overlay method that takes an `ShapeResource` to determine the input type.
    ///
    /// - `resource`: A `ShapeResource` specifying the type of geometric entity to overlay with.
    ///   It can be one of the following:
    ///     - `Contour`: A single contour representing a path or boundary.
    ///     - `Contours`: A collection of contours, each defining separate boundaries.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - Returns: A vector of `Shapes<P>` representing the cleaned-up geometric result.
    fn overlay(&self, source: &R1, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P>;
}

impl<R0, R1, P, T> SingleFloatOverlay<R0, R1, P, T> for R0
where
    R0: ShapeResource<P, T>,
    R1: ShapeResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn overlay(&self, resource: &R1, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        FloatOverlay::with_subj_and_clip(self, resource).overlay(overlay_rule, fill_rule)
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay_rule::OverlayRule;
    use crate::float::overlay::FloatOverlay;
    use crate::float::single::SingleFloatOverlay;

    #[test]
    fn test_contour() {
        let left_rect = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let right_rect = vec![[1.0, 0.0], [1.0, 1.0], [2.0, 1.0], [2.0, 0.0]];

        let shapes = left_rect
            .overlay(&right_rect, OverlayRule::Union, FillRule::EvenOdd);

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