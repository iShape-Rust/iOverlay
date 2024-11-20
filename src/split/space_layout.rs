pub(super) struct SpaceLayout {
    pub(super) power: usize,
}

impl SpaceLayout {
    const MIN_POWER: usize = 2;
    const MAX_POWER: usize = 12;
    pub(super) const MIN_HEIGHT: usize = 1 << Self::MIN_POWER;

    pub(super) fn new(height: usize, count: usize) -> Self {
        let max_power_range = (height.ilog2() - 1) as usize;
        let max_power_count = (count.ilog2() >> 1) as usize;
        let original_power = max_power_range.min(max_power_count);
        let power = original_power.clamp(Self::MIN_POWER, Self::MAX_POWER);
        Self { power }
    }
}