use i_float::int::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};
use crate::core::solver::Solver;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use crate::segm::winding_count::WindingCount;
use crate::split::grid_layout::GridLayout;
use crate::split::iso_fragment::{DgLine, Group, HzLine, IsoFragmentBuffer, VrLine};
use crate::split::line_mark::LineMark;
use crate::split::solver::SplitSolver;
use crate::util::sort::SmartBinSort;


pub(crate) trait IsoSplitSegments<C: WindingCount> {
    fn iso_split_segments(self, solver: Solver) -> Vec<Segment<C>>;
}

impl<C: WindingCount> IsoSplitSegments<C> for Vec<Segment<C>> {
    #[inline]
    fn iso_split_segments(self, solver: Solver) -> Vec<Segment<C>> {
        SplitSolver::new(solver).iso_split(self)
    }
}

impl SplitSolver {
    pub(super) fn iso_split<C: WindingCount>(&self, segments: Vec<Segment<C>>) -> Vec<Segment<C>> {
        let layout = if let Some(layout) = GridLayout::new(segments.iter().map(|it| it.x_segment), segments.len()) {
            layout
        } else {
            panic!("not valid geometry");
        };

        let mut buffer = IsoFragmentBuffer::new(layout);
        let hz_count = buffer.init_fragment_buffer(segments.iter().map(|it| it.x_segment));
        let mut hz_lines = Vec::with_capacity(hz_count);

        for (i, segment) in segments.iter().enumerate() {
            if segment.x_segment.is_horizontal() {
                hz_lines.push(HzLine::new(i, segment.x_segment));
                continue;
            }
            buffer.add_segment(i, segment.x_segment);
        }

        let mut marks = self.iso_process(&mut buffer);

        if !buffer.on_border.is_empty() {
            for (&index, vr_lines) in buffer.on_border.iter_mut() {
                if let Some(group) = buffer.groups.get(index) {
                    let border_x = buffer.layout.pos(index);
                    Self::iso_on_border_split(border_x, &group.dg_lines, vr_lines, &mut marks);
                }
            }
        }

        hz_lines.smart_bin_sort_by(&self.solver, |h0, h1| h0.min_x.cmp(&h1.min_x));
        self.iso_hz_split(&hz_lines, &buffer, &mut marks);

        if marks.is_empty() {
            return segments;
        }

        self.apply(&mut marks, segments, false)
    }

    #[inline]
    fn iso_on_border_split(border_x: i32, dg_lines: &[DgLine], vr_lines: &mut [VrLine], marks: &mut Vec<LineMark>) {
        let mut yy = Vec::new();
        for dg_line in dg_lines.iter() {
            if dg_line.full.max_x == border_x {
                yy.push(dg_line.y_for_x(dg_line.full.max_x))
            }
        }

        if yy.is_empty() {
            return;
        }

        yy.sort_unstable();
        vr_lines.sort_unstable_by(|s0, s1| s0.min_y.cmp(&s1.min_y));

        let mut i = 0;
        for vr in vr_lines.iter() {
            // scroll by y to first overlap
            while i < yy.len() && yy[i] <= vr.min_y {
                i += 1;
            }
            let mut j = i;
            while j < yy.len() && yy[j] < vr.max_y {
                marks.push(LineMark { index: vr.index, point: IntPoint::new(border_x, yy[j]) });
                j += 1;
            }
        }
    }

    #[inline]
    fn iso_hz_split(&self, hz_lines: &[HzLine], buffer: &IsoFragmentBuffer, marks: &mut Vec<LineMark>) {
        let mut scan_list = Vec::with_capacity(hz_lines.len() / buffer.vr_lines.len());

        let mut i = 0;
        let mut min_x = buffer.layout.min_x;
        for (buffer_index, group) in buffer.groups.iter().enumerate() {
            let max_x = buffer.layout.pos(buffer_index + 1);
            if group.is_empty() {
                min_x = max_x;
                continue
            }

            while i < hz_lines.len() {
                let hz = &hz_lines[i];
                if hz.min_x > max_x {
                    break;
                }
                scan_list.push(hz.clone());
                i += 1;
            }

            let mut j = 0;
            while j < scan_list.len() {
                let hz = &scan_list[j];
                if hz.max_x < min_x {
                    scan_list.swap_remove(j);
                    continue
                }

                if !group.dg_lines.is_empty() {
                    let mut k = group.dg_lines
                        .binary_search_by(|dg| dg.rect.min_y.cmp(&hz.y))
                        .unwrap_or_else(|index| index);

                    while k < group.dg_lines.len() && hz.y <= group.dg_lines[k].rect.min_y {
                        let dg = &group.dg_lines[k];
                        if hz.max_x < dg.rect.min_x || dg.rect.max_x < hz.min_x {
                            continue
                        }
                        Self::cross_hz_dg(hz, dg, marks);
                        k += 1;
                    }
                }

                if !group.vr_lines.is_empty() {
                    let mut k = group.vr_lines
                        .binary_search_by(|dg| dg.min_y.cmp(&hz.y))
                        .unwrap_or_else(|index| index);

                    while k < group.vr_lines.len() && hz.y <= group.vr_lines[k].min_y {
                        let vr = &group.vr_lines[k];
                        if hz.max_x < vr.x || vr.x < hz.min_x {
                            continue
                        }
                        Self::cross_vr_hz(vr, hz, marks);
                        k += 1;
                    }
                }

                j += 1;
            }

            min_x = max_x;
        }
    }

    #[inline]
    fn iso_process(&self, buffer: &mut IsoFragmentBuffer) -> Vec<LineMark> {
        #[cfg(feature = "allow_multithreading")]
        {
            if self.solver.multithreading.is_some() {
                return Self::iso_parallel_split(buffer);
            }
        }

        Self::iso_serial_split(buffer)
    }

    #[inline]
    fn iso_serial_split(buffer: &mut IsoFragmentBuffer) -> Vec<LineMark> {
        let mut marks = Vec::new();
        let width = 1i32 << buffer.layout.power;
        for group in buffer.groups.iter_mut() {
            if group.dg_lines.is_empty() { continue; }
            SplitSolver::iso_bin_split(width, group, &mut marks);
        }

        marks
    }

    #[cfg(feature = "allow_multithreading")]
    #[inline]
    fn iso_parallel_split(buffer: &mut IsoFragmentBuffer) -> Vec<LineMark> {
        use rayon::iter::IntoParallelRefMutIterator;
        use rayon::iter::ParallelIterator;

        let width = 1i32 << buffer.layout.power;

        buffer
            .groups
            .par_iter_mut()
            .flat_map(|group| {
                let mut marks = Vec::new();
                SplitSolver::iso_bin_split(width, group, &mut marks);
                marks
            })
            .collect()
    }

    #[inline]
    fn iso_bin_split(width: i32, group: &mut Group, marks: &mut Vec<LineMark>) {
        group.dg_lines.sort_unstable_by(|a, b| a.rect.min_y.cmp(&b.rect.min_y));

        for (i, di) in group.dg_lines.iter().enumerate().take(group.dg_lines.len() - 1) {
            for dj in group.dg_lines.iter().skip(i + 1) {
                if di.rect.max_y < dj.rect.min_y {
                    break;
                }
                if di.k == dj.k || !di.rect.is_intersect_border_include(&dj.rect) {
                    continue;
                }

                SplitSolver::cross_dg_dg(di, dj, marks)
            }
        }

        group.vr_lines.sort_unstable_by(|a, b| a.min_y.cmp(&b.min_y));
        let mut i = 0;
        for vr in group.vr_lines.iter() {
            // scroll by y to first overlap
            let min_y = vr.min_y.saturating_sub(width);
            while i < group.dg_lines.len() && group.dg_lines[i].rect.min_y <= min_y {
                i += 1;
            }
            let mut j = i;
            while j < group.dg_lines.len() && group.dg_lines[i].rect.min_y < vr.max_y {
                SplitSolver::cross_vr_dg(vr, &group.dg_lines[i], marks);
                j += 1;
            }
        }
    }

    #[inline]
    fn cross_vr_hz(vr: &VrLine, hz: &HzLine, marks: &mut Vec<LineMark>) {
        let x = vr.x;
        let y = hz.y;

        let point = IntPoint { x, y };

        if vr.min_y < y && y < vr.max_y {
            marks.push(LineMark { index: vr.index, point })
        }

        if hz.min_x < x && x < hz.max_x {
            marks.push(LineMark { index: hz.index, point })
        }
    }

    #[inline]
    fn cross_vr_dg(vr: &VrLine, dg: &DgLine, marks: &mut Vec<LineMark>) {
        let x = vr.x;
        let y = dg.y_for_x(x);

        let point = IntPoint { x, y };

        if y < vr.min_y || vr.max_y < y {
            return
        }

        if vr.min_y < y && y < vr.max_y {
            marks.push(LineMark { index: vr.index, point });
        }

        if dg.contains_exclude_border(point) {
            marks.push(LineMark { index: dg.index, point });
        }
    }

    #[inline]
    fn cross_hz_dg(hz: &HzLine, dg: &DgLine, marks: &mut Vec<LineMark>) {
        let y = hz.y;
        let x = dg.x_for_y(y);

        let point = IntPoint { x, y };

        if x < hz.min_x || hz.max_x < x {
            return
        }

        if hz.min_x < x && x < hz.max_x {
            marks.push(LineMark { index: hz.index, point });
        }

        if dg.contains_exclude_border(point) {
            marks.push(LineMark { index: dg.index, point });
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
            marks.push(LineMark { index: d0.index, point });
        }

        if d1.contains_exclude_border(point) {
            marks.push(LineMark { index: d1.index, point });
        }
    }
}

impl HzLine {
    #[inline(always)]
    fn new(index: usize, x_segment: XSegment) -> Self {
        HzLine {
            index,
            y: x_segment.a.y,
            min_x: x_segment.a.x,
            max_x: x_segment.b.x,
        }
    }
}

impl DgLine {

    #[inline]
    pub(super) fn y_for_x(&self, x: i32) -> i32{
        (self.k * x).wrapping_add(self.b)
    }

    #[inline]
    fn x_for_y(&self, y: i32) -> i32{
        y.wrapping_sub(self.b) * self.k
    }

    #[inline]
    fn contains_exclude_border(&self, point: IntPoint) -> bool {
        let xx = self.full.min_x < point.x && point.x < self.full.max_x;
        let yy = self.full.min_y < point.y && point.y < self.full.max_y;
        xx && yy
    }
}

impl BinKey<i32> for HzLine {
    #[inline(always)]
    fn bin_key(&self) -> i32 {
        self.min_x
    }

    #[inline(always)]
    fn bin_index(&self, layout: &BinLayout<i32>) -> usize {
        layout.index(self.min_x)
    }
}