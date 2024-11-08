use i_float::f64_adapter::F64PointAdapter;
use i_float::f64_rect::F64Rect;
use i_shape::f64::adapter::{ShapesToFloat, ShapeToInt};
use i_shape::f64::rect::RectInit;
use i_shape::f64::shape::{F64Path, F64Shapes};
use i_shape::f64::adapter::ShapeToFloat;
use crate::core::fill_rule::FillRule;
use crate::core::solver::Solver;
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;
use crate::f64::line::F64Line;
use crate::string::clip::ClipRule;
use crate::string::overlay::StringOverlay;

// #[deprecated(
//     since = "1.8.0",
//     note = "Use FloatStringOverlay<P, T> instead, which provides a more flexible and efficient API"
// )]
#[derive(Clone)]
pub struct F64StringOverlay {
    shape_paths: Vec<F64Path>,
    string_lines: Vec<F64Line>,
}

pub struct F64StringGraph {
    pub graph: StringGraph,
    pub adapter: F64PointAdapter,
}

impl F64StringOverlay {
    /// Creates a new `F64StringOverlay` instance, initializing the internal storage for paths and lines.
    /// This is useful for adding both closed shape paths and open line strings.
    #[inline]
    pub fn new() -> Self { Self { shape_paths: vec![], string_lines: vec![] } }

    /// Adds a single closed shape path to the overlay.
    /// - `path`: A `F64Path` representing a sequence of points forming a closed shape.
    #[inline]
    pub fn add_shape_path(&mut self, path: F64Path) {
        self.shape_paths.push(path);
    }

    /// Adds multiple closed shape paths to the overlay.
    /// - `paths`: A vector of `F64Path` instances, each representing a closed shape path.
    #[inline]
    pub fn add_shape_paths(&mut self, mut paths: Vec<F64Path>) {
        self.shape_paths.append(&mut paths);
    }

    /// Adds multiple shapes to the overlay.
    /// - `shapes`: A vector of `F64Shape` instances.
    #[inline]
    pub fn add_shapes(&mut self, shapes: F64Shapes) {
        for mut shape in shapes.into_iter() {
            self.shape_paths.append(&mut shape);
        }
    }

    /// Adds a single open line string to the overlay.
    /// - `line`: A `F64Line` representing a line string with two points.
    #[inline]
    pub fn add_string_line(&mut self, line: F64Line) {
        self.string_lines.push(line);
    }

    /// Adds multiple open line strings to the overlay.
    /// - `lines`: A vector of `F64Line` instances, each representing a line string.
    #[inline]
    pub fn add_string_lines(&mut self, mut lines: Vec<F64Line>) {
        self.string_lines.append(&mut lines);
    }

    /// Adds a path as an open or closed line string to the overlay.
    /// - `path`: A `F64Path` representing a sequence of points.
    /// - `is_open`: A boolean flag indicating whether the path should be treated as open (true) or closed (false).
    #[inline]
    pub fn add_string_path(&mut self, path: F64Path, is_open: bool) {
        self.string_lines.extend(
            path.windows(2)
                .map(|w| [w[0], w[1]])
        );
        if !is_open && path.len() > 2 {
            let &a = path.first().unwrap();
            let &b = path.last().unwrap();
            self.add_string_line([b, a])
        }
    }

    /// Adds multiple paths as open or closed line strings to the overlay.
    /// - `paths`: A vector of `F64Path` instances.
    /// - `is_open`: A boolean flag indicating whether the paths should be treated as open (true) or closed (false).
    #[inline]
    pub fn add_string_paths(&mut self, paths: Vec<F64Path>, is_open: bool) {
        for path in paths.into_iter() {
            self.add_string_path(path, is_open);
        }
    }

    /// Converts the current overlay into an `F64StringGraph` based on the specified fill rule.
    /// The resulting graph is the foundation for performing boolean operations, and it's optimized for such operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule used to determine the filled areas within the shapes (e.g., non-zero or even-odd).
    /// - Returns: An `F64StringGraph` containing the graph representation of the overlay.
    pub fn into_graph(self, fill_rule: FillRule) -> F64StringGraph {
        self.into_graph_with_solver(fill_rule, Default::default())
    }

    /// Converts the current overlay into an `F64StringGraph` based on the specified fill rule and solver.
    /// This method allows for finer control over the boolean operation process by passing a custom solver.
    /// - `fill_rule`: Specifies the rule used to determine the filled areas within the shapes (e.g., non-zero or even-odd).
    /// - `solver`: A custom solver for optimizing or modifying the graph creation process.
    /// - Returns: An `F64StringGraph` containing the graph representation of the overlay.
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> F64StringGraph {
        let (overlay, adapter) = self.into_overlay_and_adapter();
        let graph = overlay.into_graph_with_solver(fill_rule, solver);

        F64StringGraph { graph, adapter }
    }

    /// Clips lines according to the specified fill and clip rules.
    /// - `fill_rule`: Specifies the rule determining the filled areas, influencing the inclusion of line segments.
    /// - `clip_rule`: The rule for clipping, determining how the boundary and inversion settings affect the result.
    /// - `solver`: A solver type to be used for advanced control over the graph building process.
    ///
    /// # Returns
    /// A vector of `F64Path` instances representing the clipped sections of the input lines.
    #[inline]
    pub fn clip_string_lines_with_solver(self, fill_rule: FillRule, clip_rule: ClipRule, solver: Solver) -> Vec<F64Path> {
        let (overlay, adapter) = self.into_overlay_and_adapter();
        let paths = overlay.clip_string_lines_with_solver(fill_rule, clip_rule, solver);
        paths.to_float(&adapter)
    }

    fn into_overlay_and_adapter(self) -> (StringOverlay, F64PointAdapter) {
        let mut rect = F64Rect::with_shape(&self.shape_paths).unwrap_or(F64Rect {
            min_x: -1.0,
            max_x: 1.0,
            min_y: -1.0,
            max_y: 1.0,
        });

        for line in self.string_lines.iter() {
            rect.unsafe_add_point(&line[0]);
            rect.unsafe_add_point(&line[1]);
        }

        let adapter = F64PointAdapter::new(rect);

        let int_shapes = self.shape_paths.to_int(&adapter);

        let mut overlay = StringOverlay::with_shape_contours(&int_shapes);
        for line in self.string_lines.iter() {
            let a = adapter.convert_to_int(&line[0]);
            let b = adapter.convert_to_int(&line[1]);
            overlay.add_string_line([a, b]);
        }

        (overlay, adapter)
    }
}

impl Default for F64StringOverlay {
    fn default() -> Self {
        Self::new()
    }
}


impl F64StringGraph {
    /// Extracts shapes from the overlay graph based on the specified string rule.
    /// This method is used to retrieve the final geometric shapes after boolean operations have been applied.
    /// It's suitable for most use cases where the minimum area of shapes is not a concern.
    ///
    /// # Parameters
    /// - `string_rule`: The string operation rule to apply when extracting shapes from the graph, such as slice.
    ///
    /// # Returns
    /// A vector of `F64Shape`, representing the geometric result of the applied overlay rule.
    ///
    /// # Shape Representation
    /// The output is a `Vec<Vec<Vec<F64Point>>>`, where:
    /// - The outer `Vec<F64Shape>` represents a set of shapes.
    /// - Each shape `Vec<F64Path>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<F64Point>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline(always)]
    pub fn extract_shapes(&self, string_rule: StringRule) -> F64Shapes {
        self.extract_shapes_min_area(string_rule, 0.0)
    }

    /// Extracts shapes from the overlay graph similar to `extract_shapes`, but with an additional constraint on the minimum area of the shapes.
    /// This is useful for filtering out shapes that do not meet a certain size threshold, which can be beneficial for eliminating artifacts or noise from the output.
    ///
    /// # Parameters
    /// - `string_rule`: The string operation rule to apply when extracting shapes from the graph, such as slice.
    /// - `min_area`: The minimum area threshold for shapes to be included in the result. Shapes with an area smaller than this value will be excluded.
    ///
    /// # Returns
    /// A vector of `F64Shapes` that meet the specified area criteria, representing the cleaned-up geometric result.
    ///
    /// # Shape Representation
    /// The output is a `Vec<Vec<Vec<F64Point>>>`, where:
    /// - The outer `Vec<F64Shape>` represents a set of shapes.
    /// - Each shape `Vec<F64Path>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<F64Point>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline]
    pub fn extract_shapes_min_area(&self, string_rule: StringRule, min_area: f64) -> F64Shapes {
        let sqr_scale = self.adapter.dir_scale * self.adapter.dir_scale;
        let area = (sqr_scale * min_area) as usize;
        let shapes = self.graph.extract_shapes_min_area(string_rule, area);

        shapes.to_float(&self.adapter)
    }
}