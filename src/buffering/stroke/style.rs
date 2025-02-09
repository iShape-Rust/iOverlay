use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

#[derive(Debug)]
pub enum LineCap<P: FloatPointCompatible<T>, T: FloatNumber> {
    Butt,
    Round(T),
    Square,
    Custom(Vec<P>)
}

#[derive(Debug)]
pub enum LineJoin<T: FloatNumber> {
    Miter(T),
    Round(T), // A / R; A - arc length, R - radius
    Bevel,
}

#[derive(Debug)]
pub struct StrokeStyle<P: FloatPointCompatible<T>, T: FloatNumber> {
    pub(super) width: T,
    pub(super) start_cap: LineCap<P, T>,
    pub(super) end_cap: LineCap<P, T>,
    pub(super) join: LineJoin<T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> StrokeStyle<P, T> {
    pub fn new(width: T) -> Self {
        Self { width, ..Default::default() }
    }

    pub fn width(mut self, width: T) -> Self {
        self.width = width;
        self
    }

    pub fn start_cap(mut self, cap: LineCap<P, T>) -> Self {
        self.start_cap = cap;
        self
    }

    pub fn end_cap(mut self, cap: LineCap<P, T>) -> Self {
        self.end_cap = cap;
        self
    }

    pub fn line_join(mut self, join: LineJoin<T>) -> Self {
        self.join = join;
        self
    }
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> Default for StrokeStyle<P, T> {
    fn default() -> Self {
        Self {
            width: T::from_float(1.0),
            start_cap: LineCap::Butt,
            end_cap: LineCap::Butt,
            join: LineJoin::Bevel
        }
    }
}