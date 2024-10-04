use i_shape::int::path::IntPath;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{BuildSegments, Overlay};
use crate::core::solver::Solver;
use crate::extension::line::{IntLine, LineGeometry};
use crate::extension::unstable_graph::UnstableGraph;
use crate::segm::segment::{Segment, ToSegment};
use crate::segm::shape_count::ShapeCount;

impl Overlay {
    pub fn into_ext(self) -> ExtOverlay {
        let mut segments = self.segments;
        for s in segments.iter_mut() {
            s.count.clip = s.count.clip.abs();
        }
        ExtOverlay { segments }
    }
}

pub struct ExtOverlay {
    pub(super) segments: Vec<Segment>,
}

impl ExtOverlay {
    #[inline]
    pub fn add_line(&mut self, line: &IntLine) {
        if line.sqr_length() > 0 {
            self.segments.push(line.to_segment(ShapeCount::new(0, 1)));
        }
    }

    #[inline]
    pub fn add_lines(&mut self, lines: &[IntLine]) {
        for line in lines.iter() {
            if line.sqr_length() > 0 {
                self.segments.push(line.to_segment(ShapeCount::new(0, 1)));
            }
        }
    }

    #[inline]
    pub fn add_open_path(&mut self, path: &IntPath) {
        self.segments.extend(
            path.windows(2)
                .map(|w| w.to_segment(ShapeCount::new(0, 1)))
        );
    }

    #[inline]
    pub fn add_open_paths(&mut self, paths: &[IntPath]) {
        for path in paths {
            self.segments.extend(
                path.windows(2)
                    .map(|w| w.to_segment(ShapeCount::new(0, 1)))
            );
        }
    }

    /// Convert into `UnstableGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `UnstableGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    pub fn into_graph(self, fill_rule: FillRule) -> UnstableGraph {
        self.into_graph_with_solver(fill_rule, Default::default())
    }

    /// Convert into `UnstableGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `UnstableGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> UnstableGraph {
        UnstableGraph::new(solver, self.segments.prepare_and_fill(fill_rule, solver))
    }
}