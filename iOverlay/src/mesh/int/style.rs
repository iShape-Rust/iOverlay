use i_float::int::number::int::IntNumber;

/// Join styles for integer mesh operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntLineJoin {
    #[default]
    Bevel,
    /// Placeholder until integer angles and miter construction are implemented.
    /// Requesting this join returns `IntOutlineError::UnsupportedJoin`.
    Miter,
    /// Placeholder until integer angles and arc construction are implemented.
    /// Requesting this join returns `IntOutlineError::UnsupportedJoin`.
    Round,
}

/// Signed offsets in input coordinate units. Positive values expand the filled
/// shape: the outer boundary grows and holes shrink. Negative values reverse it.
/// Distances have no fractional part and are never automatically rescaled.
#[derive(Debug, Clone, Copy)]
pub struct IntOutlineStyle<I: IntNumber = i32> {
    pub outer_offset: I,
    pub inner_offset: I,
    pub join: IntLineJoin,
}

impl<I: IntNumber> IntOutlineStyle<I> {
    pub fn new(offset: I) -> Self {
        Self {
            outer_offset: offset,
            inner_offset: offset,
            join: IntLineJoin::Bevel,
        }
    }

    pub fn offset(mut self, offset: I) -> Self {
        self.outer_offset = offset;
        self.inner_offset = offset;
        self
    }

    pub fn outer_offset(mut self, offset: I) -> Self {
        self.outer_offset = offset;
        self
    }

    pub fn inner_offset(mut self, offset: I) -> Self {
        self.inner_offset = offset;
        self
    }

    pub fn line_join(mut self, join: IntLineJoin) -> Self {
        self.join = join;
        self
    }
}
