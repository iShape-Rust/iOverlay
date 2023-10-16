use i_shape::fix_path::{FixPath, FixPathExtension};
use crate::fill::fill_segments::FillSegments;
use crate::split::split_edges::SplitEdges; 

use crate::{split::{shape_edge::ShapeEdge, shape_count::ShapeCount}, fill::{shape_type::ShapeType, segment::{Segment, SegmentFill}}};

use super::overlay_graph::OverlayGraph;

pub struct Overlay {
    edges: Vec<ShapeEdge>,
}

impl Overlay {
    pub fn new(capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(capacity),
        }
    }

    pub fn from_paths(subject_paths: Vec<FixPath>, clip_paths: Vec<FixPath>) -> Self {
        let mut overlay = Self::new(64);
        overlay.add_paths(subject_paths, ShapeType::SUBJECT);
        overlay.add_paths(clip_paths, ShapeType::CLIP);
        overlay
    }

    pub fn from_subject_paths(subject_paths: Vec<FixPath>) -> Self {
        let mut overlay = Self::new(subject_paths.len() + 1);
        overlay.add_paths(subject_paths, ShapeType::SUBJECT);
        overlay
    }

    pub fn from_subject_path(subject_path: FixPath) -> Self {
        let mut overlay = Self::new(2);
        overlay.add_path(subject_path, ShapeType::SUBJECT);
        overlay
    }

    pub fn add_path(&mut self, path: FixPath, shape_type: ShapeType) {
        let count = if shape_type == ShapeType::CLIP { ShapeCount::new(0, 1) } else { ShapeCount::new(1, 0) };
        self.add_single_path(path, count);
    }

    pub fn add_paths(&mut self, paths: Vec<FixPath>, shape_type: ShapeType) {
        let count = if shape_type == ShapeType::CLIP { ShapeCount::new(0, 1) } else { ShapeCount::new(1, 0) };
        self.add_multiple_paths(paths, count);
    }

    fn add_multiple_paths(&mut self, paths: Vec<FixPath>, shape_count: ShapeCount) {
        for path in paths {
            self.add_single_path(path, shape_count);
        }
    }

    fn add_single_path(&mut self, path: FixPath, shape_count: ShapeCount) {
        let clean = path.removed_degenerates();
        let path_edges = Self::create_edges(&clean, shape_count);
        self.edges.extend(path_edges);
    }

    pub fn build_segments(&mut self) -> Vec<Segment> {
        if self.edges.is_empty() {
            return Vec::new()
        }
        
        let mut sorted_list = self.edges.clone();
        self.edges.clear();

        sorted_list.sort_by(|a, b| a.order(b));
        
        
        let mut prev: ShapeEdge = sorted_list[0];
        
        for i in 1..sorted_list.len() {
            let next = sorted_list[i];
            
            if prev.is_equal(next) {
                prev = prev.merge(next);
            } else {
                if !prev.is_even() {
                    self.edges.push(prev);
                }
                prev = next;
            }
        }

        if !prev.is_even() {
            self.edges.push(prev);
        }
        
        self.edges.split();

        let mut segments = Vec::with_capacity(self.edges.len());

        for edge in &self.edges {
            let is_subj = edge.is_odd_subj();
            let is_clip = edge.is_odd_clip();
            
            if is_subj || is_clip {
                let clip = if is_clip { ShapeType::CLIP } else { ShapeType::NONE };
                let subj = if is_subj { ShapeType::SUBJECT } else { ShapeType::NONE };
                let shape = clip | subj;

                let segment = Segment::new(
                    edge.a,
                    edge.b,
                    shape,
                    SegmentFill::NONE
                );
            
                segments.push(segment);
            }
        }
        
        segments.fill();
        
        return segments
    }

    pub fn build_graph(&mut self) -> OverlayGraph {
        OverlayGraph::new(self.build_segments())
    }

    fn create_edges(path: &FixPath, shape_count: ShapeCount) -> Vec<ShapeEdge> {
        let n = path.len();
        if n < 3 {
            return Vec::new()
        }
        
        let mut edges:Vec<ShapeEdge> = vec![ShapeEdge::ZERO; n];
        
        let i0 = n - 1;
        let mut a = path[i0];
        
        for i in 0..n {
            let b = path[i];
            edges[i] = ShapeEdge::new(a, b, shape_count);
            a = b;
        }
        
        return edges
    }

}
