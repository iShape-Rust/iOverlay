use crate::space::line_range::LineRange;
use crate::space::line_space::{IntExtensions, LineSpace};
use crate::split::shape_edge::ShapeEdge;
use crate::split::version_index::VersionedIndex;

impl LineSpace<VersionedIndex> {
    pub(crate) fn with_edges(edges: &Vec<ShapeEdge>) -> Self {
        let mut y_min = i64::MAX;
        let mut y_max = i64::MIN;
        for edge in edges.iter() {
            if edge.a.y > edge.b.y {
                y_min = y_min.min(edge.b.y.value());
                y_max = y_max.max(edge.a.y.value());
            } else {
                y_min = y_min.min(edge.a.y.value());
                y_max = y_max.max(edge.b.y.value());
            }
        }

        let max_level = ((edges.len() as f64).sqrt() as usize).log_two();
        let range = LineRange { min: y_min as i32, max: y_max as i32 };

        LineSpace::new(max_level, range)
    }
}
