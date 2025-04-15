use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::Paths;
use i_shape::float::adapter::ShapeToFloat;
use crate::core::fill_rule::FillRule;
use crate::core::solver::Solver;
use crate::float::source::resource::OverlayResource;
use crate::float::string_graph::FloatStringGraph;
use crate::string::clip::ClipRule;
use crate::string::overlay::StringOverlay;

/// The `FloatStringOverlay` struct is a builder for overlaying geometric shapes by converting
/// floating-point geometry to integer space. It provides methods for adding paths and shapes,
/// as well as for converting the overlay into a `FloatStringGraph`.
#[derive(Clone)]
pub struct FloatStringOverlay<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) overlay: StringOverlay,
    pub(super) adapter: FloatPointAdapter<P, T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> FloatStringOverlay<P, T> {
    /// Constructs a new `FloatStringOverlay`, a builder for overlaying geometric shapes
    /// by converting float-based geometry to integer space, using a pre-configured adapter.
    ///
    /// - `adapter`: A `FloatPointAdapter` instance responsible for coordinate conversion between
    ///   float and integer values, ensuring accuracy during geometric transformations.
    /// - `capacity`: Initial capacity for storing segments, ideally matching the total number of
    ///   segments for efficient memory allocation.
    #[inline]
    pub fn with_adapter(adapter: FloatPointAdapter<P, T>, capacity: usize) -> Self {
        Self { overlay: StringOverlay::new(capacity), adapter }
    }

    /// Creates a new `FloatOverlay` instance and initializes it with subject and clip shapes.
    /// - `shape`: A `OverlayResource` define the shape.
    ///   `OverlayResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `string`: A `OverlayResource` define the string paths.
    ///   `OverlayResource` can be one of the following:
    ///     - `Path`: A path representing a string line.
    ///     - `Paths`: A collection of paths, each representing a string line.
    ///     - `Vec<Paths>`: A collection of grouped paths, where each group may consist of multiple paths.
    pub fn with_shape_and_string<R0, R1>(shape: &R0, string: &R1) -> Self
    where
        R0: OverlayResource<P, T>,
        R1: OverlayResource<P, T>,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let iter = shape.iter_paths().chain(string.iter_paths()).flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let shape_capacity = shape.iter_paths().fold(0, |s, c| s + c.len());
        let string_capacity = string.iter_paths().fold(0, |s, c| s + c.len());

        Self::with_adapter(adapter, shape_capacity + string_capacity)
            .unsafe_add_shapes(shape)
            .unsafe_add_string_lines(string)
    }

    /// Adds a shapes to the overlay.
    /// - `source`: A `OverlayResource` that define shape.
    ///   `OverlayResource` can be one of the following:
    ///     - `Contour`: A contour representing a closed path. This path is interpreted as closed, so it doesn’t require the start and endpoint to be the same for processing.
    ///     - `Contours`: A collection of contours, each representing a closed path.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn unsafe_add_shapes<S: OverlayResource<P, T>>(mut self, source: &S) -> Self {
        for contour in source.iter_paths() {
            self = self.unsafe_add_shape_contour(contour);
        }
        self
    }

    /// Adds a string line paths to the overlay.
    /// - `resource`: A `OverlayResource` that define shape.
    ///   `OverlayResource` can be one of the following:
    ///     - `Path`: A path representing a string line.
    ///     - `Paths`: A collection of paths, each representing a string line.
    ///     - `Vec<Paths>`: A collection of grouped paths, where each group may consist of multiple paths.
    #[inline]
    pub fn unsafe_add_string_lines<S: OverlayResource<P, T>>(mut self, resource: &S) -> Self {
        for path in resource.iter_paths() {
            self = self.unsafe_add_string_line(path);
        }
        self
    }

    /// Adds a closed shape path to the overlay.
    /// - `contour`: An array of points that form a closed path.
    /// - **Safety**: Marked `unsafe` because it assumes the path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_shape_contour(mut self, contour: &[P]) -> Self {
        self.overlay.add_shape_contour_iter(contour.iter().map(|p| self.adapter.float_to_int(p)));
        self
    }

    /// Adds an open string line path to the overlay.
    /// - `path`: A path representing a string line.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_string_line(mut self, path: &[P]) -> Self {
        for w in path.windows(2) {
            let a = self.adapter.float_to_int(&w[0]);
            let b = self.adapter.float_to_int(&w[1]);
            self.overlay.add_string_line([a, b]);
        }

        self
    }

    /// Converts the current overlay into an `FloatStringGraph` based on the specified fill rule.
    /// The resulting graph is the foundation for performing boolean operations, and it's optimized for such operations based on the provided fill rule.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - Returns: A `FloatStringGraph` containing the graph representation of the overlay's geometry.
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> FloatStringGraph<P, T> {
        self.into_graph_with_solver(fill_rule, Default::default())
    }

    /// Converts the current overlay into an `FloatStringGraph` based on the specified fill rule and solver.
    /// This method allows for finer control over the boolean operation process by passing a custom solver.
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `solver`: A custom solver for optimizing or modifying the graph creation process.
    /// - Returns: A `FloatStringGraph` containing the graph representation of the overlay's geometry.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> FloatStringGraph<P, T> {
        let graph = self.overlay.into_graph_with_solver(fill_rule, solver);
        FloatStringGraph { graph, adapter: self.adapter }
    }

    /// Executes a single Boolean operation on the current geometry using the specified fill and clip rules.
    ///
    /// ### Parameters:
    /// - `fill_rule`: Fill rule to determine filled areas (non-zero, even-odd, positive, negative).
    /// - `clip_rule`: Clip rule to determine how the boundary and inversion settings affect the result.
    /// - `solver`: Type of solver to use.
    /// - Returns: A `Paths<P>` collection of string lines that meet the clipping conditions.
    #[inline]
    pub fn clip_string_lines_with_solver(self, fill_rule: FillRule, clip_rule: ClipRule, solver: Solver) -> Paths<P> {
        let paths = self.overlay.clip_string_lines_with_solver(fill_rule, clip_rule, solver);
        paths.to_float(&self.adapter)
    }
}