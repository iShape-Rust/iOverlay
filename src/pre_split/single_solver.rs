use crate::pre_split::solver::PreSplitSolver;
use crate::split::cross_solver::ScanCrossSolver;
use crate::split::shape_edge::ShapeEdge;

impl PreSplitSolver {
    pub(super) fn single_split(max_repeat_count: usize, edges: &mut Vec<ShapeEdge>) -> bool {
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut split_count = 0;

        while need_to_fix && split_count < max_repeat_count {
            split_count += 1;
            need_to_fix = false;

            marks.clear();

            let n = edges.len();
            for i in 0..n - 1 {
                let ei = &edges[i].x_segment;
                for j in i + 1..n {
                    let ej = &edges[j].x_segment;
                    if ei.b <= ej.a {
                        break;
                    }

                    if ScanCrossSolver::test_y(ei, ej) {
                        continue;
                    }

                    Self::cross(i, j, ei, ej, &mut need_to_fix, &mut marks);
                }
            }

            if marks.is_empty() {
                return false;
            }

            Self::apply(need_to_fix, &mut marks, edges);
        }

        need_to_fix
    }
}