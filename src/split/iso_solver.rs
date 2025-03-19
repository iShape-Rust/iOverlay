use crate::core::solver::Solver;
use crate::geom::x_segment::XSegment;
use crate::segm::merge::ShapeSegmentsMerge;
use crate::segm::segment::Segment;
use crate::segm::winding_count::WindingCount;
use crate::split::grid_layout::GridLayout;
use crate::split::iso_fragment::{DgLine, Group, HzLine, IsoFragmentBuffer, VrLine};
use crate::split::line_mark::LineMark;
use crate::split::solver::SplitSolver;
use crate::util::sort::SmartBinSort;
use i_float::int::point::IntPoint;
use i_key_sort::index::{BinKey, BinLayout};

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
        let layout = if let Some(layout) =
            GridLayout::new(segments.iter().map(|it| it.x_segment), segments.len())
        {
            layout
        } else {
            panic!("not valid geometry");
        };

        let mut buffer = IsoFragmentBuffer::new(layout);
        let hz_count = buffer.init_fragment_buffer(segments.iter().map(|it| it.x_segment));
        let mut hz_lines = Vec::with_capacity(hz_count);

        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for (i, segment) in segments.iter().enumerate() {
            if segment.x_segment.is_horizontal() {
                let hz_line = HzLine::new(i, segment.x_segment);
                min_y = min_y.min(hz_line.y);
                max_y = max_y.max(hz_line.y);
                hz_lines.push(hz_line);
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

        if !hz_lines.is_empty() {
            let power: u32 = (buffer.layout.power.saturating_sub(1)).max(1);
            let rows = RowsBank::new(RowLayout::new(min_y, max_y, power));
            self.iso_hz_split(rows, &hz_lines, &buffer, &mut marks);
        }

        if marks.is_empty() {
            return segments;
        }

        self.apply(&mut marks, segments, false)
    }

    #[inline]
    fn iso_on_border_split(
        border_x: i32,
        dg_lines: &[DgLine],
        vr_lines: &mut [VrLine],
        marks: &mut Vec<LineMark>,
    ) {
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
                marks.push(LineMark {
                    index: vr.index,
                    point: IntPoint::new(border_x, yy[j]),
                });
                j += 1;
            }
        }
    }

    #[inline]
    fn iso_hz_split(
        &self,
        mut rows_bank: RowsBank,
        hz_lines: &[HzLine],
        buffer: &IsoFragmentBuffer,
        marks: &mut Vec<LineMark>,
    ) {
        let mut i = 0;
        let mut min_x = buffer.layout.min_x;
        for (group_index, group) in buffer.groups.iter().enumerate() {
            let max_x = buffer.layout.pos(group_index + 1);
            if group.is_empty() {
                min_x = max_x;
                continue;
            }
            rows_bank.remove_all_before(min_x);

            while i < hz_lines.len() {
                let hz = &hz_lines[i];
                if hz.min_x > max_x {
                    break;
                }
                rows_bank.insert(hz.clone());
                i += 1;
            }

            if !group.dg_lines.is_empty() {
                for dg in group.dg_lines.iter() {
                    let min_i = rows_bank.layout.safe_index(dg.rect.min_y);
                    let max_i = rows_bank.layout.safe_index(dg.rect.max_y + 1);
                    for row in &rows_bank.rows[min_i..=max_i] {
                        for hz in &row.lines {
                            if hz.max_x < dg.rect.min_x
                                || dg.rect.max_x < hz.min_x
                                || dg.rect.max_y < hz.y
                                || hz.y < dg.rect.min_y
                            {
                                continue;
                            }
                            Self::cross_hz_dg(hz, dg, marks);
                        }
                    }
                }
            }

            if !group.vr_lines.is_empty() {
                for vr in group.vr_lines.iter() {
                    let min_i = rows_bank.layout.safe_index(vr.min_y);
                    let max_i = rows_bank.layout.safe_index(vr.max_y + 1);
                    for row in &rows_bank.rows[min_i..=max_i] {
                        for hz in &row.lines {
                            if hz.max_x < vr.x
                                || vr.x < hz.min_x
                                || vr.max_y < hz.y
                                || hz.y < vr.min_y
                            {
                                continue;
                            }
                            Self::cross_vr_hz(vr, hz, marks);
                        }
                    }
                }
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
            if group.dg_lines.is_empty() {
                continue;
            }
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
        if group.dg_lines.len() > 1 {
            group
                .dg_lines
                .sort_unstable_by(|a, b| a.rect.min_y.cmp(&b.rect.min_y));

            for (i, di) in group
                .dg_lines
                .iter()
                .enumerate()
                .take(group.dg_lines.len() - 1)
            {
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
        }

        group
            .vr_lines
            .sort_unstable_by(|a, b| a.min_y.cmp(&b.min_y));

        if group.dg_lines.is_empty() {
            return;
        }

        let mut i = 0;
        for vr in group.vr_lines.iter() {
            // scroll by y to first overlap
            let min_y = vr.min_y.saturating_sub(width);
            while i < group.dg_lines.len() && group.dg_lines[i].rect.min_y <= min_y {
                i += 1;
            }

            let mut j = i;
            while j < group.dg_lines.len() && group.dg_lines[j].rect.min_y <= vr.max_y {
                let dg = &group.dg_lines[j];
                j += 1;
                if dg.rect.max_y < vr.min_y || vr.x < dg.rect.min_x || dg.rect.max_x < vr.x {
                    continue;
                }

                SplitSolver::cross_vr_dg(vr, dg, marks);
            }
        }
    }

    #[inline]
    fn cross_vr_hz(vr: &VrLine, hz: &HzLine, marks: &mut Vec<LineMark>) {
        let x = vr.x;
        let y = hz.y;

        let point = IntPoint { x, y };

        if vr.min_y < y && y < vr.max_y {
            marks.push(LineMark {
                index: vr.index,
                point,
            })
        }

        if hz.min_x < x && x < hz.max_x {
            marks.push(LineMark {
                index: hz.index,
                point,
            })
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

#[derive(Clone)]
struct Row {
    min_x: i32,
    lines: Vec<HzLine>,
}

struct RowLayout {
    min_y: i32,
    power: u32,
    max_index: usize,
}

struct RowsBank {
    layout: RowLayout,
    rows: Vec<Row>,
}

impl Row {
    #[inline]
    fn new() -> Self {
        Self {
            min_x: i32::MIN,
            lines: vec![],
        }
    }

    #[inline]
    fn insert(&mut self, line: HzLine) {
        self.min_x = self.min_x.min(line.min_x);
        self.lines.push(line);
    }

    #[inline]
    fn remove_all_before(&mut self, x: i32) {
        if self.lines.is_empty() || self.min_x > x {
            return;
        }
        let mut new_min_x = i32::MAX;
        self.lines.retain(|line| {
            let keep = line.max_x >= x;
            if keep {
                new_min_x = new_min_x.min(line.max_x);
            }
            keep
        });
        self.min_x = new_min_x;
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

impl RowsBank {
    fn new(layout: RowLayout) -> Self {
        let count = layout.max_index + 1;
        let rows = vec![Row::new(); count];
        Self { layout, rows }
    }

    #[inline]
    fn remove_all_before(&mut self, x: i32) {
        for row in self.rows.iter_mut() {
            row.remove_all_before(x);
        }
    }

    #[inline]
    fn insert(&mut self, line: HzLine) {
        let index = self.layout.index(line.y);
        unsafe { self.rows.get_unchecked_mut(index) }.insert(line);
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
