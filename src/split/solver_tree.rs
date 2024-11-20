use crate::geom::line_range::LineRange;
use crate::segm::segment::Segment;
use crate::segm::winding_count::WindingCount;
use crate::split::fragment::Fragment;
use crate::split::segment_tree::SegmentTree;
use crate::split::solver::SplitSolver;
use crate::split::space_layout::SpaceLayout;


impl SplitSolver {
    pub(super) fn tree_split<C: WindingCount>(&mut self, mut segments: Vec<Segment<C>>) -> Vec<Segment<C>> {
        let ver_range = segments.ver_range();
        let height = ver_range.width() as usize;

        if height < SpaceLayout::MIN_HEIGHT {
            return self.list_split(segments);
        }

        let layout = SpaceLayout::new(height, segments.len());

        let mut tree = SegmentTree::new(ver_range, layout.power, 0);
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut iter = 0;

        while need_to_fix && segments.len() > 2 {
            need_to_fix = false;

            tree.radius = self.solver.radius(iter);

            for (i, e) in segments.iter().enumerate() {
                let fragment = Fragment::with_index_and_segment(i, e.x_segment);
                let any_round = tree.intersect(&fragment, &mut marks);
                need_to_fix = any_round || need_to_fix;

                tree.insert(fragment);
            }

            if marks.is_empty() {
                return segments;
            }

            tree.clear();

            segments = self.apply(&mut marks, segments, need_to_fix);

            marks.clear();

            iter += 1;
        }

        segments
    }
}

trait VerticalRange {
    fn ver_range(&self) -> LineRange;
}

impl<C: Send> VerticalRange for Vec<Segment<C>> {
    fn ver_range(&self) -> LineRange {
        let mut min_y = self[0].x_segment.a.y;
        let mut max_y = min_y;

        for edge in self.iter() {
            min_y = min_y.min(edge.x_segment.a.y);
            max_y = max_y.max(edge.x_segment.a.y);
            min_y = min_y.min(edge.x_segment.b.y);
            max_y = max_y.max(edge.x_segment.b.y);
        }

        LineRange { min: min_y, max: max_y }
    }
}