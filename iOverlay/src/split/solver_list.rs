use alloc::vec::Vec;
use crate::core::solver::Solver;
use crate::segm::segment::Segment;
use crate::segm::winding::WindingCount;
use crate::split::snap_radius::SnapRadius;
use crate::split::solver::SplitSolver;

impl SplitSolver {
    pub(super) fn list_split<C: WindingCount>(
        &mut self,
        snap_radius: SnapRadius,
        segments: &mut Vec<Segment<C>>,
        solver: &Solver
    ) -> bool {
        let mut need_to_fix = true;

        let mut snap_radius = snap_radius;
        let mut any_intersection = false;

        while need_to_fix && segments.len() > 1 {
            need_to_fix = false;
            self.marks.clear();

            let radius: i64 = snap_radius.radius();

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

                    let is_round = SplitSolver::cross(i, j, ei, ej, &mut self.marks, radius);
                    need_to_fix = need_to_fix || is_round
                }
            }

            if self.marks.is_empty() {
                return any_intersection;
            }
            any_intersection = true;
            self.apply(segments, need_to_fix, solver);

            snap_radius.increment();

            if need_to_fix && !solver.is_list_split(segments) {
                // finish with tree solver if edges is become large
                self.tree_split(snap_radius, segments, solver);
                return true;
            }
        }

        any_intersection
    }
}
