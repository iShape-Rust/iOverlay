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
    /// - `clip_rule`: The clipping rule specifying whether to invert the selection and include boundaries.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    #[inline]
    pub fn clip_string_lines(&self, clip_rule: ClipRule) -> Paths<P> {
        let lines = self.graph.clip_string_lines(clip_rule);
        lines.into_iter().map(|path| path.to_float(&self.adapter)).collect()
    }
}

pub trait FloatClip<S, P, T>
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    /// Clips a paths according to the specified fill and clip rules.
    /// - `source`: A source for clipping shapes.
    ///   `ContourSource` can be one of the following:
    ///     - `Path`: A single open path.
    ///     - `Paths`: An array of open paths.
    ///     - `Shapes`: A two-dimensional array where each element defines a separate open path.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of path segments.
    /// - `clip_rule`: The rule for clipping, determining how boundary and inversion settings affect the result.
    ///
    /// # Returns
    /// A `Paths<P>` collection of string lines that meet the clipping conditions.
    fn clip(&self, source: &S, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P>;
}

impl<S, P, T> FloatClip<S, P, T> for S
where
    S: OverlayResource<P, T>,
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    #[inline]
    fn clip(&self, source: &S, fill_rule: FillRule, clip_rule: ClipRule) -> Paths<P> {
        FloatStringOverlay::with_shape_and_string(source, self)
            .into_graph(fill_rule)
            .clip_string_lines(clip_rule)
    }
}
