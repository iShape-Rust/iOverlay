use crate::geom::line_range::LineRange;
use crate::segm::segment::Segment;
use crate::split::fragment::Fragment;
use crate::split::segment_tree::SegmentTree;
use crate::split::solver::SplitSolver;
use crate::split::space_layout::SpaceLayout;


impl SplitSolver {
    pub(super) fn tree_split(&mut self, segments: Vec<Segment>) -> Vec<Segment> {
        let ver_range = segments.ver_range();
        let height = ver_range.width() as usize;

        if height < SpaceLayout::MIN_HEIGHT {
            return self.list_split(segments);
        }

        let layout = SpaceLayout::new(height, segments.len());

        if layout.is_fragmentation_required_for_edges(&segments) {
            self.simple(ver_range, &layout, segments)
        } else {
            self.complex(ver_range, &layout, segments)
        }
    }

    fn simple(&self, ver_range: LineRange, layout: &SpaceLayout, mut segments: Vec<Segment>) -> Vec<Segment> {
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

    fn complex(&self, ver_range: LineRange, layout: &SpaceLayout, mut segments: Vec<Segment>) -> Vec<Segment> {
        let mut tree = SegmentTree::new(ver_range, layout.power, 0);
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut fragments = Vec::with_capacity(2 * segments.len());

        let mut iter = 0;

        while need_to_fix && segments.len() > 2 {
            need_to_fix = false;

            for (i, &e) in segments.iter().enumerate() {
                layout.break_into_fragments(i, e.x_segment, &mut fragments);
            }

            if 100 * fragments.len() <= 110 * segments.len() {
                // we can switch to simple solution
                segments = self.simple(ver_range, layout, segments);
                return segments;
            }

            tree.radius = self.solver.radius(iter);

            for fragment in fragments.iter() {
                let any_round = tree.intersect(fragment, &mut marks);
                need_to_fix = any_round || need_to_fix;

                tree.insert(fragment.clone());
            }

            if marks.is_empty() {
                return segments;
            }

            fragments.clear();
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

impl VerticalRange for Vec<Segment> {
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