use std::cmp::Ordering;
use crate::split::cross_solver::{CrossResult, ScanCrossSolver};
use crate::split::line_mark::LineMark;
use crate::split::shape_edge::ShapeEdge;

pub(super) struct SimplePreSplitSolver;

impl SimplePreSplitSolver {
    pub(super) fn split(max_repeat_count: usize, edges: &mut Vec<ShapeEdge>) -> bool {
        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut split_count = 0;

        while need_to_fix && split_count < max_repeat_count {
            need_to_fix = false;

            marks.clear();
            split_count += 1;

            let n = edges.len();
            let mut extra_space = 0;

            for i in 0..n - 1 {
                let ei = &edges[i].x_segment;
                for j in i + 1..n {
                    let ej = &edges[j].x_segment;
                    if ei.b.x < ej.a.x {
                        break;
                    }

                    let test_x = ScanCrossSolver::test_x(ei, ej);
                    let test_y = ScanCrossSolver::test_y(ei, ej);

                    if test_x || test_y {
                        continue;
                    }

                    if let Some(cross) = ScanCrossSolver::pre_cross(ei, ej) {
                        match cross {
                            CrossResult::PureExact(p) => {
                                extra_space += 3;

                                let li = ei.a.sqr_distance(p);
                                let lj = ej.a.sqr_distance(p);

                                marks.push(LineMark { index: i, length: li, point: p });
                                marks.push(LineMark { index: j, length: lj, point: p });
                            }
                            CrossResult::PureRound(p) => {
                                extra_space += 3;

                                let li = ei.a.sqr_distance(p);
                                let lj = ej.a.sqr_distance(p);

                                marks.push(LineMark { index: i, length: li, point: p });
                                marks.push(LineMark { index: j, length: lj, point: p });
                                need_to_fix = true;
                            }
                            CrossResult::TargetEndExact(p) => {
                                extra_space += 1;

                                let lj = ej.a.sqr_distance(p);
                                marks.push(LineMark { index: j, length: lj, point: p });
                            }
                            CrossResult::TargetEndRound(p) => {
                                extra_space += 1;

                                let lj = ej.a.sqr_distance(p);
                                marks.push(LineMark { index: j, length: lj, point: p });
                                need_to_fix = true;
                            }
                            CrossResult::OtherEndExact(p) => {
                                extra_space += 1;

                                let li = ei.a.sqr_distance(p);

                                marks.push(LineMark { index: i, length: li, point: p });
                            }
                            CrossResult::OtherEndRound(p) => {
                                extra_space += 1;

                                let li = ei.a.sqr_distance(p);

                                marks.push(LineMark { index: i, length: li, point: p });
                                need_to_fix = true;
                            }
                            _ => {
                                panic!("Can not be here");
                            }
                        }
                    } else {
                        continue;
                    }
                }
            }

            if marks.is_empty() {
                return false;
            }

            marks.sort_unstable_by(|a, b|
                if a.index < b.index || a.index == b.index && a.length < b.length {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            );

            if !need_to_fix {
                edges.reserve_exact(extra_space);
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

            edges.sort_by(|a, b| a.x_segment.cmp(&b.x_segment));

            Self::merge(edges);
        }

        need_to_fix
    }

    fn merge(edges: &mut Vec<ShapeEdge>) {
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