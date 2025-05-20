use alloc::vec;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_key_sort::bin_key::index::{BinKey, BinLayout};
use i_key_sort::sort::key_sort::Bin;
use crate::core::solver::Solver;
use crate::segm::segment::Segment;
use crate::segm::winding::WindingCount;
use crate::split::cross_solver::{CrossType, CrossSolver, EndMask};
use crate::split::line_mark::LineMark;
use crate::geom::x_segment::XSegment;
use crate::segm::merge::ShapeSegmentsMerge;
use crate::util::sort::SmartBinSort;

#[derive(Clone)]
pub(crate) struct SplitSolver {
    pub(super) marks: Vec<LineMark>,
}

impl SplitSolver {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self { marks: Vec::new() }
    }

    #[inline]
    pub(crate) fn split_segments<C: WindingCount>(&mut self, segments: &mut Vec<Segment<C>>, solver: &Solver) -> bool {
        if segments.is_empty() {
            return false;
        }
        segments.smart_bin_sort_by(solver, |a, b| a.x_segment.cmp(&b.x_segment));
        let any_merged = segments.merge_if_needed();
        let any_intersection = self.split(segments, solver);

        any_merged | any_intersection
    }

    #[inline]
    fn split<C: WindingCount>(&mut self, segments: &mut Vec<Segment<C>>, solver: &Solver) -> bool {
        let is_list = solver.is_list_split(segments);
        let snap_radius = solver.snap_radius();
        if is_list {
            return self.list_split(snap_radius, segments, solver);
        }

        let is_fragmentation = solver.is_fragmentation_required(segments);

        if is_fragmentation {
            self.fragment_split(snap_radius, segments, solver)
        } else {
            self.tree_split(snap_radius, segments, solver)
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

    pub(super) fn apply<C: WindingCount>(&mut self, segments: &mut Vec<Segment<C>>, need_to_fix: bool, solver: &Solver) {
        self.sort_and_filter_marks(segments, solver);
        let min = segments[0].x_segment.a.x;
        let mut max = segments[0].x_segment.b.x;

        for m in self.marks.iter() {
            max = max.max(m.point.x);
        }

        for s in segments.iter().skip(1) {
            max = max.max(s.x_segment.b.x);
        }

        let new_len = segments.len() + self.marks.len();
        if new_len <= 16 {
            return self.one_bin_merge(segments, need_to_fix);
        };

        let layout = if let Some(layout) = BinLayout::new(min..max, new_len) {
            layout
        } else {
            return self.one_bin_merge(segments, need_to_fix);
        };

        let mut bins = Self::init_bins(max, &layout, &self.marks, segments);

        let empty = Segment {
            x_segment: XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO },
            count: C::new(0, 0),
        };

        let mut buffer = vec![empty; new_len];

        let slice = buffer.as_mut_slice();

        // split segments

        let mut j = 0;
        let mut mj = self.marks[0];
        for (i, s) in segments.iter().enumerate() {
            // TODO early out
            if i != mj.index {
                // not modified
                let bin_index = s.bin_index(&layout);
                let bin = unsafe { bins.get_unchecked_mut(bin_index) };
                *unsafe { slice.get_unchecked_mut(bin.data) } = *s;
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
                while j < self.marks.len() {
                    mj = self.marks[j];
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

        segments.copy_and_merge(&buffer);
    }

    #[inline]
    fn init_bins<C: Send>(max: i32, layout: &BinLayout<i32>, marks: &[LineMark], segments: &[Segment<C>]) -> Vec<Bin> {
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
    fn one_bin_merge<C: WindingCount>(&mut self, segments: &mut Vec<Segment<C>>, need_to_fix: bool) {
        if need_to_fix {
            segments.reserve(self.marks.len());
        } else {
            segments.reserve_exact(self.marks.len());
        }

        let mut i = 0;
        while i < self.marks.len() {
            let index = self.marks[i].index;
            let i0 = i;
            i += 1;
            while i < self.marks.len() && self.marks[i].index == index {
                i += 1;
            }

            if i0 + 1 == i {
                let e0 = unsafe { segments.get_unchecked_mut(index) };
                let p = self.marks[i0].point;
                let b = e0.x_segment.b;
                let count = e0.count;
                *e0 = Segment::create_and_validate(e0.x_segment.a, p, count);
                segments.push(Segment::create_and_validate(p, b, count));
            } else {
                Self::multi_split_edge(&self.marks[i0..i], segments);
            }
        }

        segments.sort_unstable_by(|a, b| a.x_segment.cmp(&b.x_segment));

        segments.merge_if_needed();
    }

    #[inline]
    fn multi_split_edge<C: WindingCount>(marks: &[LineMark], segments: &mut Vec<Segment<C>>) {
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
    fn sort_and_filter_marks<C: Send>(&mut self, segments: &[Segment<C>], solver: &Solver) {
        self.marks.smart_bin_sort_by(solver, |a, b| a.index.cmp(&b.index).then(a.point.cmp(&b.point)));
        self.marks.dedup();

        let mut i = 1;
        let mut i0 = 0;
        let mut m0_index = self.marks[0].index;

        while i < self.marks.len() {
            let mi_index = self.marks[i].index;
            if mi_index == m0_index {
                i += 1;
                continue;
            }

            if i0 + 1 < i {
                Self::sort_sub_marks(&mut self.marks[i0..i], segments);
            }

            m0_index = mi_index;
            i0 = i;
            i += 1;
        }

        if i0 + 1 < i {
            Self::sort_sub_marks(&mut self.marks[i0..i], segments);
        }
    }

    #[inline]
    fn sort_sub_marks<C: Send>(marks: &mut [LineMark], segments: &[Segment<C>]) {
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
                let (y0, y1) = Self::y_range(j0, j, segments[m0.index].x_segment, marks);
                Self::sort_sub_marks_by_y(y0, y1, &mut marks[j0..j]);
            }

            x0 = xi;
            j0 = j;
            j += 1;
        }

        if j0 + 1 < j {
            let (y0, y1) = Self::y_range(j0, j, segments[m0.index].x_segment, marks);
            Self::sort_sub_marks_by_y(y0, y1, &mut marks[j0..j]);
        }
    }

    #[inline]
    fn y_range(j0: usize, j1: usize, s: XSegment, marks: &[LineMark]) -> (i32, i32) {
        let y0 = if j0 == 0 { s.a.y } else { marks[j0 - 1].point.y };
        let y1 = if j1 == marks.len() { s.b.y } else { marks[j1].point.y };
        (y0, y1)
    }

    #[inline]
    fn sort_sub_marks_by_y(y0: i32, y1: i32, marks: &mut [LineMark]) {
        // The x-coordinate is the same for every point
        // By default, the range should be sorted in ascending order by the y-coordinate.
        if y0 > y1 {
            // reverse the order to sort the range in descending order by the y-coordinate.
            marks.reverse();
        }
    }
}