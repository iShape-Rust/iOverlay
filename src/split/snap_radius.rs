pub(super) struct SnapRadius {
    pub(super) current: usize,
    pub(super) step: usize,
}

impl SnapRadius {

    #[inline]
    pub(super) fn increment(&mut self) {
        self.current = 60.min(self.current + self.step);
    }

    #[inline]
    pub(super) fn radius(&self) -> i64 {
        1 << self.current
    }
}