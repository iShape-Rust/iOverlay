use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::float::source::resource::OverlayResource;

pub struct DoubleResourceIterator<'a, P> {
    slice: &'a [Vec<P>],
    index: usize,
}

impl<'a, P> DoubleResourceIterator<'a, P> {
    #[inline]
    fn with_slice(slice: &'a [Vec<P>]) -> Self {
        Self { slice, index: 0 }
    }
}

impl<'a, P> Iterator for DoubleResourceIterator<'a, P> {
    type Item = &'a [P];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.slice.len() {
            return None;
        }
        let i = self.index;
        self.index += 1;
        let it = unsafe { self.slice.get_unchecked(i) };

        Some(it.as_slice())
    }
}

impl<P, T> OverlayResource<P, T> for [Vec<P>]
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    type ResourceIter<'a> = DoubleResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        DoubleResourceIterator::with_slice(self)
    }
}

impl<P, T, const N: usize> OverlayResource<P, T> for [Vec<P>; N]
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    type ResourceIter<'a> = DoubleResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        DoubleResourceIterator::with_slice(self)
    }
}

impl<P, T> OverlayResource<P, T> for Vec<Vec<P>>
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    type ResourceIter<'a> = DoubleResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        DoubleResourceIterator::with_slice(self.as_slice())
    }
}

impl<'b, P, T> OverlayResource<P, T> for &'b [Vec<P>]
where
    P: FloatPointCompatible<T>,
    T: FloatNumber,
{
    type ResourceIter<'a> = DoubleResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_paths(&self) -> Self::ResourceIter<'b> {
        DoubleResourceIterator::with_slice(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::float::source::resource::OverlayResource;

    #[test]
    fn test_resource_fixed_array() {
        let array = [vec![[0.0, 0.0], [0.0, 1.0]]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_slice_array() {
        let array = [vec![[0.0, 0.0], [0.0, 1.0]]];

        let count = array.as_slice().iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }

    #[test]
    fn test_resource_vec_array() {
        let array = vec![vec![[0.0, 0.0], [0.0, 1.0]]];

        let count = array.iter_paths().fold(0, |s, it| s + it.len());

        assert_eq!(count, 2);
    }
}