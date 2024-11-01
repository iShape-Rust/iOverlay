use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Paths;
use i_shape::float::adapter::PathToFloat;
use crate::core::fill_rule::FillRule;
use crate::float::source::resource::OverlayResource;
use crate::float::string_graph::FloatStringGraph;
use crate::float::string_overlay::FloatStringOverlay;
use crate::string::clip::ClipRule;

impl<P, T> FloatStringGraph<P, T>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Clips the line strings in the graph based on the specified `ClipRule`.
    ///
    /// - `clip_rule`: Clip rule to determine how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    #[inline]
    pub fn clip_string_lines(&self, clip_rule: ClipRule) -> Paths<P> {
        let lines = self.graph.clip_string_lines(clip_rule);
        lines.into_iter().map(|path| path.to_float(&self.adapter)).collect()
    }
}

pub trait FloatClip<R, P, T>
where
    R: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Clips paths according to the specified fill and clip rules.
    /// - `resource`: A clipping shape.
    ///   `OverlayResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `clip_rule`: Clip rule to determine how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip_by(&self, source: &R, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P>;
}

impl<R0, R1, P, T> FloatClip<R0, P, T> for R1
where
    R0: OverlayResource<P, T>,
    R1: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn clip_by(&self, resource: &R0, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        FloatStringOverlay::with_shape_and_string(resource, self)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }
}
