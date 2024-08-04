use crate::split::shape_edge::ShapeEdge;
use crate::split::solver::SplitSolver;
use crate::x_segment::Boundary;

impl SplitSolver {
    pub(super) fn list_split(&self, edges: &mut Vec<ShapeEdge>) -> bool {
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        while need_to_fix {
            marks.clear();

            let n = edges.len();

            if n < 3 {
                return true
            }

            for i in 0..n - 1 {
                let ei = &edges[i].x_segment;
                let ri = ei.boundary();
                for j in i + 1..n {
                    let ej = &edges[j].x_segment;
                    if ei.b.x < ej.a.x {
                        break;
                    }

                    if !ej.boundary().is_intersect_border_include(&ri) {
                        continue;
                    }

                    let is_round = Self::cross(i, j, ei, ej, &mut marks);
                    need_to_fix = need_to_fix || is_round
                }
            }

            if marks.is_empty() {
                return true;
            }

            self.apply(&mut marks, edges);

            if !self.solver.is_list(edges) {
                // finish with tree solver if edges is become large
                return self.tree_split(edges);
            }
        }

        true
    }
}