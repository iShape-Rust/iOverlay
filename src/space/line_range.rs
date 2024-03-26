#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineRange {
    pub min: i32,
    pub max: i32
}

impl LineRange {
    pub fn is_overlap(self, other: LineRange) -> bool {
        self.min <= other.max && self.max >= other.min
    }

    pub fn clamp(&self, range: LineRange) -> LineRange {
        let min = self.min.clamp(range.min, range.max);
        let max = self.max.clamp(range.min, range.max);
        Self { min, max }
    }
}