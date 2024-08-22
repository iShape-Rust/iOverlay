use std::cmp::Ordering;
use crate::core::solver::Solver;
use crate::segm::segment::{Segment, ShapeEdgesMerge};
use crate::sort::SmartSort;
use crate::split::cross_solver::{CrossType, CrossSolver, EndMask};
use crate::split::line_mark::LineMark;
use crate::segm::x_segment::XSegment;

pub(crate) struct SplitSolver {
    pub(super) solver: Solver,
}

impl SplitSolver {
    pub(crate) fn new(solver: Solver) -> Self {
        Self { solver }
    }

    pub(crate) fn split(&self, edges: &mut Vec<Segment>) {
        let is_list = self.solver.is_list_split(edges);

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

        cross.is_round
    }

    pub(super) fn apply(&self, marks: &mut Vec<LineMark>, edges: &mut Vec<Segment>) {
        marks.smart_sort_by(&self.solver, |a, b|
        if a.index < b.index || a.index == b.index && (a.length < b.length || a.length == b.length && a.point < b.point) {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        edges.reserve(marks.len());

        let mut i = 0;
        while i < marks.len() {
            let index = marks[i].index;
            let i0 = i;
            i += 1;
            while i < marks.len() && marks[i].index == index {
                i += 1;
            }

            if i0 + 1 == i {
                let e0 = unsafe { edges.get_unchecked_mut(index) };
                let p = marks[i0].point;
                let b = e0.x_segment.b;
                let count = e0.count;
                *e0 = Segment::create_and_validate(e0.x_segment.a, p, count);
                edges.push(Segment::create_and_validate(p, b, count));
            } else {
                Self::multi_split_edge(&marks[i0..i], edges);
            }
        }

        edges.smart_sort_by(&self.solver, |a, b| a.x_segment.cmp(&b.x_segment));

        edges.merge_if_needed();
    }

    fn multi_split_edge(marks: &[LineMark], edges: &mut Vec<Segment>) {
        let mut iter = marks.iter();
        let m0 = iter.next().unwrap();

        let mut p = m0.point;
        let mut l = m0.length;

        let e0 = unsafe { edges.get_unchecked_mut(m0.index) };

        let b = e0.x_segment.b;
        let count = e0.count;
        *e0 = Segment::create_and_validate(e0.x_segment.a, p, count);

        for mj in iter {
            if l != mj.length || p != mj.point {
                edges.push(Segment::create_and_validate(p, mj.point, count));
                p = mj.point;
                l = mj.length;
            }
        }

        edges.push(Segment::create_and_validate(p, b, count));
    }
}