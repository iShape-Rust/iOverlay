use i_float::float::number::FloatNumber;

#[derive(Clone, Copy)]
pub struct ContourFilter<T> {
    pub min_area: T,
    pub simplify: bool,
}

impl<T: FloatNumber> Default for ContourFilter<T> {
    fn default() -> Self {
        ContourFilter { min_area: T::from_float(0.0), simplify: false }
    }
}