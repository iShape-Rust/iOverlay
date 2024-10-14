#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LineRange {
    pub(crate) min: i32,
    pub(crate) max: i32,
}

impl LineRange {
    #[inline(always)]
    pub(crate) fn width(&self) -> i64 {
        self.max as i64 - self.min as i64
    }
}