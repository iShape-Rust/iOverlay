use std::collections::HashMap;
use i_float::int::rect::IntRect;
use crate::geom::x_segment::XSegment;
use crate::split::grid_layout::GridLayout;

#[derive(Debug, Clone)]
pub(super) struct DgLine {
    pub(super) index: usize,
    pub(super) rect: IntRect,
    pub(super) full: IntRect,
    pub(super) k: i32,
    pub(super) b: i32,
}

#[derive(Debug, Clone)]
pub(super) struct VrLine {
    pub(super) index: usize,
    pub(super) x: i32,
    pub(super) min_y: i32,
    pub(super) max_y: i32,
}

#[derive(Debug, Clone)]
pub(super) struct HzLine {
    pub(super) index: usize,
    pub(super) y: i32,
    pub(super) min_x: i32,
    pub(super) max_x: i32,
}

#[derive(Debug, Clone)]
pub(super) struct Group {
    pub(super) dg_lines: Vec<DgLine>,
    pub(super) vr_lines: Vec<VrLine>,
}

pub(super) struct IsoFragmentBuffer {
    pub(super) layout: GridLayout,
    pub(super) groups: Vec<Group>,
    pub(super) on_border: HashMap<usize, Vec<VrLine>>,
}

impl Group {
    fn new() -> Self {
        Group { dg_lines: vec![], vr_lines: vec![] }
    }

    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        self.vr_lines.is_empty() && self.dg_lines.is_empty()
    }
}

impl IsoFragmentBuffer {
    #[inline]
    pub(super) fn new(layout: GridLayout) -> Self {
        let n = layout.index(layout.max_x) + 1;
        Self { layout, groups: vec![Group::new(); n], on_border: HashMap::new() }
    }

    pub(super) fn init_fragment_buffer<I>(&mut self, iter: I) -> usize
    where
        I: Iterator<Item=XSegment>,
    {
        let mut hz_count = 0;
        let mut vr_counts = vec![0; self.groups.len()];
        let mut dg_counts = vec![0; self.groups.len()];
        for s in iter {
            if s.is_horizontal() {
                hz_count += 1;
                continue;
            }
            let i0 = self.layout.index(s.a.x);
            if s.a.x < s.b.x {
                let i1 = self.layout.index(s.b.x - 1);
                for count in dg_counts.iter_mut().take(i1).skip(i0) {
                    *count += 1;
                }
            } else {
                vr_counts[i0] += 1;
            }
        }

        for (i, group) in self.groups.iter_mut().enumerate() {
            group.vr_lines.reserve(vr_counts[i]);
            group.dg_lines.reserve(dg_counts[i]);
        }

        hz_count
    }

    #[inline]
    fn insert(&mut self, dg_line: DgLine, bin_index: usize) {
        unsafe { self.groups.get_unchecked_mut(bin_index) }.dg_lines.push(dg_line);
    }

    #[inline]
    fn insert_vertical(&mut self, line: VrLine, bin_index: usize) {
        if bin_index > 0 && line.x == self.layout.pos(bin_index) {
            if let Some(lines) = self.on_border.get_mut(&bin_index) {
                lines.push(line.clone());
            } else {
                self.on_border.insert(bin_index, vec![line.clone()]);
            }
        }
        unsafe { &mut self.groups.get_unchecked_mut(bin_index).vr_lines }.push(line);
    }

    #[inline]
    pub(super) fn add_segment(&mut self, segment_index: usize, s: XSegment) {
        let i0 = self.layout.index(s.a.x);
        if s.a.x == s.b.x {
            self.insert_vertical(VrLine::new(segment_index, &s), i0);
            return;
        }

        let i1 = self.layout.index(s.b.x - 1);
        if i0 >= i1 {
            self.insert(DgLine::new(segment_index, s), i0);
            return;
        }

        let y0 = s.a.y;
        let mut x0 = s.a.x;
        let x1 = self.layout.pos(i0 + 1);
        let dx = 1 << self.layout.power;

        let (dy, k, b, mut min_y, mut max_y) = if s.a.y < s.b.y {
            let min_y = y0;
            let max_y = min_y + x1 - x0;
            let b = y0.wrapping_sub(x0);
            (dx, 1, b, min_y, max_y)
        } else {
            let max_y = y0;
            let min_y = max_y - (x1 - x0);
            let b = y0.wrapping_add(x0);
            (-dx, -1, b, min_y, max_y)
        };

        let rect = IntRect { min_x: x0, max_x: x1, min_y, max_y };
        let full = s.rect();

        self.insert(DgLine { index: segment_index, rect, full: full.clone(), k, b }, i0);

        let mut i = i0 + 1;
        x0 = x1;
        while i < i1 {
            let xi = x0 + dx;
            min_y += dy;
            max_y += dy;

            let rect = IntRect { min_x: x0, max_x: xi, min_y, max_y };
            self.insert(DgLine { index: segment_index, rect, full: full.clone(), k, b }, i);

            x0 = xi;
            i += 1
        }

        let rect = if s.a.y < s.b.y {
            IntRect { min_x: x0, max_x: s.b.x, min_y: max_y, max_y: s.b.y }
        } else {
            IntRect { min_x: x0, max_x: s.b.x, min_y: s.b.y, max_y: min_y }
        };

        self.insert(DgLine { index: segment_index, rect, full: full.clone(), k, b }, i1);
    }

}

impl DgLine {

    #[inline]
    fn new(index: usize, x_segment: XSegment) -> Self {
        let (min_y, max_y,k , b) = if x_segment.a.y < x_segment.b.y {
            let b = x_segment.a.y.wrapping_sub(x_segment.a.x);
            (x_segment.a.y, x_segment.b.y, 1, b)
        } else {
            let b = x_segment.a.y.wrapping_add(x_segment.a.x);
            (x_segment.b.y, x_segment.a.y, -1, b)
        };

        let rect = IntRect {
            min_x: x_segment.a.x,
            max_x: x_segment.b.x,
            min_y,
            max_y,
        };

        Self { index, full: rect.clone(), rect, k, b }
    }
}

impl VrLine {
    #[inline(always)]
    pub(crate) fn new(index: usize, x_segment: &XSegment) -> Self {
        let (min_y, max_y) = if x_segment.a.y < x_segment.b.y {
            (x_segment.a.y, x_segment.b.y)
        } else {
            (x_segment.b.y, x_segment.a.y)
        };

        VrLine { index, x: x_segment.a.x, min_y, max_y}
    }
}

impl XSegment {
    #[inline(always)]
    fn rect(&self) -> IntRect {
        let (min_y, max_y) = if self.a.y < self.b.y {
            (self.a.y, self.b.y)
        } else {
            (self.b.y, self.a.y)
        };

        IntRect { min_x: self.a.x, max_x: self.b.x, min_y, max_y }
    }
}