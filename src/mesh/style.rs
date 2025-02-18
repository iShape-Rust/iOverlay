use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

/// The endpoint style of a line.
#[derive(Debug)]
pub enum LineCap<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// A line with a squared-off end. This is the default.
    Butt,
    /// A line with a rounded end. The line ends with a semicircular arc with a radius of 1/2 the line’s width, centered on the endpoint.
    /// Takes a parameter `Angle` in radians.
    Round(T),
    /// A line with a squared-off end. An extended distance equal to half the line width.
    Square,
    /// Set a custom end with template points.
    Custom(Vec<P>)
}

/// The join style of a line.
#[derive(Debug)]
pub enum LineJoin<T: FloatNumber> {
    /// Cuts off the corner where two lines meet. This is the default.
    Bevel,
    /// Creates a sharp corner where two lines meet.
    /// The corner is limited by a miter, where the parameter `Angle`
    /// is a minimum sharp angle
    Miter(T),
    /// Creates an arc corner where two lines meet, with a radius of 1/2 the line’s width.
    /// The arc is approximated using a group of segments, where the parameter `Angle`
    /// is defined as `L / R`, with `L` being the maximum segment length and `R` being the arc radius.
    Round(T),
}

/// Defines the stroke style for outlining paths.
#[derive(Debug)]
pub struct StrokeStyle<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// The width of the stroke.
    pub width: T,
    /// The cap style at the start of the stroke.
    pub start_cap: LineCap<P, T>,
    /// The cap style at the end of the stroke.
    pub end_cap: LineCap<P, T>,
    /// The join style where two lines meet.
    pub join: LineJoin<T>,
}

/// Defines the outline style for offsetting shapes.
#[derive(Debug)]
pub struct OutlineStyle<T: FloatNumber> {
    pub outer_offset: T,
    pub inner_offset: T,
    pub join: LineJoin<T>,
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> StrokeStyle<P, T> {
    /// Creates a new `StrokeStyle` with the specified width.
    pub fn new(width: T) -> Self {
        Self { width, ..Default::default() }
    }

    /// Sets the stroke width.
    pub fn width(mut self, width: T) -> Self {
        self.width = width;
        self
    }

    /// Sets the cap style at the start of the stroke.
    pub fn start_cap(mut self, cap: LineCap<P, T>) -> Self {
        self.start_cap = cap;
        self
    }

    /// Sets the cap style at the end of the stroke.
    pub fn end_cap(mut self, cap: LineCap<P, T>) -> Self {
        self.end_cap = cap;
        self
    }

    /// Sets the line join style.
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

impl<T: FloatNumber> OutlineStyle<T> {

    /// Creates a new `OutlineStyle` with the specified offset.
    pub fn new(offset: T) -> Self {
        Self { outer_offset: offset, inner_offset: offset, ..Default::default() }
    }

    /// Sets the offset distance.
    pub fn offset(mut self, offset: T) -> Self {
        self.outer_offset = offset;
        self.inner_offset = offset;
        self
    }

    /// Sets the outer distance.
    pub fn outer_offset(mut self, outer_offset: T) -> Self {
        self.outer_offset = outer_offset;
        self
    }

    /// Sets the inner distance.
    pub fn inner_offset(mut self, inner_offset: T) -> Self {
        self.inner_offset = inner_offset;
        self
    }

    /// Sets the line join style for the offset path.
    pub fn line_join(mut self, join: LineJoin<T>) -> Self {
        self.join = join;
        self
    }
}

impl<T: FloatNumber> Default for OutlineStyle<T> {
    fn default() -> Self {
        Self {
            outer_offset: T::from_float(1.0),
            inner_offset: T::from_float(1.0),
            join: LineJoin::Bevel
        }
    }
}