use crate::dual_index::DualIndex;
use crate::space::line_range::LineRange;
use crate::space::line_space::{IntExtensions, LineContainer, LineSegment, LineSpace};
use crate::split::shape_edge::ShapeEdge;
use crate::split::version_index::VersionedIndex;

pub(super) struct SplitScanList {
    space: LineSpace<VersionedIndex>,
}

impl SplitScanList {
    pub(super) fn new(edges: &Vec<ShapeEdge>) -> Self {
        let mut ymin: i64 = i64::MAX;
        let mut ymax: i64 = i64::MIN;
        for edge in edges.iter() {
            if edge.a.y > edge.b.y {
                ymin = ymin.min(edge.b.y.value());
                ymax = ymax.max(edge.a.y.value());
            } else {
                ymin = ymin.min(edge.a.y.value());
                ymax = ymax.max(edge.b.y.value());
            }
        }

        let max_level = ((edges.len() as f64).sqrt() as usize).log_two();

        Self { space: LineSpace::new(max_level, LineRange { min: ymin as i32, max: ymax as i32 }) }
    }

    pub(super) fn all_in_range(&mut self, range: LineRange) -> &Vec<LineContainer<VersionedIndex>> {
        self.space.all_in_range(range)
    }

    pub(super) fn insert(&mut self, segment: LineSegment<VersionedIndex>) {
        self.space.insert(segment);
    }

    pub(super) fn remove(&mut self, indices: &mut Vec<DualIndex>) {
        if indices.len() > 1 {
            indices.sort_by(|a, b| a.order_asc_major_des_minor(b));
            for index in indices {
                self.space.remove(index);
            }
        } else {
            self.space.remove(&indices[0]);
        }
    }

    pub(super) fn clear(&mut self) {
        self.space.clear()
    }
}