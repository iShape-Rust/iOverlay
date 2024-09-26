use crate::segm::segment::Segment;
use crate::split::solver::SplitSolver;

impl SplitSolver {
    pub(super) fn list_split(&mut self, edges: &mut Vec<Segment>) {
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut iter = 0;

        while need_to_fix && edges.len() > 2 {
            need_to_fix = false;
            marks.clear();

            let radius: i64 = self.solver.radius(iter);

            for i in 0..edges.len() - 1 {
                let ei = &edges[i].x_segment;
                let ri = ei.y_range();
                for (j, s) in edges.iter().enumerate().skip(i + 1) {
                    let ej = &s.x_segment;
                    if ei.b.x < ej.a.x {
                        break;
                    }

                    if ej.is_not_intersect_y_range(&ri) {
                        continue;
                    }

                    let is_round = SplitSolver::cross(i, j, ei, ej, &mut marks, radius);
                    need_to_fix = need_to_fix || is_round
                }
            }

            if marks.is_empty() {
                return;
            }

            self.apply(&mut marks, edges, need_to_fix);

            if need_to_fix && !self.solver.is_list_split(edges) {
                // finish with tree solver if edges is become large
                self.tree_split(edges);
                return;
            }

            iter += 1;
        }
    }
}