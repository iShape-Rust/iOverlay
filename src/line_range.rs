#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineRange {
    pub min: i32,
    pub max: i32
}

impl LineRange {

    #[inline(always)]
    pub fn width(&self) -> i64 {
        self.max as i64 - self.min as i64
    }

    #[inline]
    pub fn add_value(&mut self, value: i32) {
        if self.min > value {
            self.min = value
        } else if self.max < value {
            self.max = value
        }
    }
}