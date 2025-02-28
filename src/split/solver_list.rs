use crate::segm::segment::Segment;
use crate::segm::winding_count::WindingCount;
use crate::split::grid_layout::GridLayout;
use crate::split::snap_radius::SnapRadius;
use crate::split::solver::SplitSolver;

impl SplitSolver {
    pub(super) fn list_split<C: WindingCount>(&self, snap_radius: SnapRadius, mut segments: Vec<Segment<C>>) -> Vec<Segment<C>> {
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut snap_radius = snap_radius;

        while need_to_fix && segments.len() > 2 {
            if segments.len() > 5000 {
                // continue with fragment solver
                if let Some(layout) = GridLayout::new(segments.iter().map(|it| it.x_segment), segments.len()) {
                    return self.fragment_split(layout, snap_radius, segments);
                }
            }

            need_to_fix = false;
            marks.clear();

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

                    let is_round = SplitSolver::cross(i, j, ei, ej, &mut marks, radius);
                    need_to_fix = need_to_fix || is_round
                }
            }

            if marks.is_empty() {
                return segments;
            }

            segments = self.apply(&mut marks, segments, need_to_fix);

            snap_radius.increment();
        }

        segments
    }
}