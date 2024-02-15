use crate::space::line_range::LineRange;
use crate::space::line_space::IntExtensions;

#[derive(Debug)]
pub struct LineIndexer {
    pub scale: usize,
    pub size: usize,
    pub range: LineRange,
    pub max_level: usize,
    offset: i32
}

impl LineIndexer {

    pub fn new(level: usize, range: LineRange) -> Self {
        let x_min = range.min;
        let x_max = range.max;
        let dif = (x_max - x_min) as usize;

        let d_log = dif.log_two();

        let max_level = if dif <= 2 {
            0
        } else {
            // 10.min(level.min(d_log - 1))
            level.min((d_log - 1).min(10))
        };

        let offset = -range.min;
        let scale = d_log - max_level;

        let size = Self::space_count(max_level);

        Self { scale, size, range, max_level, offset }
    }

    pub fn unsafe_index(&self, range: LineRange) -> usize {
        if self.max_level < 1 {
            return 0;
        }
        assert!(range.min >= self.range.min);
        assert!(range.max <= self.range.max);

        // scale to indexer coordinate system
        let mut i_min = (range.min + self.offset) as usize;
        let mut i_max = (range.max + self.offset) as usize;

        let dif = (i_max - i_min) >> (self.scale - 1);
        let d_log = if dif != 0 { dif.log_two() } else { 0 };

        let level = if d_log < self.max_level {
            self.max_level - d_log
        } else {
            0
        };

        let s = self.scale + d_log;

        i_min = i_min >> s;
        i_max = i_max >> s;

        let i_dif = i_max - i_min;
        let index = Self::custom_space_count(level + i_dif, level) + i_min;

        index
    }

    pub fn index(&self, range: LineRange) -> usize {
        let clamp_range = LineRange {
            min: self.range.min.max(range.min),
            max: self.range.max.min(range.max)
        };

        self.unsafe_index(clamp_range)
    }

    pub fn fill(&self, range: LineRange, buffer: &mut Vec<usize>) {
        let clamp = range.clamp(self.range);
        self.fill_unsafe(clamp, buffer);
    }

    pub fn fill_unsafe(&self, range: LineRange, buffer: &mut Vec<usize>) {
        if self.max_level < 1 {
            buffer.push(0);
            return;
        }

        let x0 = (range.min + self.offset) as usize;
        let x1 = (range.max + self.offset) as usize;

        let mut x_left = x0 >> self.scale;
        let mut x_right = x1 >> self.scale;

        for n in 1..=self.max_level {

            let level = self.max_level - n;
            let index_offset = Self::space_count(level);

            for x in x_left..=x_right {
                let index = index_offset + x;
                assert!(index > 0);
                buffer.push(index);
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
                assert!(index > 0);
                buffer.push(index);
            }
        }

        buffer.push(0);
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

    // Test purpose only, must be same logic as in iterateAllInRange
    pub fn heap_indices(&self, range: LineRange) -> Vec<usize> {
        let mut result = Vec::new();
        self.fill_unsafe(range, &mut result);
        result
    }
}