use crate::mesh::variable_stroke::style::StrokeVertex;
use alloc::vec::Vec;
use i_float::float::compatible::FloatPointCompatible;

pub trait VariableStrokeSource<P>
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>: Iterator<Item = &'a [StrokeVertex<P>]>
    where
        P: 'a,
        Self: 'a;

    fn iter_variable_paths(&self) -> Self::ResourceIter<'_>;
}

pub struct ContourResourceIterator<'a, P: FloatPointCompatible> {
    slice: &'a [StrokeVertex<P>],
    finished: bool,
}

impl<'a, P: FloatPointCompatible> ContourResourceIterator<'a, P> {
    #[inline]
    fn with_slice(slice: &'a [StrokeVertex<P>]) -> Self {
        Self {
            slice,
            finished: false,
        }
    }
}

impl<'a, P: FloatPointCompatible> Iterator for ContourResourceIterator<'a, P> {
    type Item = &'a [StrokeVertex<P>];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        self.finished = true;
        Some(self.slice)
    }

    #[inline]
    fn count(self) -> usize {
        1
    }
}

impl<P> VariableStrokeSource<P> for [StrokeVertex<P>]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ContourResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self)
    }
}

impl<P, const N: usize> VariableStrokeSource<P> for [StrokeVertex<P>; N]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ContourResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self)
    }
}

impl<P> VariableStrokeSource<P> for Vec<StrokeVertex<P>>
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ContourResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self.as_slice())
    }
}

impl<'b, P> VariableStrokeSource<P> for &'b [StrokeVertex<P>]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ContourResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'b> {
        ContourResourceIterator::with_slice(self)
    }
}

pub struct ShapeResourceIterator<'a, P: FloatPointCompatible> {
    slice: &'a [Vec<StrokeVertex<P>>],
    index: usize,
}

impl<'a, P: FloatPointCompatible> ShapeResourceIterator<'a, P> {
    #[inline]
    fn with_slice(slice: &'a [Vec<StrokeVertex<P>>]) -> Self {
        Self { slice, index: 0 }
    }
}

impl<'a, P: FloatPointCompatible> Iterator for ShapeResourceIterator<'a, P> {
    type Item = &'a [StrokeVertex<P>];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.slice.len() {
            return None;
        }

        let i = self.index;
        self.index += 1;
        let path = unsafe { self.slice.get_unchecked(i) };

        Some(path.as_slice())
    }

    #[inline]
    fn count(self) -> usize {
        self.slice.len()
    }
}

impl<P> VariableStrokeSource<P> for [Vec<StrokeVertex<P>>]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator::with_slice(self)
    }
}

impl<P, const N: usize> VariableStrokeSource<P> for [Vec<StrokeVertex<P>>; N]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator::with_slice(self)
    }
}

impl<P> VariableStrokeSource<P> for Vec<Vec<StrokeVertex<P>>>
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator::with_slice(self.as_slice())
    }
}

impl<'b, P> VariableStrokeSource<P> for &'b [Vec<StrokeVertex<P>>]
where
    P: FloatPointCompatible,
{
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'b> {
        ShapeResourceIterator::with_slice(self)
    }
}
