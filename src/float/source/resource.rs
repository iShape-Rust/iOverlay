use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;

pub trait OverlayResource<P, T>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    type ResourceIter<'a>: Iterator<Item=&'a [P]>
    where
        P: 'a,
        Self: 'a;

    fn iter_paths(&self) -> Self::ResourceIter<'_>;
}