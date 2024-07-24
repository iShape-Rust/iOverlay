use std::cmp::Ordering;
use crate::core::solver::Solver;
use crate::line_range::LineRange;
use crate::sort::SmartSort;
use crate::split::cross_solver::{CrossType, CrossSolver};
use crate::split::line_mark::LineMark;
use crate::split::shape_edge::ShapeEdge;
use crate::x_segment::XSegment;

pub(crate) struct SplitSolver {
    pub(crate) solver: Solver,
    pub(crate) range: LineRange,
}

impl SplitSolver {
    pub(crate) fn split(&self, edges: &mut Vec<ShapeEdge>) -> bool {
        let is_list = self.solver.is_list(self.range.width(), edges.len());

        if is_list {
            self.list_split(edges)
        } else {
            self.tree_split(edges)
        }
    }

    pub(super) fn cross(i: usize, j: usize, ei: &XSegment, ej: &XSegment, marks: &mut Vec<LineMark>) -> bool {
        let cross = if let Some(cross) = CrossSolver::cross(ei, ej) {
            cross
        } else {
            return false;
        };

        match cross.cross_type {
            CrossType::Pure => {
                let li = ei.a.sqr_distance(cross.point);
                let lj = ej.a.sqr_distance(cross.point);

                marks.push(LineMark { index: i, length: li, point: cross.point });
                marks.push(LineMark { index: j, length: lj, point: cross.point });
            }
            CrossType::TargetEnd => {
                let lj = ej.a.sqr_distance(cross.point);
                marks.push(LineMark { index: j, length: lj, point: cross.point });
            }
            CrossType::OtherEnd => {
                let li = ei.a.sqr_distance(cross.point);

                marks.push(LineMark { index: i, length: li, point: cross.point });
            }
        }
        return cross.is_round;
    }

    pub(super) fn apply(need_to_fix: bool, marks: &mut Vec<LineMark>, edges: &mut Vec<ShapeEdge>) {
        marks.smart_sort_by(|a, b|
        if a.index < b.index || a.index == b.index && a.length < b.length {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        if !need_to_fix {
            edges.reserve_exact(4 * marks.len());
        }

        let mut i = 0;
        while i < marks.len() {
            let i0 = i;
            let index = marks[i].index;
            while i < marks.len() && marks[i].index == index {
                i += 1;
            }

            let e0 = edges[index];
            let mut p = marks[i0].point;
            let mut l = marks[i0].length;
            edges[index] = ShapeEdge::create_and_validate(e0.x_segment.a, p, e0.count);

            let mut j = i0 + 1;
            while j < i {
                let mj = &marks[j];
                if l != mj.length {
                    edges.push(ShapeEdge::create_and_validate(p, mj.point, e0.count));

                    p = mj.point;
                    l = mj.length;
                }
                j += 1;
            }
            edges.push(ShapeEdge::create_and_validate(p, e0.x_segment.b, e0.count));
        }

        edges.smart_sort_by(|a, b| a.x_segment.cmp(&b.x_segment));

        edges.merge();
    }
}

trait Merge {
    fn merge(&mut self);
}

impl Merge for Vec<ShapeEdge> {
    fn merge(&mut self) {
        let mut prev = if let Some(edge) = self.first() {
            edge.clone()
        } else {
            return;
        };

        let mut is_modified = false;
        let mut i = 1;
        while i < self.len() {
            if prev.x_segment == self[i].x_segment {
                let c = self.remove(i).count;
                prev.count = prev.count.add(c);
                is_modified = true;

                continue;
            } else if is_modified {
                is_modified = false;
                self[i - 1].count = prev.count;
                prev = self[i];
            }

            i += 1
        }
    }
}