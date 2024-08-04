use std::cmp::Ordering;
use crate::core::solver::Solver;
use crate::sort::SmartSort;
use crate::split::cross_solver::{CrossType, CrossSolver, EndMask};
use crate::split::line_mark::LineMark;
use crate::split::shape_edge::{ShapeEdge, ShapeEdgesMerge};
use crate::x_segment::XSegment;

pub(crate) struct SplitSolver {
    pub(super) solver: Solver
}

impl SplitSolver {

    pub(crate) fn new(solver: Solver) -> Self {
        Self { solver }
    }

    pub(crate) fn split(&self, edges: &mut Vec<ShapeEdge>) -> bool {
        let is_list = self.solver.is_list(edges);

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
            CrossType::Overlay => {
                let mask = CrossSolver::collinear(ei, ej);
                if mask == 0 { return false; }

                if mask.is_target_a() {
                    let lj = ej.a.sqr_distance(ei.a);
                    marks.push(LineMark { index: j, length: lj, point: ei.a });
                }

                if mask.is_target_b() {
                    let lj = ej.a.sqr_distance(ei.b);
                    marks.push(LineMark { index: j, length: lj, point: ei.b });
                }

                if mask.is_other_a() {
                    let li = ei.a.sqr_distance(ej.a);
                    marks.push(LineMark { index: i, length: li, point: ej.a });
                }

                if mask.is_other_b() {
                    let li = ei.a.sqr_distance(ej.b);
                    marks.push(LineMark { index: i, length: li, point: ej.b });
                }
            }
        }
        return cross.is_round;
    }

    pub(super) fn apply(&self, marks: &mut Vec<LineMark>, edges: &mut Vec<ShapeEdge>) {
        marks.smart_sort_by(&self.solver, |a, b|
        if a.index < b.index || a.index == b.index && (a.length < b.length || a.length == b.length && a.point < b.point) {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        let mut i = 0;
        while i < marks.len() {
            let i0 = i;
            let index = marks[i0].index;
            i += 1;
            while i < marks.len() && marks[i].index == index {
                i += 1;
            }

            if i0 + 1 == i {
                let e0 = edges[index];
                let p = marks[i0].point;
                edges[index] = ShapeEdge::create_and_validate(e0.x_segment.a, p, e0.count);
                edges.push(ShapeEdge::create_and_validate(p, e0.x_segment.b, e0.count));
            } else {
                Self::multi_split_edge(&marks[i0..i], edges);
            }
        }

        edges.smart_sort_by(&self.solver, |a, b| a.x_segment.cmp(&b.x_segment));

        edges.merge_if_needed();
    }

    fn multi_split_edge(marks: &[LineMark], edges: &mut Vec<ShapeEdge>) {
        let index = marks[0].index;
        let mut p = marks[0].point;
        let mut l = marks[0].length;

        let n = marks.len();
        let e0 = edges[index];

        edges[index] = ShapeEdge::create_and_validate(e0.x_segment.a, p, e0.count);

        let mut j = 1;
        while j < n {
            let mj = &marks[j];
            if l != mj.length || p != mj.point {
                edges.push(ShapeEdge::create_and_validate(p, mj.point, e0.count));
                p = mj.point;
                l = mj.length;
            }
            j += 1;
        }
        edges.push(ShapeEdge::create_and_validate(p, e0.x_segment.b, e0.count));
    }
}