use crate::core::solver::Solver;

pub(super) struct SnapRadius {
    current: usize,
    step: usize,
}

impl SnapRadius {
    pub(super) fn increment(&mut self) {
        self.current = 60.min(self.current + self.step);
    }

    pub(super) fn radius(&self) -> i64 {
        1 << self.current
    }
}

impl Solver {
    pub(super) fn snap_radius(&self) -> SnapRadius {
        SnapRadius {
            current: self.precision.start,
            step: self.precision.progression,
        }
    }
}