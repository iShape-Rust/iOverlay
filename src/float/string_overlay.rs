use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_shape::base::data::{Contour, Shape};
use i_shape::float::count::PointsCount;
use crate::core::fill_rule::FillRule;
use crate::core::solver::Solver;
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

    /// Creates a new `FloatStringOverlay` instance and initializes it with the array of shapes.
    /// - `shapes`: An array of `Shape` instances.
    pub fn with_shapes(shapes: &[Shape<P>]) -> Self {
        let adapter = FloatPointAdapter::with_iter(shapes.iter().flatten().flatten());
        let capacity = shapes.points_count();

        Self::with_adapter(adapter, capacity).unsafe_add_shapes(shapes)
    }

    /// Creates a new `FloatStringOverlay` instance and initializes it with the array of paths.
    /// - `contours`: An array of `Contour` instances, each representing a closed path.
    pub fn with_paths(contours: &[Contour<P>]) -> Self {
        let adapter = FloatPointAdapter::with_iter(contours.iter().flatten());
        let capacity = contours.points_count();

        Self::with_adapter(adapter, capacity).unsafe_add_contours(contours)
    }

    /// Creates a new `FloatStringOverlay` instance and initializes it with a closed path.
    /// - `contour`: An array of points that form a closed path.
    pub fn with_contour(contour: &[P]) -> Self {
        let adapter = FloatPointAdapter::with_iter(contour.iter());

        Self::with_adapter(adapter, contour.len()).unsafe_add_contour(contour)
    }

    /// Adds multiple shapes to the overlay.
    /// - `shapes`: An array of `Shape` instances.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    pub fn unsafe_add_shapes(mut self, shapes: &[Shape<P>]) -> Self {
        for shape in shapes.iter() {
            self = self.unsafe_add_contours(shape);
        }
        self
    }

    /// Adds multiple closed shape paths to the overlay.
    /// - `contours`: An array of `Contour` instances, each representing a closed path.
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    pub fn unsafe_add_contours(mut self, contours: &[Contour<P>]) -> Self {
        for contour in contours.iter() {
            self = self.unsafe_add_contour(contour);
        }
        self
    }

    /// Adds a single closed shape path to the overlay.
    /// - `contour`: An array of points that form a closed path.
    /// - **Safety**: Marked `unsafe` because it assumes the path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_contour(mut self, contour: &[P]) -> Self {
        self.overlay.add_path_iter(contour.iter().map(|&p| self.adapter.float_to_int(p)));
        self
    }

    /// Adds a single open line string to the overlay.
    /// - `line`: An array of two points representing a line.
    /// - **Safety**: Marked `unsafe` because it assumes the line lies inside the bounding box.
    #[inline]
    pub fn unsafe_add_string_line(mut self, line: &[P; 2]) -> Self {
        let a = self.adapter.float_to_int(line[0]);
        let b = self.adapter.float_to_int(line[1]);
        self.overlay.add_string_line([a, b]);
        self
    }

    /// Adds multiple open line strings to the overlay.
    /// - `lines`: An array of line strings, each represented by two points.
    /// - **Safety**: Marked `unsafe` because it assumes each line lies inside the bounding box.
    #[inline]
    pub fn unsafe_add_string_lines(mut self, lines: &[[P; 2]]) -> Self {
        for line in lines.iter() {
            self = self.unsafe_add_string_line(line);
        }
        self
    }

    /// Adds a path to the overlay as an open or closed line string.
    /// - `path`: An array of points forming the path.
    /// - `is_open`: Indicates if the path is open (true) or closed (false).
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_string_path(mut self, path: &[P], is_open: bool) -> Self {
        for w in path.windows(2) {
            let a = self.adapter.float_to_int(w[0]);
            let b = self.adapter.float_to_int(w[1]);
            self.overlay.add_string_line([a, b]);
        }
        if !is_open && path.len() > 2 {
            let a = self.adapter.float_to_int(*path.first().unwrap());
            let b = self.adapter.float_to_int(*path.last().unwrap());
            self.overlay.add_string_line([b, a]);
        }

        self
    }

    /// Adds multiple paths to the overlay, either as open or closed line strings.
    /// - `paths`: An array of paths, each a vector of points.
    /// - `is_open`: Indicates if each path is open (`true`) or closed (`false`).
    /// - **Safety**: Marked `unsafe` because it assumes each path is fully contained within the bounding box.
    #[inline]
    pub fn unsafe_add_string_paths(mut self, paths: &[Contour<P>], is_open: bool) -> Self {
        for path in paths.iter() {
            self = self.unsafe_add_string_path(path, is_open);
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