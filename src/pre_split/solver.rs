use std::cmp::Ordering;
use crate::core::solver::Solver;
use crate::pre_split::line_mark::LineMark;
use crate::sort::SmartSort;
use crate::split::cross_solver::{CrossResult, ScanCrossSolver};
use crate::split::shape_edge::ShapeEdge;
use crate::x_segment::XSegment;

pub(crate) struct PreSplitSolver;

impl PreSplitSolver {
    pub(crate) fn split(solver: Solver, edges: &mut Vec<ShapeEdge>) -> bool {
        let count = edges.len();
        if count > solver.pre_split_skip_threshold {
            return true;
        }

        if count <= solver.pre_split_multithread_threshold {
            Self::single_split(solver.pre_split_max_iterations, edges)
        } else {
            #[cfg(feature = "allow_multithreading")]
            {
                Self::multiple_split(solver.pre_split_max_iterations, edges)
            }

            #[cfg(not(feature = "allow_multithreading"))]
            {
                true
            }
        }
    }

    pub(super) fn cross(i: usize, j: usize, ei: &XSegment, ej: &XSegment, need_to_fix: &mut bool, marks: &mut Vec<LineMark>) {
        if let Some(cross) = ScanCrossSolver::pre_cross(ei, ej) {
            match cross {
                CrossResult::PureExact(p) => {
                    let li = ei.a.sqr_distance(p);
                    let lj = ej.a.sqr_distance(p);

                    marks.push(LineMark { index: i, length: li, point: p });
                    marks.push(LineMark { index: j, length: lj, point: p });
                }
                CrossResult::PureRound(p) => {
                    let li = ei.a.sqr_distance(p);
                    let lj = ej.a.sqr_distance(p);

                    marks.push(LineMark { index: i, length: li, point: p });
                    marks.push(LineMark { index: j, length: lj, point: p });
                    *need_to_fix = true;
                }
                CrossResult::TargetEndExact(p) => {
                    let lj = ej.a.sqr_distance(p);
                    marks.push(LineMark { index: j, length: lj, point: p });
                }
                CrossResult::TargetEndRound(p) => {
                    let lj = ej.a.sqr_distance(p);
                    marks.push(LineMark { index: j, length: lj, point: p });
                    *need_to_fix = true;
                }
                CrossResult::OtherEndExact(p) => {
                    let li = ei.a.sqr_distance(p);

                    marks.push(LineMark { index: i, length: li, point: p });
                }
                CrossResult::OtherEndRound(p) => {
                    let li = ei.a.sqr_distance(p);

                    marks.push(LineMark { index: i, length: li, point: p });
                    *need_to_fix = true;
                }
                _ => {
                    panic!("Can not be here");
                }
            }
        }
    }

    pub(super) fn apply(need_to_fix: bool, marks: &mut Vec<LineMark>, edges: &mut Vec<ShapeEdge>) {
        marks.smart_sort_by(|a, b|
            if a.index < b.index || a.index == b.index && a.length < b.length {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );

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

        let mut i = 0;
        while i < edges.len() {
            let ei = edges[i];
            let mut count = ei.count;
            let mut is_modified = false;
            while i + 1 < edges.len() && ei.x_segment == edges[i + 1].x_segment {
                let c = edges.remove(i + 1).count;
                count = count.add(c);
                is_modified = true;
            }

            if is_modified || count.is_empty() {
                if count.is_empty() {
                    edges.remove(i);
                } else {
                    edges[i].count = count;
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }
}