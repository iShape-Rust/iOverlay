use super::arc::ArcOptions;
use super::math::backend::DEFAULT_MITER_MIN_TURN;
use crate::mesh::math::MathMode;
use i_float::int::angle::Angle;
use i_float::int::number::int::IntNumber;

/// Join styles for integer mesh operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntLineJoin {
    #[default]
    Bevel,
    /// Clipped miter, limited by the minimum interior angle.
    /// Integer math clamps the minimum interior angle to at least 5 degrees.
    /// Float math clamps the minimum to 0.01*pi (1.8 degrees).
    /// Both default to bevel joins for turns below 5 degrees; the style's
    /// `miter_min_turn` configures this cutoff independently. Both clamp the maximum to one Angle unit
    /// below pi. The floating-point LineJoin adapter additionally clamps its
    /// input angle to 0.01*pi..=0.99*pi before conversion to this type.
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
    /// Miter turns below this angle use bevel joins in both math modes.
    /// Defaults to 5 degrees; clamped to 0..=pi during construction.
    /// Zero disables the cutoff. Smaller values allow less stable intersections.
    /// Ignored for bevel and round joins.
    pub miter_min_turn: Angle,
    /// Arithmetic used to construct offsets and joins.
    pub math: MathMode,
}

impl<I: IntNumber> IntOutlineStyle<I> {
    pub fn new(offset: I) -> Self {
        Self {
            outer_offset: offset,
            inner_offset: offset,
            join: IntLineJoin::Bevel,
            miter_min_turn: Angle::from_bits(DEFAULT_MITER_MIN_TURN),
            math: MathMode::Integer,
        }
    }

    /// Selects construction arithmetic; boolean operations stay integer.
    pub fn math(mut self, math: MathMode) -> Self {
        self.math = math;
        self
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

    /// Sets the near-straight bevel cutoff, independently of the miter clipping angle.
    pub fn miter_min_turn(mut self, angle: Angle) -> Self {
        self.miter_min_turn = angle;
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
    /// Miter turns below this angle use bevel joins in both math modes.
    /// Defaults to 5 degrees; clamped to 0..=pi during construction.
    /// Zero disables the cutoff. Smaller values allow less stable intersections.
    /// Ignored for bevel and round joins.
    pub miter_min_turn: Angle,
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
            miter_min_turn: Angle::from_bits(DEFAULT_MITER_MIN_TURN),
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

    /// Sets the near-straight bevel cutoff, independently of the miter clipping angle.
    pub fn miter_min_turn(mut self, angle: Angle) -> Self {
        self.miter_min_turn = angle;
        self
    }
}
