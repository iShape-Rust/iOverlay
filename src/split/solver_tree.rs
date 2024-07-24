use crate::split::fragment::Fragment;
use crate::split::segment_tree::SegmentTree;
use crate::split::shape_edge::ShapeEdge;
use crate::split::solver::SplitSolver;
use crate::split::space_layout::SpaceLayout;


impl SplitSolver {
    pub(super) fn tree_split(&self, edges: &mut Vec<ShapeEdge>) -> bool {
        let layout = SpaceLayout::new(self.range, edges.len());

        if layout.is_fragmentation_required_for_edges(edges) {
            self.simple(&layout, edges);
        } else {
            self.complex(&layout, edges);
        }

        return false;
    }

    fn simple(&self, layout: &SpaceLayout, edges: &mut Vec<ShapeEdge>) {
        let mut tree = SegmentTree::new(self.range, layout.power);
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        while need_to_fix {
            need_to_fix = false;

            marks.clear();

            for i in 0..edges.len() {
                let fragment = Fragment::with_index_and_segment(i, edges[i].x_segment);
                let any_round = tree.intersect(&fragment, &mut marks);
                need_to_fix = any_round || need_to_fix;

                tree.insert(fragment);
            }

            if marks.is_empty() {
                return;
            }

            tree.clear();

            Self::apply(need_to_fix, &mut marks, edges);
        }
    }

    fn complex(&self, layout: &SpaceLayout, edges: &mut Vec<ShapeEdge>) {
        let mut tree = SegmentTree::new(self.range, layout.power);
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut fragments = Vec::with_capacity(2 * edges.len());


        while need_to_fix {
            need_to_fix = false;

            marks.clear();
            fragments.clear();

            for i in 0..edges.len() {
                layout.break_into_fragments(i, &edges[i].x_segment, &mut fragments);
            }

            if 100 * fragments.len() <= 110 * edges.len() {
                // we can switch to simple solution
                self.simple(layout, edges);
                return;
            }


            for fragment in fragments.iter() {
                let any_round = tree.intersect(fragment, &mut marks);
                need_to_fix = any_round || need_to_fix;

                tree.insert(fragment.clone());
            }

            if marks.is_empty() {
                return;
            }

            tree.clear();

            Self::apply(need_to_fix, &mut marks, edges);
        }
    }
}