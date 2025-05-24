use i_shape::int::path::ContourExtension;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_shape::int::shape::IntContour;
use i_shape::util::reserve::Reserve;

pub(super) trait Split {
    fn split_loops(self, min_area: u64, contour_buffer: &mut IntContour, bin_store: &mut BinStore) -> Vec<Self>
    where
        Self: Sized;
}

impl Split for IntContour {
    fn split_loops(self, min_area: u64, contour_buffer: &mut IntContour, bin_store: &mut BinStore) -> Vec<Self> {
        if self.is_empty() {
            return Vec::new();
        }
        contour_buffer.reserve_capacity(self.len());
        contour_buffer.clear();

        bin_store.init(&self);

        let mut result: Vec<IntContour> = Vec::with_capacity(16);

        for point in self {
            let next_pos = contour_buffer.len() + 1;
            let pos = bin_store.insert_if_not_exist(point, next_pos);
            if pos < contour_buffer.len() {
                // found a loop
                let tail_len = contour_buffer.len() - pos;
                if tail_len < 2 {
                    // tail is too small
                    contour_buffer.truncate(pos);
                } else {
                    let mut tail = contour_buffer.split_off(pos);
                    tail.push(point);
                    if tail.validate_area(min_area) {
                        result.push(tail);
                    }
                }
            } else {
                contour_buffer.push(point);
            }
        }

        if contour_buffer.len() > 2 {
            result.push(contour_buffer.as_slice().to_vec());
        }

        result
    }
}

#[derive(Debug, Clone, Copy)]
struct PointItem {
    point: IntPoint,
    pos: usize,
}

#[derive(Debug, Clone, Copy)]
struct Bin {
    offset: usize,
    data: usize,
}

pub(super) struct BinStore {
    mask: u32,
    bins: Vec<Bin>,
    items: Vec<PointItem>,
}

impl BinStore {
    pub(super) fn new() -> Self {
        Self {
            mask: 0,
            bins: Vec::new(),
            items: Vec::new(),
        }
    }

    fn init(&mut self, contour: &IntContour) {
        let log = contour.len().ilog2().saturating_sub(4).clamp(1, 30);
        let bins_count = (1 << log) as usize;

        self.bins.clear();
        self.bins.resize(bins_count, Bin { offset: 0, data: 0 });

        self.items.clear();
        self.items.resize(contour.len(), PointItem { point: IntPoint::EMPTY, pos: 0 });

        self.mask = bins_count.wrapping_sub(1) as u32;

        for &p in contour.iter() {
            let index = self.bin_index(p);
            unsafe { self.bins.get_unchecked_mut(index).data += 1 };
        }

        let mut offset = 0;
        for bin in self.bins.iter_mut() {
            let next_offset = offset + bin.data;
            *bin = Bin {
                offset,
                data: offset,
            };
            offset = next_offset;
        }
    }

    #[inline]
    fn insert_if_not_exist(&mut self, point: IntPoint, pos: usize) -> usize {
        let index = self.bin_index(point);
        let bin = unsafe { self.bins.get_unchecked_mut(index) };
        let start = bin.offset;
        let end = bin.data;
        for i in start..end {
            let item = unsafe { self.items.get_unchecked_mut(i) };
            if item.point == point {
                return item.pos;
            }
        }
        bin.data = end + 1;
        unsafe { *self.items.get_unchecked_mut(end) = PointItem { point, pos } }

        usize::MAX
    }

    #[inline]
    fn bin_index(&self, p: IntPoint) -> usize {
        let x = p.x.unsigned_abs();
        let y = p.y.unsigned_abs();
        let hash = x.wrapping_mul(31) ^ y.wrapping_mul(17);
        (hash & self.mask) as usize
    }
}

trait ValidateArea {
    fn validate_area(&self, min_area: u64) -> bool;
}

impl ValidateArea for IntContour {
    #[inline]
    fn validate_area(&self, min_area: u64) -> bool {
        if min_area == 0 {
            return true;
        }
        let abs_area = self.unsafe_area().unsigned_abs() >> 1;
        abs_area < min_area
    }
}

#[cfg(test)]
mod tests {
    use crate::i_shape::int::path::IntPath;
    use super::*;
    use alloc::vec;

    #[test]
    fn test_empty_path() {
        let path: IntPath = vec![];
        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert_eq!(result, vec![] as Vec<IntPath>);
    }

    #[test]
    fn test_single_point() {
        let path = vec![IntPoint::new(0, 0)];
        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert!(result.is_empty());
    }

    #[test]
    fn test_two_points() {
        let path = vec![IntPoint::new(0, 0), IntPoint::new(1, 1)];
        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert!(result.is_empty());
    }

    #[test]
    fn test_no_repeated_points() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(0, 1),
            IntPoint::new(1, 1),
            IntPoint::new(1, 0),
        ];

        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.clone().split_loops(0, &mut contour, &mut bin_store);
        assert_eq!(result, vec![path]);
    }

    #[test]
    fn test_2_loops_0() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0), // same point
            IntPoint::new(1, -1),
        ];

        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            [
                IntPoint::new(3, 1),
                IntPoint::new(4, 0),
                IntPoint::new(3, -1),
                IntPoint::new(2, 0),
            ]
            .to_vec()
        );
        assert_eq!(
            result[1],
            [
                IntPoint::new(0, 0),
                IntPoint::new(1, 1),
                IntPoint::new(2, 0),
                IntPoint::new(1, -1),
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_2_loops_1() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0), // same point
            IntPoint::new(1, -1),
        ];

        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            [
                IntPoint::new(3, 1),
                IntPoint::new(3, -1),
                IntPoint::new(2, 0),
            ]
            .to_vec()
        );
        assert_eq!(
            result[1],
            [
                IntPoint::new(0, 0),
                IntPoint::new(1, 1),
                IntPoint::new(2, 0),
                IntPoint::new(1, -1),
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_2_loops_with_tails() {
        let path = vec![
            IntPoint::new(-1, 0),
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(5, 0),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0), // same point
            IntPoint::new(1, -1),
            IntPoint::new(0, 0),
        ];

        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            [
                IntPoint::new(3, 1),
                IntPoint::new(4, 0),
                IntPoint::new(3, -1),
                IntPoint::new(2, 0),
            ]
            .to_vec()
        );
        assert_eq!(
            result[1],
            [
                IntPoint::new(1, 1),
                IntPoint::new(2, 0),
                IntPoint::new(1, -1),
                IntPoint::new(0, 0),
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_single_loop() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(0, 0), // same point, forms a loop
        ];

        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            [
                IntPoint::new(1, 1),
                IntPoint::new(2, 0),
                IntPoint::new(0, 0),
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_cross() {
        let path = vec![
            IntPoint::new(-2, -1),
            IntPoint::new(-2, 1),
            IntPoint::new(0, 0),
            IntPoint::new(-1, 2),
            IntPoint::new(1, 2),
            IntPoint::new(0, 0), // same point, forms a loop
            IntPoint::new(2, 1),
            IntPoint::new(2, -1),
            IntPoint::new(0, 0), // same point, forms a loop
            IntPoint::new(1, -2),
            IntPoint::new(-1, -2),
            IntPoint::new(0, 0), // same point, forms a loop
        ];

        let mut contour: IntContour = Vec::new();
        let mut bin_store = BinStore::new();
        let result = path.split_loops(0, &mut contour, &mut bin_store);
        assert_eq!(result.len(), 4);
        assert_eq!(
            result[0],
            [
                IntPoint::new(-1, 2),
                IntPoint::new(1, 2),
                IntPoint::new(0, 0),
            ]
            .to_vec()
        );
        assert_eq!(
            result[1],
            [
                IntPoint::new(2, 1),
                IntPoint::new(2, -1),
                IntPoint::new(0, 0),
            ]
            .to_vec()
        );
        assert_eq!(
            result[2],
            [
                IntPoint::new(1, -2),
                IntPoint::new(-1, -2),
                IntPoint::new(0, 0),
            ]
            .to_vec()
        );
        assert_eq!(
            result[3],
            [
                IntPoint::new(-2, -1),
                IntPoint::new(-2, 1),
                IntPoint::new(0, 0),
            ]
            .to_vec()
        );
    }
}
