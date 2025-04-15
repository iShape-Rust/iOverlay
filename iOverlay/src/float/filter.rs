use i_float::float::number::FloatNumber;

#[derive(Clone, Copy)]
pub struct ContourFilter<T> {
    pub min_area: T,
    pub simplify: bool,
}

impl<T: FloatNumber> Default for ContourFilter<T> {
    fn default() -> Self {
        // f32 precision is not enough to cover i32
        let simplify = T::bit_width() <= 32;
        ContourFilter { min_area: T::from_float(0.0), simplify }
    }
}