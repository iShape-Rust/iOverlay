use super::arc::ArcOptions;
use crate::mesh::math::MathMode;
use i_float::int::angle::Angle;
use i_float::int::number::int::IntNumber;

/// Join styles for integer mesh operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntLineJoin {
    #[default]
    Bevel,
    /// Clipped miter, limited by the minimum interior angle.
    /// The angle is clamped to 0.01*pi..=0.99*pi.
    Miter(Angle),
    /// Rounded join with reusable integer rotation settings.
    Round(ArcOptions),
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

/// End caps for integer strokes. Custom points are local displacements in input
/// coordinate units, with x pointing outward and y pointing to its left. They
/// are already scaled to the desired size; no fractional template scale is used.
#[derive(Debug, Clone)]
pub enum IntLineCap<I: IntNumber = i32> {
    Butt,
    Square,
    Round(ArcOptions),
    Custom(alloc::rc::Rc<[i_float::int::point::IntPoint<I>]>),
}

#[derive(Debug, Clone)]
pub struct IntStrokeStyle<I: IntNumber = i32> {
    pub width: I,
    pub start_cap: IntLineCap<I>,
    pub end_cap: IntLineCap<I>,
    pub join: IntLineJoin,
    /// Arithmetic for stroke construction. Float mode may change rounded output.
    pub math: MathMode,
}

impl<I: IntNumber> IntStrokeStyle<I> {
    pub fn new(width: I) -> Self {
        Self {
            width,
            start_cap: IntLineCap::Butt,
            end_cap: IntLineCap::Butt,
            join: IntLineJoin::Bevel,
            math: MathMode::Integer,
        }
    }
    /// Selects construction arithmetic; coordinates and boolean operations stay integer.
    pub fn math(mut self, math: MathMode) -> Self {
        self.math = math;
        self
    }

    pub fn width(mut self, width: I) -> Self {
        self.width = width;
        self
    }
    pub fn start_cap(mut self, cap: IntLineCap<I>) -> Self {
        self.start_cap = cap;
        self
    }
    pub fn end_cap(mut self, cap: IntLineCap<I>) -> Self {
        self.end_cap = cap;
        self
    }
    pub fn line_join(mut self, join: IntLineJoin) -> Self {
        self.join = join;
        self
    }
}
