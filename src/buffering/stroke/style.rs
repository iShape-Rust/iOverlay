use i_float::float::number::FloatNumber;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokeStyle<T: FloatNumber> {
    pub width: T,
    pub round_limit: T,
    pub miter_limit: T,
    pub begin_cap: LineCap,
    pub end_cap: LineCap,
    pub join: LineJoin,
}

impl<T: FloatNumber> StrokeStyle<T> {
    pub fn new(width: T) -> Self {
        Self { width, ..Default::default() }
    }

    pub fn line_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn line_cap(mut self, cap: LineCap) -> Self {
        self.begin_cap = cap;
        self.end_cap = cap;
        self
    }

    pub fn line_join(mut self, join: LineJoin) -> Self {
        self.join = join;
        self
    }
}

impl<T: FloatNumber> Default for StrokeStyle<T> {
    fn default() -> Self {
        Self {
            width: T::from_float(1.0),
            round_limit: T::from_float(0.5),
            miter_limit: T::from_float(2.0),
            begin_cap: LineCap::Butt,
            end_cap: LineCap::Butt,
            join: LineJoin::Round
        }
    }
}