use crate::split::shape_edge::ShapeEdge;
use crate::split::solver::SplitSolver;
use crate::x_segment::XSegment;

impl SplitSolver {
    pub(super) fn list_split(&self, edges: &mut Vec<ShapeEdge>) -> bool {
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        while need_to_fix {
            need_to_fix = false;

            marks.clear();

            let n = edges.len();

            for i in 0..n - 1 {
                let ei = &edges[i].x_segment;
                for j in i + 1..n {
                    let ej = &edges[j].x_segment;
                    if ei.b.x < ej.a.x {
                        break;
                    }

                    if ei.is_boundary_not_cross(ej) {
                        continue;
                    }

                    let is_round = Self::cross(i, j, ei, ej, &mut marks);
                    need_to_fix = need_to_fix || is_round
                }
            }

            if marks.is_empty() {
                return true;
            }

            Self::apply(need_to_fix, &mut marks, edges);

            if !self.solver.is_list(self.range.width(), edges.len()) {
                // finish with tree solver if edges is become large
                return self.tree_split(edges);
            }
        }

        true
    }
}

impl XSegment {
    fn is_boundary_not_cross(&self, other: &Self) -> bool {
        Self::test_y(self, other) || Self::test_x(self, other)
    }

    fn test_x(target: &Self, other: &Self) -> bool {
        // MARK: a.x <= b.x by design
        target.a.x > other.a.x && target.a.x > other.b.x || other.a.x > target.a.x && other.a.x > target.b.x
    }

    fn test_y(target: &Self, other: &Self) -> bool {
        target.a.y > other.a.y && target.a.y > other.b.y &&
            target.b.y > other.a.y && target.b.y > other.b.y ||
            target.a.y < other.a.y && target.a.y < other.b.y &&
                target.b.y < other.a.y && target.b.y < other.b.y
    }
}