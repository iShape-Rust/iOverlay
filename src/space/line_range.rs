#[derive(Debug, Clone, Copy)]
pub struct LineRange {
    pub min: i32,
    pub max: i32
}

impl LineRange {
    pub fn is_overlap(self, other: LineRange) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}