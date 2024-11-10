use i_float::float::number::FloatNumber;

#[derive(Clone, Copy)]
pub struct Filter<T> {
    pub min_area: T,
    pub simplify: bool,
}

impl<T: FloatNumber> Default for Filter<T> {
    fn default() -> Self {
        Filter { min_area: T::from_float(0.0), simplify: false }
    }
}