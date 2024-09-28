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
                marks.push(LineMark { index: i, point: cross.point });
                marks.push(LineMark { index: j, point: cross.point });
            }
            CrossType::TargetEnd => {
                marks.push(LineMark { index: j, point: cross.point });
            }
            CrossType::OtherEnd => {
                marks.push(LineMark { index: i, point: cross.point });
            }
            CrossType::Overlay => {
                let mask = CrossSolver::collinear(ei, ej);
                if mask == 0 { return false; }

                if mask.is_target_a() {
                    marks.push(LineMark { index: j, point: ei.a });
                }

                if mask.is_target_b() {
                    marks.push(LineMark { index: j, point: ei.b });
                }

                if mask.is_other_a() {
                    marks.push(LineMark { index: i, point: ej.a });
                }

                if mask.is_other_b() {
                    marks.push(LineMark { index: i, point: ej.b });
                }
            }
        }

        cross.is_round
    }

    pub(super) fn apply(&self, marks: &mut Vec<LineMark>, segments: Vec<Segment>, need_to_fix: bool) -> Vec<Segment> {
        self.sort_and_filter_marks(marks, &segments);
        let min = segments[0].x_segment.a.x;
        let mut max = segments[0].x_segment.b.x;

        for m in marks.iter() {
            max = max.max(m.point.x);
        }

        for s in segments.iter() {
            max = max.max(s.x_segment.b.x);
        }

        let new_len = segments.len() + marks.len();
        if new_len <= 16 {
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

        let mut buffer = vec![empty; new_len];

        // let mut buffer = vec![empty; new_len];
        let slice = buffer.as_mut_slice();

        // split segments

        let mut j = 0;
        let mut mj = marks[0];
        for (i, s) in segments.iter().enumerate() {
            // TODO early out
            if i != mj.index {
                // not modified
                let bin_index = s.bin_index(&layout);
                let bin = unsafe { bins.get_unchecked_mut(bin_index) };
                *unsafe { slice.get_unchecked_mut(bin.data) } = s.clone();
                bin.data += 1;
            } else {
                let s0 = Segment::create_and_validate(s.x_segment.a, mj.point, s.count);

                // add first
                let s0_bin_index = s0.bin_index(&layout);
                let s0_bin = unsafe { bins.get_unchecked_mut(s0_bin_index) };
                *unsafe { slice.get_unchecked_mut(s0_bin.data) } = s0;
                s0_bin.data += 1;


                // add middle
                let mut m0 = mj;
                j += 1;
                while j < marks.len() {
                    mj = marks[j];
                    if m0.index != mj.index {
                        break;
                    }

                    let sj = Segment::create_and_validate(m0.point, mj.point, s.count);
                    let sj_bin_index = sj.bin_index(&layout);
                    let sj_bin = unsafe { bins.get_unchecked_mut(sj_bin_index) };
                    *unsafe { slice.get_unchecked_mut(sj_bin.data) } = sj;
                    sj_bin.data += 1;

                    m0 = mj;
                    j += 1;
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
        let mut mj = marks[0];
        for (i, s) in segments.iter().enumerate() {
            if i != mj.index {
                let bin_index = layout.index(s.x_segment.a.x);
                unsafe { bins.get_unchecked_mut(bin_index) }.data += 1;
            } else {
                // add first
                let min_x = s.x_segment.a.x.min(mj.point.x);
                unsafe { bins.get_unchecked_mut(layout.index(min_x)) }.data += 1;

                // add middle
                let mut m0 = mj;
                j += 1;
                while j < marks.len() {
                    mj = marks[j];
                    if m0.index != mj.index {
                        break;
                    }

                    let min_x = m0.point.x.min(mj.point.x);

                    unsafe { bins.get_unchecked_mut(layout.index(min_x)) }.data += 1;

                    m0 = mj;
                    j += 1;
                }

                // add last
                let min_x = m0.point.x.min(s.x_segment.b.x);
                unsafe { bins.get_unchecked_mut(layout.index(min_x)) }.data += 1;
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
    fn multi_split_edge(marks: &[LineMark], segments: &mut Vec<Segment>) {
        let mut iter = marks.iter();
        let m0 = iter.next().unwrap();

        let mut p = m0.point;

        let e0 = unsafe { segments.get_unchecked_mut(m0.index) };

        let b = e0.x_segment.b;
        let count = e0.count;
        *e0 = Segment::create_and_validate(e0.x_segment.a, p, count);

        for mj in iter {
            segments.push(Segment::create_and_validate(p, mj.point, count));
            p = mj.point;
        }

        segments.push(Segment::create_and_validate(p, b, count));
    }

    #[inline]
    fn sort_and_filter_marks(&self, marks: &mut Vec<LineMark>, segments: &[Segment]) {
        marks.smart_bin_sort_by(&self.solver, |a, b| a.index.cmp(&b.index).then(a.point.cmp(&b.point)));
        marks.dedup();

        let mut i = 1;
        let mut i0 = 0;
        let mut m0_index = marks[0].index;

        while i < marks.len() {
            let mi_index = marks[i].index;
            if mi_index == m0_index {
                i += 1;
                continue;
            }

            if i0 + 1 < i {
                Self::sort_sub_marks(&mut marks[i0..i], segments);
            }

            m0_index = mi_index;
            i0 = i;
            i += 1;
        }

        if i0 + 1 < i {
            Self::sort_sub_marks(&mut marks[i0..i], segments);
        }
    }

    fn sort_sub_marks(marks: &mut [LineMark], segments: &[Segment]) {
        let mut j0 = 0;
        let mut j = 1;

        let m0 = marks[0];
        let mut x0 = m0.point.x;
        while j < marks.len() {
            let xi = marks[j].point.x;
            if x0 == xi {
                j += 1;
                continue;
            }

            if j0 + 1 < j {
                let s = segments[m0.index].x_segment;
                let y0 = if j0 == 0 { s.a.y } else { marks[j0 - 1].point.y };
                let y1 = if j == marks.len() { s.b.y } else { marks[j].point.y };
                Self::sort_sub_marks_by_y(y0, y1, &mut marks[j0..j]);
            }

            x0 = xi;
            j0 = j;
            j += 1;
        }

        if j0 + 1 < j {
            let s = segments[m0.index].x_segment;
            let y0 = if j0 == 0 { s.a.y } else { marks[j0 - 1].point.y };
            let y1 = if j == marks.len() { s.b.y } else { marks[j].point.y };
            Self::sort_sub_marks_by_y(y0, y1, &mut marks[j0..j]);
        }
    }

    #[inline]
    fn sort_sub_marks_by_y(y0: i32, y1: i32, marks: &mut [LineMark]) {
        // the goal is to sort close to y0 and far from y1
        let y0 = y0 as i64;
        let y1 = y1 as i64;
        marks.sort_unstable_by(|ma, mb| {
            let ya = ma.point.y as i64;
            let yb = mb.point.y as i64;
            let sa = (y0 - ya).abs() - (y0 - yb).abs();
            let sb = (y1 - ya).abs() - (y1 - yb).abs();
            sa.cmp(&sb)
        });
    }
}