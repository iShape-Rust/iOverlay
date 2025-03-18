use std::collections::HashMap;
use i_float::int::rect::IntRect;
use crate::bind::segment::IdSegment;
use crate::geom::line_range::LineRange;
use crate::geom::x_segment::XSegment;
use crate::split::grid_layout::GridLayout;

#[derive(Debug, Clone)]
pub(super) struct Fragment {
    pub(super) index: usize,
    pub(super) rect: IntRect,
    pub(super) x_segment: XSegment,
}

impl Fragment {

    #[inline]
    pub(super) fn with_index_and_segment(index: usize, x_segment: XSegment) -> Self {
        let (min_y, max_y) = if x_segment.a.y < x_segment.b.y {
            (x_segment.a.y, x_segment.b.y)
        } else {
            (x_segment.b.y, x_segment.a.y)
        };

        let rect = IntRect {
            min_x: x_segment.a.x,
            max_x: x_segment.b.x,
            min_y,
            max_y,
        };

        Self {
            index,
            rect,
            x_segment,
        }
    }

    #[inline(always)]
    pub(super) fn y_range(&self) -> LineRange {
        LineRange { min: self.rect.min_y, max: self.rect.max_y }
    }
}


pub(super) struct FragmentBuffer {
    pub(super) layout: GridLayout,
    pub(super) groups: Vec<Vec<Fragment>>,
    pub(super) on_border: HashMap<usize, Vec<IdSegment>>,
}

impl FragmentBuffer {
    #[inline]
    pub(super) fn new(layout: GridLayout) -> Self {
        let n = layout.index(layout.max_x) + 1;
        Self { layout, groups: vec![Vec::new(); n], on_border: HashMap::new() }
    }

    pub(super) fn init_fragment_buffer<I>(&mut self, iter: I)
    where
        I: Iterator<Item=XSegment>,
    {
        let mut counts = vec![0; self.groups.len()];
        for s in iter {
            let i0 = self.layout.index(s.a.x);
            if s.a.x < s.b.x {
                let i1 = self.layout.index(s.b.x - 1);
                for count in counts.iter_mut().take(i1).skip(i0) {
                    *count += 1;
                }
            } else {
                counts[i0] += 1;
            }
        }

        for (i, group) in self.groups.iter_mut().enumerate() {
            group.reserve(counts[i]);
        }
    }

    #[inline]
    fn insert(&mut self, fragment: Fragment, bin_index: usize) {
        unsafe { self.groups.get_unchecked_mut(bin_index) }.push(fragment);
    }

    #[inline]
    fn insert_vertical(&mut self, fragment: Fragment, bin_index: usize) {
        if bin_index > 0 && fragment.x_segment.a.x == self.layout.pos(bin_index) {
            let id_segment = IdSegment { id: fragment.index, x_segment: fragment.x_segment };
            if let Some(segments) = self.on_border.get_mut(&bin_index) {
                segments.push(id_segment);
            } else {
                self.on_border.insert(bin_index, vec![id_segment]);
            }
        }
        self.insert(fragment, bin_index);
    }

    #[inline]
    pub(super) fn clear(&mut self) {
        for group in self.groups.iter_mut() {
            group.clear();
        }
        self.on_border.clear();
    }

    #[inline]
    pub(super) fn add_segment(&mut self, segment_index: usize, s: XSegment) {
        if s.a.y == s.b.y {
            self.add_horizontal(segment_index, s);
            return;
        }

        let i0 = self.layout.index(s.a.x);
        if s.a.x == s.b.x {
            self.insert_vertical(Fragment::with_index_and_segment(segment_index, s), i0);
            return;
        }

        let i1 = self.layout.index(s.b.x - 1);
        if i0 >= i1 {
            self.insert(Fragment::with_index_and_segment(segment_index, s), i0);
            return;
        }

        let x0 = s.a.x;
        let y0 = s.a.y;

        let mut prev_x = x0;
        let mut prev_y = y0;

        let is_inc = s.a.y <= s.b.y;

        let width = (s.b.x - s.a.x) as u64;
        let height = (s.b.y - s.a.y).unsigned_abs() as u64;

        let log = (width * height).ilog2();
        let p = 63 - log;
        let k = (height << p) / width;

        let mut w = (self.layout.pos(i0 + 1) - s.a.x) as u64;
        let dw = 1 << self.layout.power;

        for i in i0..i1 {
            let h_min = (w * k) >> p;
            let mut h_max = h_min;
            while h_max * width < height * w {
                h_max += 1;
            }

            let max_x = x0 + (w as i32);

            let rect = if is_inc {
                let max_y = y0 + h_max as i32;
                let rect = IntRect { min_x: prev_x, max_x, min_y: prev_y, max_y };
                prev_y = y0 + h_min as i32;
                rect
            } else {
                let min_y = y0 - h_max as i32;
                let rect = IntRect { min_x: prev_x, max_x, min_y, max_y: prev_y };
                prev_y = y0 - h_min as i32;
                rect
            };

            prev_x = max_x;
            w += dw;

            self.insert(Fragment { index: segment_index, rect, x_segment: s }, i);
        }

        let rect = if is_inc {
            IntRect { min_x: prev_x, max_x: s.b.x, min_y: prev_y, max_y: s.b.y }
        } else {
            IntRect { min_x: prev_x, max_x: s.b.x, min_y: s.b.y, max_y: prev_y }
        };

        self.insert(Fragment { index: segment_index, rect, x_segment: s }, i1);
    }

    fn add_horizontal(&mut self, segment_index: usize, s: XSegment) {
        let i0 = self.layout.index(s.a.x);
        let i1 = self.layout.index(s.b.x - 1);

        let y = s.a.y;
        let mut x0 = s.a.x;
        let mut i = i0;
        while i < i1 {
            let x = self.layout.pos(i + 1);
            let rect = IntRect { min_x: x0, max_x: x, min_y: y, max_y: y };
            self.insert(Fragment { index: segment_index, rect, x_segment: s }, i);

            x0 = x;
            i += 1;
        }

        let rect = IntRect { min_x: x0, max_x: s.b.x, min_y: y, max_y: y };
        self.insert(Fragment { index: segment_index, rect, x_segment: s }, i1);
    }
}