use std::cmp::Ordering;
use i_float::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};
use i_key_sort::key_sort::Bin;
use crate::core::solver::Solver;
use crate::segm::segment::{Segment, ShapeEdgesMerge};
use crate::segm::shape_count::ShapeCount;
use crate::split::cross_solver::{CrossType, CrossSolver, EndMask};
use crate::split::line_mark::LineMark;
use crate::segm::x_segment::XSegment;
use crate::sort::SmartBinSort;

pub(crate) struct SplitSolver {
    pub(super) solver: Solver,
}

impl SplitSolver {
    pub(crate) fn new(solver: Solver) -> Self {
        Self { solver }
    }

    pub(crate) fn split(&mut self, segments: Vec<Segment>) -> Vec<Segment> {
        let is_list = self.solver.is_list_split(&segments);

        if is_list {
            self.list_split(segments)
        } else {
            self.tree_split(segments)
        }
    }

    pub(super) fn cross(i: usize, j: usize, ei: &XSegment, ej: &XSegment, marks: &mut Vec<LineMark>, radius: i64) -> bool {
        let cross = if let Some(cross) = CrossSolver::cross(ei, ej, radius) {
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

    pub(super) fn apply(&self, marks: &mut Vec<LineMark>, segments: Vec<Segment>, need_to_fix: bool) -> Vec<Segment> {
        marks.smart_bin_sort_by(&self.solver, |a, b|
        if a.index < b.index || a.index == b.index && (a.length < b.length || a.length == b.length && a.point < b.point) {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        let min = segments[0].x_segment.a.x;
        let mut max = segments[segments.len() - 1].x_segment.a.x;
        let mut mark_iter = marks.iter();
        let mut m0 = mark_iter.next().unwrap();
        let mut m_count = 0;
        max = max.max(m0.point.x);
        for mark in mark_iter {
            if !mark.eq(m0) {
                m_count += 1;
                max = max.max(mark.point.x);
                m0 = mark;
            }
        }

        let new_len = segments.len() + m_count;
        if new_len <= 16 { // TODO up to equal
            return Self::one_bin_merge(marks, segments, need_to_fix);
        };

        let layout = if let Some(layout) = BinLayout::new(min..max, new_len) {
            layout
        } else {
            return Self::one_bin_merge(marks, segments, need_to_fix);
        };

        let mut bins = Self::init_bins(max, &layout, marks, &segments);

        let empty = Segment {
            x_segment: XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO },
            count: ShapeCount { subj: 0, clip: 0 },
        };

        let new_len = segments.len() + marks.len();
        let mut buffer = Vec::new();
        buffer.reserve_exact(new_len);
        for _ in 0..new_len {
            buffer.push(empty);
        }
        let slice = buffer.as_mut_slice();

        // let mut buffer = vec![empty; new_len];

        // split segments

        let mut j = 0;
        let mut m0 = marks[0];
        let mut mj = m0;
        for (i, s) in segments.into_iter().enumerate() {
            // TODO early out
            if i != mj.index {
                // not modified
                let bin_index = s.bin_index(&layout);
                let bin = unsafe { bins.get_unchecked_mut(bin_index) };
                *unsafe { slice.get_unchecked_mut(bin.data) } = s;
                bin.data += 1;
            } else {
                let s0 = Segment::create_and_validate(s.x_segment.a, mj.point, s.count);

                // add first
                let s0_bin_index = s0.bin_index(&layout);
                let s0_bin = unsafe { bins.get_unchecked_mut(s0_bin_index) };
                *unsafe { slice.get_unchecked_mut(s0_bin.data) } = s0;
                s0_bin.data += 1;


                // add middle
                j += 1;
                while j < marks.len() {
                    mj = marks[j];
                    if m0.index != mj.index {
                        break;
                    }

                    j += 1;
                    if m0.point == mj.point {
                        continue;
                    }

                    let sj = Segment::create_and_validate(m0.point, mj.point, s.count);
                    let sj_bin_index = sj.bin_index(&layout);
                    let sj_bin = unsafe { bins.get_unchecked_mut(sj_bin_index) };
                    *unsafe { slice.get_unchecked_mut(sj_bin.data) } = sj;
                    sj_bin.data += 1;

                    m0 = mj;
                }

                // add last
                let sj = Segment::create_and_validate(m0.point, s.x_segment.b, s.count);
                let sj_bin_index = sj.bin_index(&layout);
                let sj_bin = unsafe { bins.get_unchecked_mut(sj_bin_index) };
                *unsafe { slice.get_unchecked_mut(sj_bin.data) } = sj;
                sj_bin.data += 1;
            }
        }

        // sort by bins
        for bin in bins.iter() {
            let start = bin.offset;
            let end = bin.data;
            if start < end {
                slice[start..end].sort_by(|a, b| a.x_segment.cmp(&b.x_segment));
            }
        }

        buffer.merge_if_needed();

        buffer
    }

    #[inline]
    fn init_bins(max: i32, layout: &BinLayout<i32>, marks: &[LineMark], segments: &[Segment]) -> Vec<Bin> {
        let bin_count = layout.index(max) + 1;
        let mut bins = vec![Bin { offset: 0, data: 0 }; bin_count];

        // move new

        let mut j = 0;
        let mut m0 = marks[0];
        let mut mj = m0;
        for (i, s) in segments.iter().enumerate() {
            if i != mj.index {
                let bin_index = layout.index(s.x_segment.a.x);
                unsafe { bins.get_unchecked_mut(bin_index) }.data += 1;
            } else {
                // add first
                let bin_index = layout.index(s.x_segment.a.x.min(mj.point.x));
                unsafe { bins.get_unchecked_mut(bin_index) }.data += 1;

                // add middle
                j += 1;
                while j < marks.len() {
                    mj = marks[j];
                    if m0.index != mj.index {
                        break;
                    }

                    j += 1;
                    if m0.point == mj.point {
                        continue;
                    }

                    let min_x = m0.point.x.min(mj.point.x);

                    let bin_index = layout.index(min_x);
                    unsafe { bins.get_unchecked_mut(bin_index) }.data += 1;

                    m0 = mj;
                }

                // add last
                let min_x = m0.point.x.min(s.x_segment.b.x);
                let bin_index = layout.index(min_x);
                unsafe { bins.get_unchecked_mut(bin_index) }.data += 1;
            }
        }

        let mut offset = 0;
        for bin in bins.iter_mut() {
            let next_offset = offset + bin.data;
            bin.offset = offset;
            bin.data = offset;
            offset = next_offset;
        }

        bins
    }

    #[inline]
    fn one_bin_merge(marks: &mut [LineMark], mut segments: Vec<Segment>, need_to_fix: bool) -> Vec<Segment> {
        if need_to_fix {
            segments.reserve(marks.len());
        } else {
            segments.reserve_exact(marks.len());
        }

        let mut i = 0;
        while i < marks.len() {
            let index = marks[i].index;
            let i0 = i;
            i += 1;
            while i < marks.len() && marks[i].index == index {
                i += 1;
            }

            if i0 + 1 == i {
                let e0 = unsafe { segments.get_unchecked_mut(index) };
                let p = marks[i0].point;
                let b = e0.x_segment.b;
                let count = e0.count;
                *e0 = Segment::create_and_validate(e0.x_segment.a, p, count);
                segments.push(Segment::create_and_validate(p, b, count));
            } else {
                Self::multi_split_edge(&marks[i0..i], &mut segments);
            }
        }

        segments.sort_unstable_by(|a, b| a.x_segment.cmp(&b.x_segment));

        segments.merge_if_needed();

        segments
    }

    #[inline]
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