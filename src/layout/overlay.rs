use i_float::fix_vec::FixVec;
use i_shape::fix_path::{FixPath, FixPathExtension};
use i_shape::fix_shape::FixShape;
use crate::fill::fill_segments::FillSegments;
use crate::split::split_edges::SplitEdges;

use crate::{split::{shape_edge::ShapeEdge, shape_count::ShapeCount}, fill::{segment::Segment}};
use crate::bool::fill_rule::FillRule;
use crate::space::line_range::LineRange;

use super::overlay_graph::OverlayGraph;

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Subject,
    Clip,
}

pub struct Overlay {
    y_min: i32,
    y_max: i32,
    edges: Vec<ShapeEdge>,
}

impl Overlay {
    pub fn new(capacity: usize) -> Self {
        Self {
            y_min: i32::MAX,
            y_max: i32::MIN,
            edges: Vec::with_capacity(capacity),
        }
    }

    pub fn from_paths(subject_paths: &[FixPath], clip_paths: &[FixPath]) -> Self {
        let mut overlay = Self::new(64);
        overlay.add_paths(subject_paths, ShapeType::Subject);
        overlay.add_paths(clip_paths, ShapeType::Clip);
        overlay
    }

    pub fn add_shape(&mut self, shape: &FixShape, shape_type: ShapeType) {
        self.add_paths(&shape.paths, shape_type);
    }

    pub fn add_paths(&mut self, paths: &[FixPath], shape_type: ShapeType) {
        for path in paths.iter() {
            self.add_path(path, shape_type);
        }
    }

    pub fn add_path(&mut self, path: &FixPath, shape_type: ShapeType) {
        if let Some(mut result) = path.to_vec().removed_degenerates().edges(shape_type) {
            self.y_min = self.y_min.min(result.y_min);
            self.y_max = self.y_max.max(result.y_max);
            self.edges.append(&mut result.edges);
        }
    }

    pub fn build_segments(&self, fill_rule: FillRule) -> Vec<Segment> {
        if self.edges.is_empty() {
            return Vec::new();
        }

        let mut sorted_list = self.edges.clone();
        sorted_list.sort_by(|a, b| a.order(b));

        let mut buffer = Vec::with_capacity(sorted_list.len());

        let mut prev = ShapeEdge {
            a: FixVec::ZERO,
            b: FixVec::ZERO,
            count: ShapeCount::new(0, 0),
        };

        for next in sorted_list.into_iter() {
            if prev.is_equal(&next) {
                prev.count = prev.count.add(next.count);
            } else {
                if prev.count.is_not_empty() {
                    buffer.push(prev);
                }
                prev = next;
            }
        }

        if prev.count.is_not_empty() {
            buffer.push(prev);
        }

        let range = LineRange { min: self.y_min, max: self.y_max };

        let mut segments = buffer.split(range);

        segments.fill(fill_rule, range);

        return segments;
    }

    pub fn build_graph(&self, fill_rule: FillRule) -> OverlayGraph {
        OverlayGraph::new(self.build_segments(fill_rule))
    }
}

struct EdgeResult {
    edges: Vec<ShapeEdge>,
    y_min: i32,
    y_max: i32,
}

trait CreateEdges {
    fn edges(&self, shape_type: ShapeType) -> Option<EdgeResult>;
}

impl CreateEdges for FixPath {
    fn edges(&self, shape_type: ShapeType) -> Option<EdgeResult> {
        let n = self.len();
        if n < 3 {
            return None;
        }

        let mut edges = vec![ShapeEdge::ZERO; n];

        let i0 = n - 1;
        let mut p0 = self[i0];

        let mut y_min = p0.y;
        let mut y_max = p0.y;

        for i in 0..n {
            let p1 = self[i];
            y_min = y_min.min(p1.y);
            y_max = y_max.max(p1.y);

            let value = if p0.bit_pack() <= p1.bit_pack() { 1 } else { -1 };
            match shape_type {
                ShapeType::Subject => {
                    edges[i] = ShapeEdge::new(p0, p1, ShapeCount::new(value, 0));
                }
                ShapeType::Clip => {
                    edges[i] = ShapeEdge::new(p0, p1, ShapeCount::new(0, value));
                }
            }

            p0 = p1
        }

        return Some(EdgeResult {
            edges,
            y_min: y_min as i32,
            y_max: y_max as i32,
        });
    }
}