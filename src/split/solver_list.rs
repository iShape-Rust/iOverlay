use crate::geom::segment::Segment;
use crate::split::solver::SplitSolver;

impl SplitSolver {
    pub(super) fn list_split(&mut self, mut segments: Vec<Segment>) -> Vec<Segment> {
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut iter = 0;

        while need_to_fix && segments.len() > 2 {
            need_to_fix = false;
            marks.clear();

            let radius: i64 = self.solver.radius(iter);

            for i in 0..segments.len() - 1 {
                let ei = &segments[i].x_segment;
                let ri = ei.y_range();
                for (j, s) in segments.iter().enumerate().skip(i + 1) {
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
                return segments;
            }

            segments = self.apply(&mut marks, segments, need_to_fix);

            if need_to_fix && !self.solver.is_list_split(&segments) {
                // finish with tree solver if edges is become large
                return self.tree_split(segments);
            }

            iter += 1;
        }

        segments
    }
}