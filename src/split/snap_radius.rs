use crate::split::solver::SplitSolver;

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

impl SplitSolver {
    pub(super) fn snap_radius(&self) -> SnapRadius {
        SnapRadius {
            current: self.solver.precision.start,
            step: self.solver.precision.progression,
        }
    }
}