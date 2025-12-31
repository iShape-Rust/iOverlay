use crate::core::fill_rule::FillRule;
use crate::core::solver::Solver;
use crate::float::scale::FixedScaleOverlayError;
use crate::float::string_overlay::FloatStringOverlay;
use crate::string::clip::ClipRule;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Paths;
use i_shape::source::resource::ShapeResource;

pub trait FloatClip<R, P, T>
where
    R: ShapeResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Clips paths according to the specified build and clip rules.
    /// - `resource`: A clipping shape.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `clip_rule`: Clip rule to determine how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip_by(&self, source: &R, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P>;

    /// Clips paths according to the specified build and clip rules using a fixed float-to-integer scale.
    /// - `resource`: A clipping shape.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `clip_rule`: Clip rule to determine how boundary and inversion settings affect the result.
    /// - `scale`: Fixed float-to-integer scale. Use `scale = 1.0 / grid_size` if you prefer grid size semantics.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip_by_fixed_scale(
        &self,
        source: &R,
        fill_rule: FillRule,
        clip_rule: ClipRule,
        scale: T,
    ) -> Result<Paths<P>, FixedScaleOverlayError>;

    /// Clips paths according to the specified build and clip rules.
    /// - `resource`: A clipping shape.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `clip_rule`: Clip rule to determine how boundary and inversion settings affect the result.
    /// - `solver`: Type of solver to use.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip_by_with_solver(
        &self,
        source: &R,
        fill_rule: FillRule,
        clip_rule: ClipRule,
        solver: Solver,
    ) -> Paths<P>;

    /// Clips paths according to the specified build and clip rules using a fixed float-to-integer scale.
    /// - `resource`: A clipping shape.
    ///   `ShapeResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `clip_rule`: Clip rule to determine how boundary and inversion settings affect the result.
    /// - `solver`: Type of solver to use.
    /// - `scale`: Fixed float-to-integer scale. Use `scale = 1.0 / grid_size` if you prefer grid size semantics.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip_by_fixed_scale_with_solver(
        &self,
        source: &R,
        fill_rule: FillRule,
        clip_rule: ClipRule,
        solver: Solver,
        scale: T,
    ) -> Result<Paths<P>, FixedScaleOverlayError>;
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::float::clip::FloatClip;
    use crate::string::clip::ClipRule;
    use alloc::vec;

    #[test]
    fn test_clip_fixed_scale_ok() {
        let shape = vec![[0.0, 0.0], [0.0, 2.0], [2.0, 2.0], [2.0, 0.0]];
        let string = vec![[-1.0, 1.0], [3.0, 1.0]];

        let clip_rule = ClipRule {
            invert: false,
            boundary_included: false,
        };
        let paths = string
            .clip_by_fixed_scale(&shape, FillRule::EvenOdd, clip_rule, 10.0)
            .unwrap();

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], [[0.0, 1.0], [2.0, 1.0]]);
    }

    #[test]
    fn test_clip_fixed_scale_invalid() {
        let shape = vec![[0.0, 0.0], [0.0, 2.0], [2.0, 2.0], [2.0, 0.0]];
        let string = vec![[-1.0, 1.0], [3.0, 1.0]];

        let clip_rule = ClipRule {
            invert: false,
            boundary_included: false,
        };
        assert!(
            string
                .clip_by_fixed_scale(&shape, FillRule::EvenOdd, clip_rule, 0.0)
                .is_err()
        );
        assert!(
            string
                .clip_by_fixed_scale(&shape, FillRule::EvenOdd, clip_rule, -1.0)
                .is_err()
        );
        assert!(
            string
                .clip_by_fixed_scale(&shape, FillRule::EvenOdd, clip_rule, f64::NAN)
                .is_err()
        );
        assert!(
            string
                .clip_by_fixed_scale(&shape, FillRule::EvenOdd, clip_rule, f64::INFINITY)
                .is_err()
        );
    }
}

impl<R0, R1, P, T> FloatClip<R0, P, T> for R1
where
    R0: ShapeResource<P, T>,
    R1: ShapeResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn clip_by(&self, resource: &R0, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        self.clip_by_with_solver(resource, fill_rule, clip_rule, Default::default())
    }

    #[inline]
    fn clip_by_with_solver(
        &self,
        resource: &R0,
        fill_rule: FillRule,
        clip_rule: ClipRule,
        solver: Solver,
    ) -> Paths<P> {
        FloatStringOverlay::with_shape_and_string(resource, self)
            .clip_string_lines_with_solver(fill_rule, clip_rule, solver)
    }

    #[inline]
    fn clip_by_fixed_scale(
        &self,
        resource: &R0,
        fill_rule: FillRule,
        clip_rule: ClipRule,
        scale: T,
    ) -> Result<Paths<P>, FixedScaleOverlayError> {
        self.clip_by_fixed_scale_with_solver(
            resource,
            fill_rule,
            clip_rule,
            Default::default(),
            scale,
        )
    }

    #[inline]
    fn clip_by_fixed_scale_with_solver(
        &self,
        resource: &R0,
        fill_rule: FillRule,
        clip_rule: ClipRule,
        solver: Solver,
        scale: T,
    ) -> Result<Paths<P>, FixedScaleOverlayError> {
        Ok(
            FloatStringOverlay::with_shape_and_string_fixed_scale(resource, self, scale)?
                .clip_string_lines_with_solver(fill_rule, clip_rule, solver),
        )
    }
}
