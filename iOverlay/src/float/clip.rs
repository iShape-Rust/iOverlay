use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Paths;
use i_shape::source::resource::ShapeResource;
use crate::core::fill_rule::FillRule;
use crate::core::solver::Solver;
use crate::float::string_overlay::FloatStringOverlay;
use crate::string::clip::ClipRule;

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
    fn clip_by_with_solver(&self, source: &R, fill_rule: FillRule, clip_rule: ClipRule, solver: Solver) -> Paths<P>;
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
    fn clip_by_with_solver(&self, resource: &R0, fill_rule: FillRule, clip_rule: ClipRule, solver: Solver) -> Paths<P> {
        FloatStringOverlay::with_shape_and_string(resource, self)
            .clip_string_lines_with_solver(fill_rule, clip_rule, solver)
    }
}
