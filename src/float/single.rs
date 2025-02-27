use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Shapes;
use crate::core::fill_rule::FillRule;
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::float::filter::ContourFilter;
use crate::float::overlay::FloatOverlay;
use crate::float::source::resource::OverlayResource;

/// Trait `SingleFloatOverlay` provides methods for overlay operations between various geometric entities.
/// This trait supports boolean operations on contours, shapes, and collections of shapes, using customizable overlay and fill rules.
pub trait SingleFloatOverlay<R0, R1, P, T>
where
    R0: OverlayResource<P, T>,
    R1: OverlayResource<P, T>,
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
    fn overlay(&self, source: &R1, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P>;

    /// General overlay method that takes an `OverlayResource` to determine the input type.
    ///
    /// - `resource`: A `OverlayResource` specifying the type of geometric entity to overlay with.
    ///   It can be one of the following:
    ///     - `Contour`: A single contour representing a path or boundary.
    ///     - `Contours`: A collection of contours, each defining separate boundaries.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `filter`: `ContourFilter<T>` for optional contour filtering and simplification:
    ///     - `min_area`: Only retain contours with an area larger than this.
    ///     - `simplify`: Simplifies contours and removes degenerate edges if `true`.
    /// - `solver`: Type of solver to use.
    /// - Returns: A vector of `Shapes<P>` representing the cleaned-up geometric result.
    fn overlay_with_filter_and_solver(&self, source: &R1, overlay_rule: OverlayRule, fill_rule: FillRule, filter: ContourFilter<T>, solver: Solver) -> Shapes<P>;
}

impl<R0, R1, P, T> SingleFloatOverlay<R0, R1, P, T> for R0
where
    R0: OverlayResource<P, T>,
    R1: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn overlay(&self, resource: &R1, overlay_rule: OverlayRule, fill_rule: FillRule) -> Shapes<P> {
        self.overlay_with_filter_and_solver(
            resource,
            overlay_rule,
            fill_rule,
            Default::default(),
            Default::default()
        )
    }

    #[inline]
    fn overlay_with_filter_and_solver(&self, resource: &R1, overlay_rule: OverlayRule, fill_rule: FillRule, filter: ContourFilter<T>, solver: Solver) -> Shapes<P> {
        FloatOverlay::with_subj_and_clip(self, resource).overlay_with_filter_and_solver(overlay_rule, fill_rule, filter, solver)
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