use i_float::rect::IntRect;
use crate::split::fragment::Fragment;
use crate::split::shape_edge::ShapeEdge;
use crate::x_segment::XSegment;

pub(crate) struct SpaceLayout {
    pub(super) power: usize,
    min_size: u64,
    scale: usize
}

impl SpaceLayout {
    const MIN_POWER: usize = 2;
    const MAX_POWER: usize = 12;
    pub(super) const MIN_HEIGHT: usize = 1 << Self::MIN_POWER;

    pub(super) fn new(height: usize, count: usize) -> Self {
        let max_power_range = height.log2() - 1;
        let max_power_count = count.log2() >> 1;
        let original_power = max_power_range.min(max_power_count);
        let power = original_power.clamp(Self::MIN_POWER, Self::MAX_POWER);
        let min_size = (height >> power) as u64;
        let m = (min_size as usize).log2();
        let scale = u32::BITS as usize - m;
        Self { power, min_size, scale }
    }

    pub(super) fn break_into_fragments(&self, index: usize, x_segment: &XSegment, buffer: &mut Vec<Fragment>) {
        let min_x = x_segment.a.x;
        let max_x = x_segment.b.x;

        let is_up = x_segment.a.y < x_segment.b.y;

        let (min_y, max_y) = if is_up {
            (x_segment.a.y, x_segment.b.y)
        } else {
            (x_segment.b.y, x_segment.a.y)
        };

        let dx = (max_x as i64 - min_x as i64) as u64;
        let dy = (max_y as i64 - min_y as i64) as u64;

        let is_fragmentation_required = dx > self.min_size && dy > self.min_size;

        if !is_fragmentation_required {
            buffer.push(Fragment::with_index_and_segment(index, x_segment.clone()));
            return;
        }

        let k = (dy << self.scale) / dx;

        let s = if dx < dy {
            self.min_size << self.scale
        } else {
            (self.min_size << self.scale) * dx / dy
        };

        let mut x0: u64 = 0;

        let mut ix0 = min_x;
        let mut iy0 = if is_up { min_y } else { max_y };

        let x_last = (dx << self.scale) - s;

        if x0 >= x_last {
            // must be at least two fragments
            return;
        }

        while x0 < x_last {
            let x1 = x0 + s;
            let x = x1 >> self.scale;

            let y1 = x * k;
            let y = y1 >> self.scale;

            let is_same_line = x * dy == y * dx;
            let extra = if is_same_line { 0 } else { 1 };

            let ix1 = min_x + x as i32;


            let (iy1, rect) = if is_up {
                let iy1 = min_y + y as i32;
                let rect = IntRect { min_x: ix0, max_x: ix1, min_y: iy0, max_y: iy1 + extra };
                (iy1, rect)
            } else {
                let iy1 = max_y - y as i32;
                let rect = IntRect { min_x: ix0, max_x: ix1, min_y: iy1 - extra, max_y: iy0 };
                (iy1, rect)
            };

            buffer.push(Fragment { index, rect, x_segment: x_segment.clone() });

            x0 = x1;

            ix0 = ix1;
            iy0 = iy1;
        }


        let rect = if is_up {
            IntRect { min_x: ix0, max_x, min_y: iy0, max_y }
        } else {
            IntRect { min_x: ix0, max_x, min_y, max_y: iy0 }
        };
        buffer.push(Fragment { index, rect, x_segment: x_segment.clone() });
    }

    pub(super) fn is_fragmentation_required_for_edges(&self, edges: &[ShapeEdge]) -> bool {
        let mut i = 0;
        for edge in edges.iter() {
            if self.is_fragmentation_required(edge.x_segment) {
                i += 1;
            }
        }
        // must be at least 20%
        i * 20 > edges.len()
    }

    fn is_fragmentation_required(&self, x_segment: XSegment) -> bool {
        let dx = (x_segment.b.x as i64 - x_segment.a.x as i64) as u64;
        let dy = (x_segment.b.x as i64 - x_segment.a.x as i64).abs() as u64;

        dx > self.min_size && dy > self.min_size
    }
}

trait Log2Extension {
    fn log2(&self) -> usize;
}

impl Log2Extension for usize {
    #[inline(always)]
    fn log2(&self) -> usize {
        debug_assert!(self >= &0);
        let n = self.leading_zeros();
        (usize::BITS - n) as usize
    }
}