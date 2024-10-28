use i_float::point::IntPoint;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes, PointsCount};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::ShapeType;
use crate::core::solver::Solver;
use crate::fill::solver::{FillSolver, FillStrategy};
use crate::segm::build::BuildSegments;
use crate::segm::segment::{Segment, SegmentFill, ToSegment};
use crate::segm::shape_count::ShapeCount;
use crate::string::graph::StringGraph;
use crate::split::solver::SplitSegments;
use crate::string::line::IntLine;

pub struct StringOverlay {
    pub(super) segments: Vec<Segment>,
}

impl StringOverlay {
    /// Constructs a new `StringOverlay` instance, initializing it with a capacity that should closely match the total count of edges from all shapes being processed.
    /// This pre-allocation helps in optimizing memory usage and performance.
    /// - `capacity`: The initial capacity for storing edge data. Ideally, this should be set to the sum of the edges of all shapes to be added to the overlay, ensuring efficient data management.
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            segments: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new `StringOverlay` instance and initializes it with a single shape path.
    /// - `path`: A path to be used in the overlay operation as a closed shape.
    #[inline]
    pub fn with_shape_path(path: &[IntPoint]) -> Self {
        let mut overlay = Self::new(path.len());
        overlay.add_shape_path(path);
        overlay
    }

    /// Creates a new `StringOverlay` instance and initializes it with multiple shape paths.
    /// - `paths`: An array of paths that together define multiple shapes.
    #[inline]
    pub fn with_shape_paths(paths: &[IntPath]) -> Self {
        let mut overlay = Self::new(paths.points_count());
        overlay.add_shape_paths(paths);
        overlay
    }

    /// Creates a new `StringOverlay` instance and initializes it with subject and clip shapes.
    /// - `shape`: A shape to be used in the overlay operation.
    #[inline]
    pub fn with_shape(shape: &IntShape) -> Self {
        let mut overlay = Self::new(shape.points_count());
        overlay.add_shape(shape);
        overlay
    }

    /// Creates a new `StringOverlay` instance and initializes it with subject and clip shapes.
    /// - `shapes`: An array of shapes to be used in the overlay operation.
    #[inline]
    pub fn with_shapes(shapes: &IntShapes) -> Self {
        let mut overlay = Self::new(shapes.points_count());
        overlay.add_shapes(shapes);
        overlay
    }

    /// Adds a single path to the overlay as a shape paths.
    /// - `path`: A reference to a `IntPath` instance to be added.
    #[inline]
    pub fn add_shape_path(&mut self, path: &[IntPoint]) {
        self.segments.append_path_iter(path.iter().copied(), ShapeType::Subject);
    }

    /// Adds multiple paths to the overlay as shape paths.
    /// - `paths`: An array of `IntPath` instances to be added to the overlay.
    #[inline]
    pub fn add_shape_paths(&mut self, paths: &[IntPath]) {
        for path in paths.iter() {
            self.add_shape_path(path);
        }
    }

    /// Adds a single shape to the overlay.
    /// - `shape`: A reference to a `IntShape` instance to be added.
    #[inline]
    pub fn add_shape(&mut self, shape: &IntShape) {
        self.add_shape_paths(shape);
    }

    /// Adds a list of shape to the overlay.
    /// - `shapes`: A reference to a `IntShape` instance to be added.
    #[inline]
    pub fn add_shapes(&mut self, shapes: &IntShapes) {
        for shape in shapes {
            self.add_shape(shape);
        }
    }

    /// Adds a single line (open path) to the overlay.
    /// - `line`: An `IntLine` representing the open line (defined by two points).
    #[inline]
    pub fn add_string_line(&mut self, line: IntLine) {
        if line[0] != line[1] {
            self.segments.push(line.to_segment(ShapeCount::new(0, 1)));
        }
    }

    /// Adds multiple lines (open paths) to the overlay.
    /// - `lines`: An array of `IntLine` instances to be added.
    #[inline]
    pub fn add_string_lines(&mut self, lines: &[IntLine]) {
        for &line in lines {
            self.add_string_line(line);
        }
    }

    /// Adds a path (a sequence of points) as an open or closed string to the overlay.
    /// - `path`: A reference to an array of `IntPoint` representing the path.
    /// - `is_open`: A boolean flag indicating whether the path is open (true) or closed (false).
    #[inline]
    pub fn add_string_path(&mut self, path: &[IntPoint], is_open: bool) {
        let mut a = if let Some(&p) = path.first() { p } else { return; };
        for &b in path.iter().skip(1) {
            self.add_string_line([a, b]);
            a = b;
        }

        if !is_open && path.len() > 2 {
            let &a = path.first().unwrap();
            let &b = path.last().unwrap();
            self.add_string_line([b, a])
        }
    }

    /// Adds multiple paths as open or closed strings to the overlay.
    /// - `paths`: An array of `IntPath` instances (each a sequence of points).
    /// - `is_open`: A boolean flag indicating whether the paths are open (true) or closed (false).
    #[inline]
    pub fn add_string_paths(&mut self, paths: &[IntPath], is_open: bool) {
        for path in paths {
            self.add_string_path(path, is_open);
        }
    }

    /// Converts the overlay into a `StringGraph`, using the specified `FillRule`.
    /// This graph is used for string operations, enabling analysis and manipulation of geometric data.
    /// - `fill_rule`: The rule that defines how to fill shapes (e.g., non-zero, even-odd).
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> StringGraph {
        self.into_graph_with_solver(fill_rule, Default::default())
    }

    /// Converts the overlay into a `StringGraph`, with an additional option to specify a custom solver.
    /// This graph is used for string operations, enabling analysis and manipulation of geometric data.
    /// - `fill_rule`: The rule that defines how to fill shapes (e.g., non-zero, even-odd).
    /// - `solver`: A solver type to be used for advanced control over the graph building process.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> StringGraph {
        StringGraph::new(solver, self.into_segments(fill_rule, solver))
    }

    /// Convert into segments from the added paths or shapes according to the specified fill rule.
    /// - `fill_rule`: The fill rule to use when determining the inside of shapes.
    /// - `filter`: Is need to clean empty segments
    /// - `solver`: Type of solver to use.
    fn into_segments(self, fill_rule: FillRule, solver: Solver) -> (Vec<Segment>, Vec<SegmentFill>) {
        if self.segments.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let segments = self.segments.split_segments(solver);

        let is_list = solver.is_list_fill(&segments);

        let fills = match fill_rule {
            FillRule::EvenOdd => FillSolver::fill::<EvenOddStrategyString>(is_list, &segments),
            FillRule::NonZero => FillSolver::fill::<NonZeroStrategyString>(is_list, &segments),
            FillRule::Positive => FillSolver::fill::<PositiveStrategyString>(is_list, &segments),
            FillRule::Negative => FillSolver::fill::<NegativeStrategyString>(is_list, &segments),
        };

        (segments, fills)
    }
}

pub(crate) struct EvenOddStrategyString;
pub(crate) struct NonZeroStrategyString;
pub(crate) struct PositiveStrategyString;
pub(crate) struct NegativeStrategyString;

impl FillStrategy for EvenOddStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = 1 & top.subj as SegmentFill;
        let subj_bot = 1 & bot.subj as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}

impl FillStrategy for NonZeroStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = (top.subj != 0) as SegmentFill;
        let subj_bot = (bot.subj != 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}

impl FillStrategy for PositiveStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = (top.subj < 0) as SegmentFill;
        let subj_bot = (bot.subj < 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}

impl FillStrategy for NegativeStrategyString {
    #[inline(always)]
    fn add_and_fill(this: ShapeCount, bot: ShapeCount) -> (ShapeCount, SegmentFill) {
        let subj = bot.subj + this.subj;
        let clip = (this.clip != 0) as u8;
        let top = ShapeCount { subj, clip: 0 };

        let subj_top = (top.subj > 0) as SegmentFill;
        let subj_bot = (bot.subj > 0) as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | clip << 2;

        (top, fill)
    }
}