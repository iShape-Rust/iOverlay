use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::core::fill_rule::FillRule;
use crate::core::solver::Solver;
use crate::float::source::ContourSource;
use crate::float::string_graph::FloatStringGraph;
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
    /// - `shape`: A `ContourSource` define the shape.
    /// - `string`: A `ContourSource` define the string paths.
    ///   `ContourSource` can be one of the following:
    ///     - `Contour`: A single contour or boundary path.
    ///     - `Contours`: A collection of contours, each defining separate boundaries.
    ///     - `Shapes`: A collection where each shape may contain multiple contours.
    pub fn with_shape_and_string<S0, S1>(shape: &S0, string: &S1) -> Self
    where
        S0: ContourSource<P, T> + ?Sized,
        S1: ContourSource<P, T> + ?Sized,
        P: FloatPointCompatible<T>,
        T: FloatNumber,
    {
        let iter = shape.iter_contours().chain(string.iter_contours()).flatten();
        let adapter = FloatPointAdapter::with_iter(iter);
        let shape_capacity = shape.iter_contours().fold(0, |s, c| s + c.len());
        let string_capacity = string.iter_contours().fold(0, |s, c| s + c.len());

        Self::with_adapter(adapter, shape_capacity + string_capacity)
            .unsafe_add_shape_source(shape)
            .unsafe_add_string_source(string)
    }

    /// Adds a shapes to the overlay.
    /// - `source`: A `ContourSource` that define shape.
    ///   `ContourSource` can be one of the following:
    ///     - `Contour`: A single contour representing a path or boundary.
    ///     - `Contours`: A collection of contours, each defining separate boundaries.
    ///     - `Shapes`: A collection of shapes, where each shape may consist of multiple contours.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn unsafe_add_shape_source<S: ContourSource<P, T> + ?Sized>(mut self, source: &S) -> Self {
        for contour in source.iter_contours() {
            self = self.unsafe_add_shape_contour(contour);
        }
        self
    }

    /// Adds a string paths to the overlay.
    /// - `source`: A `ContourSource` that define shape.
    ///   `ContourSource` can be one of the following:
    ///     - `Path`: A single open path.
    ///     - `Paths`: An array of open paths.
    ///     - `Shapes`: A two-dimensional array where each element defines a separate open path.
    #[inline]
    pub fn unsafe_add_string_source<S: ContourSource<P, T> + ?Sized>(mut self, source: &S) -> Self {
        for path in source.iter_contours() {
            self = self.unsafe_add_string_path(path);
        }
        self
    }

    /// Adds a closed shape path to the overlay.
    /// - `contour`: An array of points that form a closed path.
    /// - **Safety**: Marked `unsafe` because it assumes the path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_shape_contour(mut self, contour: &[P]) -> Self {
        self.overlay.add_shape_contour_iter(contour.iter().map(|&p| self.adapter.float_to_int(p)));
        self
    }

    /// Adds an open string path to the overlay.
    /// - `path`: An array of points forming an open path.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_string_path(mut self, path: &[P]) -> Self {
        for w in path.windows(2) {
            let a = self.adapter.float_to_int(w[0]);
            let b = self.adapter.float_to_int(w[1]);
            self.overlay.add_string_line([a, b]);
        }

        self
    }

    /// Converts the current overlay into an `FloatStringGraph` based on the specified fill rule.
    /// The resulting graph is the foundation for performing boolean operations, and it's optimized for such operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule used to determine the filled areas within the shapes (e.g., non-zero or even-odd).
    /// - Returns: A `FloatStringGraph` containing the graph representation of the overlay's geometry.
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> FloatStringGraph<P, T> {
        self.into_graph_with_solver(fill_rule, Default::default())
    }

    /// Converts the current overlay into an `FloatStringGraph` based on the specified fill rule and solver.
    /// This method allows for finer control over the boolean operation process by passing a custom solver.
    /// - `fill_rule`: Specifies the rule used to determine the filled areas within the shapes (e.g., non-zero or even-odd).
    /// - `solver`: A custom solver for optimizing or modifying the graph creation process.
    /// - Returns: A `FloatStringGraph` containing the graph representation of the overlay's geometry.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> FloatStringGraph<P, T> {
        let graph = self.overlay.into_graph_with_solver(fill_rule, solver);
        FloatStringGraph { graph, adapter: self.adapter }
    }
}