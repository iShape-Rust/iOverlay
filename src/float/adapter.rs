use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

pub(crate) trait AdapterExt<T: FloatNumber> {
    fn convert_area(&self, area: T) -> usize;
}

impl<P: FloatPointCompatible<T>, T: FloatNumber> AdapterExt<T> for FloatPointAdapter<P, T> {
    #[inline]
    fn convert_area(&self, area: T) -> usize {
        let scale = self.dir_scale;
        let sqr_scale = scale * scale;
        (sqr_scale * area).to_f64() as usize
    }
}