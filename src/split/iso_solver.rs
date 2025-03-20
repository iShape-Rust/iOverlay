use crate::core::solver::Solver;
use crate::geom::x_segment::XSegment;
use crate::segm::merge::ShapeSegmentsMerge;
use crate::segm::segment::Segment;
use crate::segm::winding_count::WindingCount;
use crate::split::iso_fragment::{DgLine, HzLine, VrLine};
use crate::split::line_mark::LineMark;
use crate::split::solver::SplitSolver;
use crate::util::sort::SmartBinSort;
use i_float::int::point::IntPoint;
use i_key_sort::index::{BinKey};

pub(crate) trait IsoSplitSegments<C: WindingCount> {
    fn iso_split_segments(self, solver: Solver) -> Vec<Segment<C>>;
}

impl<C: WindingCount> IsoSplitSegments<C> for Vec<Segment<C>> {
    #[inline]
    fn iso_split_segments(self, solver: Solver) -> Vec<Segment<C>> {
        let mut segments = self;
        segments.smart_bin_sort_by(&solver, |a, b| a.x_segment.cmp(&b.x_segment));
        segments.merge_if_needed();
        SplitSolver::new(solver).iso_split(segments)
    }
}

impl SplitSolver {
    pub(super) fn iso_split<C: WindingCount>(&self, segments: Vec<Segment<C>>) -> Vec<Segment<C>> {
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for s in segments.iter() {
            min_y = min_y.min(s.x_segment.a.y);
            max_y = max_y.max(s.x_segment.a.y);
            min_y = min_y.min(s.x_segment.b.y);
            max_y = max_y.max(s.x_segment.b.y);
        }

        let mut marks = Vec::new();
        self.split_90_deg(min_y, max_y, &segments, &mut marks);

        if marks.is_empty() {
            return segments;
        }

        self.apply(&mut marks, segments, false)
    }

    fn split_90_deg<C>(&self, min_y: i32, max_y: i32, segments: &[Segment<C>], marks: &mut Vec<LineMark>) {
        let max_power = segments.len().ilog2() >> 1;

        let height = max_y - min_y;
        let log = height.ilog2();
        let power = if log > max_power {
            log - max_power
        } else {
            1
        };

        let row_layout = RowLayout::new(min_y, max_y, power);
        let mut rows = vec![Vec::<HzLine>::new(); row_layout.max_index + 1];

        for (si, s) in segments.iter().enumerate() {
            if s.x_segment.is_horizontal() {
                let row_index = row_layout.index(s.x_segment.a.y);
                let hz = HzLine::new(si, &s.x_segment);
                rows[row_index].add(hz);
            } else {
                let (min_y, max_y) = if s.x_segment.a.y < s.x_segment.b.y {
                    (s.x_segment.a.y, s.x_segment.b.y)
                } else {
                    (s.x_segment.b.y, s.x_segment.a.y)
                };
                let min_i = row_layout.index(min_y);
                let max_i = row_layout.index(max_y);

                let vr = VrLine::new(si, &s.x_segment);

                for row in rows[min_i..=max_i].iter_mut() {
                    row.cross(&vr, marks);
                }
            }
        }
    }


    #[inline]
    fn cross_vr_dg(vr: &VrLine, dg: &DgLine, marks: &mut Vec<LineMark>) {
        let x = vr.x;
        let y = dg.y_for_x(x);

        let point = IntPoint { x, y };

        if y < vr.min_y || vr.max_y < y {
            return;
        }

        if vr.min_y < y && y < vr.max_y {
            marks.push(LineMark {
                index: vr.index,
                point,
            });
        }

        if dg.contains_exclude_border(point) {
            marks.push(LineMark {
                index: dg.index,
                point,
            });
        }
    }

    #[inline]
    fn cross_hz_dg(hz: &HzLine, dg: &DgLine, marks: &mut Vec<LineMark>) {
        let y = hz.y;
        let x = dg.x_for_y(y);

        let point = IntPoint { x, y };

        if x < hz.min_x || hz.max_x < x {
            return;
        }

        if hz.min_x < x && x < hz.max_x {
            marks.push(LineMark {
                index: hz.index,
                point,
            });
        }

        if dg.contains_exclude_border(point) {
            marks.push(LineMark {
                index: dg.index,
                point,
            });
        }
    }

    #[inline]
    fn cross_dg_dg(d0: &DgLine, d1: &DgLine, marks: &mut Vec<LineMark>) {
        let kk = d0.k - d1.k;
        let bb = d1.b - d0.b;
        let x = bb / kk;
        let y = d0.k * x + d0.b;
        let point = IntPoint { x, y };

        if d0.contains_exclude_border(point) {
            marks.push(LineMark {
                index: d0.index,
                point,
            });
        }

        if d1.contains_exclude_border(point) {
            marks.push(LineMark {
                index: d1.index,
                point,
            });
        }
    }
}

impl HzLine {
    #[inline(always)]
    fn new(index: usize, x_segment: &XSegment) -> Self {
        HzLine {
            index,
            y: x_segment.a.y,
            min_x: x_segment.a.x,
            max_x: x_segment.b.x,
        }
    }

    #[inline]
    fn cross_vz_vr(&self, vr: &VrLine, marks: &mut Vec<LineMark>) {
        let x = vr.x;
        let y = self.y;

        let point = IntPoint { x, y };

        if vr.min_y < y && y < vr.max_y {
            marks.push(LineMark {
                index: vr.index,
                point,
            })
        }

        if self.min_x < x && x < self.max_x {
            marks.push(LineMark {
                index: self.index,
                point,
            })
        }
    }
}

impl DgLine {
    #[inline]
    pub(super) fn y_for_x(&self, x: i32) -> i32 {
        (self.k * x).wrapping_add(self.b)
    }

    #[inline]
    fn x_for_y(&self, y: i32) -> i32 {
        y.wrapping_sub(self.b) * self.k
    }

    #[inline]
    fn contains_exclude_border(&self, point: IntPoint) -> bool {
        let xx = self.full.min_x < point.x && point.x < self.full.max_x;
        let yy = self.full.min_y < point.y && point.y < self.full.max_y;
        xx && yy
    }
}

type Row = Vec<HzLine>;

trait HzRow {
    fn add(&mut self, hz: HzLine);
    fn cross(&mut self, vr: &VrLine, marks: &mut Vec<LineMark>);
}

struct RowLayout {
    min_y: i32,
    power: u32,
    max_index: usize,
}

impl HzRow for Row {
    #[inline]
    fn add(&mut self, hz: HzLine) {
        if let Some(index) = self.iter().position(|it|it.max_x < hz.min_x) {
            self[index] = hz;
        } else {
            self.push(hz);
        }
    }

    #[inline]
    fn cross(&mut self, vr: &VrLine, marks: &mut Vec<LineMark>) {
        if self.is_empty() {
            return;
        }
        let mut last_index = self.len() - 1;
        let mut i = 0;
        while i < last_index {
            let hz = unsafe { self.get_unchecked_mut(i) };
            if hz.max_x < vr.x {
                *hz = unsafe { self.get_unchecked(last_index).clone() };
                last_index -= 1;
            }

            hz.cross_vz_vr(&vr, marks);

            i += 1;
        }
        let hz = unsafe { self.get_unchecked_mut(last_index) };
        if hz.max_x < vr.x {
            last_index -= 1;
        } else {
            hz.cross_vz_vr(&vr, marks);
        }

        self.truncate(last_index);
    }
}

impl RowLayout {
    fn new(min_y: i32, max_y: i32, power: u32) -> Self {
        let max_index = ((max_y - min_y) >> power) as usize;
        Self {
            min_y,
            power,
            max_index,
        }
    }

    #[inline]
    fn index(&self, y: i32) -> usize {
        ((y - self.min_y) >> self.power) as usize
    }

    #[inline]
    fn safe_index(&self, y: i32) -> usize {
        if y < self.min_y {
            return 0;
        }
        self.index(y).min(self.max_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::split::iso_solver::RowLayout;

    #[test]
    fn test_0() {
        let layout = RowLayout::new(-2, 2, 1);
        let i0 = layout.safe_index(-3);
        let i1 = layout.safe_index(-2);
        assert!(i0 <= i1);
    }
}
