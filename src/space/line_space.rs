use crate::dual_index::DualIndex;
use crate::space::line_range::LineRange;

#[derive(Copy, Clone)]
pub struct LineSegment<Id: Copy> {
    pub id: Id,
    pub range: LineRange
}

#[derive(Copy, Clone)]
pub struct LineContainer<Id: Copy> {
    pub id: Id,
    pub index: DualIndex
}

pub struct LineSpace<Id: Copy> {
    scale: usize,
    max_level: usize,
    offset: isize,
    heap: Vec<Vec<LineSegment<Id>>>,
    heap_buffer: Vec<usize>,
    search_buffer: Vec<LineContainer<Id>>
}

impl<Id: Copy> LineSpace<Id> {

    pub fn scale(&self) -> usize { self.scale }

    pub fn new(n: usize, range: LineRange) -> Self {
        let x_min = range.min as isize;
        let x_max = range.max as isize;
        let dif = (x_max - x_min) as usize;

        let dif_log = dif.log_two();
        let max_level: usize;
        let scale: usize;
        if dif >= 8 {
            max_level = 10.min(n.min(dif_log));
            scale = dif_log - max_level;
        } else {
            max_level = 1;
            scale = 1;
        }

        let offset = -x_min;

        let size = Self::space_count(max_level);
        let heap = vec![vec![]; size];

        LineSpace {
            scale,
            max_level,
            offset,
            heap,
            heap_buffer: Vec::new(),
            search_buffer: Vec::new(),
        }
    }

    pub fn insert(&mut self, segment: LineSegment<Id>) {
        let index = self.heap_index(segment.range);
        self.heap[index].push(segment);
    }

    pub fn clear(&mut self) {
        for i in 0..self.heap.len() {
            self.heap[i].clear();
        }
    }

    fn heap_index(&self, range: LineRange) -> usize {
        // scale to heap coordinate system
        let mut imin = (range.min as isize + self.offset) as usize;
        let mut imax = (range.max as isize + self.offset) as usize;

        let dif = (imax - imin) >> (self.scale - 1);
        let dif_log = dif.log_two();

        let level = if dif_log < self.max_level {
            self.max_level - dif_log
        } else {
            0
        };

        let s = self.scale + dif_log;

        imin = imin >> s;
        imax = imax >> s;

        let i_dif = imax - imin;
        let heap_index = Self::custom_space_count(level + i_dif, level) + imin;

        heap_index
    }

    fn fill_heap_buffer(&mut self, range: LineRange) {
        self.heap_buffer.clear();

        let x0 = (range.min as isize + self.offset) as usize;
        let x1 = (range.max as isize + self.offset) as usize;

        let mut x_left = x0 >> self.scale;
        let mut x_right = x1 >> self.scale;

        for n in 1..=self.max_level {

            let level = self.max_level - n;
            let index_offset = Self::space_count(level);

            for x in x_left..=x_right {
                let index = index_offset + x;
                if !self.heap[index].is_empty() {
                    self.heap_buffer.push(index);
                }
            }

            x_left = x_left >> 1;
            x_right = x_right >> 1;
        }

        let mut s = self.scale - 1;
        for n in 1..=self.max_level {
            let level = self.max_level - n;
            let mut x_max = (level + 2).power_of_two() - 1;
            let mut x_left = x0 >> s;
            let mut x_right = x1 >> s;

            s += 1;

            if x_right <= 0 || x_left >= x_max {
                break;
            }

            x_max = (level + 1).power_of_two() - 2;

            if x_left > 0 {
                x_left = (x_left - 1) >> 1
            }

            if x_right > 0 {
                x_right = (x_right - 1) >> 1
            }

            let index_offset = Self::middle_space_count(level);

            x_left = x_left.min(x_max);
            x_right = x_right.min(x_max);

            for x in x_left..=x_right {
                let index = index_offset + x;
                if !self.heap[index as usize].is_empty() {
                    self.heap_buffer.push(index)
                }
            }
        }

        if !self.heap[0].is_empty() {
            self.heap_buffer.push(0);
        }
    }

    pub fn all_in_range(&mut self, range: LineRange) -> &Vec<LineContainer<Id>> {
        self.fill_heap_buffer(range);
        self.search_buffer.clear();
        for &heap_index in self.heap_buffer.iter() {
            let segments = &self.heap[heap_index];
            for segment_index in 0..segments.len() {
                if range.is_overlap(segments[segment_index].range) {
                    let index = DualIndex { major: heap_index, minor: segment_index };
                    let id = segments[segment_index].id;
                    let container = LineContainer { id, index };
                    self.search_buffer.push(container);
                }
            }
        }

        &self.search_buffer
    }

    pub fn remove(&mut self, index: &DualIndex) {
        if index.minor + 1 < self.heap[index.major].len() {
            self.heap[index.major].swap_remove(index.minor);
        } else {
            _ = self.heap[index.major].pop()
        }
    }

    fn space_count(level: usize) -> usize {
        (level + 2).power_of_two() - level - 3
    }

    fn middle_space_count(level: usize) -> usize {
        (level + 2).power_of_two() + (level + 1).power_of_two() - level - 3
    }

    fn custom_space_count(main_level: usize, second_level: usize) -> usize {
        let main = main_level.power_of_two() - 1;
        let second = second_level.power_of_two() - second_level - 1;
        main + second
    }

}


pub trait IntExtensions {
    fn power_of_two(&self) -> Self;
    fn log_two(&self) -> usize;
}

impl IntExtensions for usize {

    fn power_of_two(&self) -> usize {
        1 << self
    }

    fn log_two(&self) -> usize {
        if *self <= 0 {
            return 0;
        }
        let n = self.leading_zeros();

        (usize::BITS - n) as usize
    }
}