use crate::mesh::int::variable_stroke::style::IntStrokeVertex;
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;

pub trait IntVariableStrokeSource<I>
where
    I: IntNumber,
{
    type ResourceIter<'a>: Iterator<Item = &'a [IntStrokeVertex<I>]>
    where
        I: 'a,
        Self: 'a;

    fn iter_variable_paths(&self) -> Self::ResourceIter<'_>;
}

pub struct ContourResourceIterator<'a, I: IntNumber> {
    slice: &'a [IntStrokeVertex<I>],
    finished: bool,
}

impl<'a, I: IntNumber> ContourResourceIterator<'a, I> {
    #[inline]
    fn with_slice(slice: &'a [IntStrokeVertex<I>]) -> Self {
        Self {
            slice,
            finished: false,
        }
    }
}

impl<'a, I: IntNumber> Iterator for ContourResourceIterator<'a, I> {
    type Item = &'a [IntStrokeVertex<I>];

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
        usize::from(!self.finished)
    }
}

impl<I: IntNumber> IntVariableStrokeSource<I> for [IntStrokeVertex<I>] {
    type ResourceIter<'a>
        = ContourResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self)
    }
}

impl<I: IntNumber, const N: usize> IntVariableStrokeSource<I> for [IntStrokeVertex<I>; N] {
    type ResourceIter<'a>
        = ContourResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self)
    }
}

impl<I: IntNumber> IntVariableStrokeSource<I> for Vec<IntStrokeVertex<I>> {
    type ResourceIter<'a>
        = ContourResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ContourResourceIterator::with_slice(self.as_slice())
    }
}

impl<'b, I: IntNumber> IntVariableStrokeSource<I> for &'b [IntStrokeVertex<I>] {
    type ResourceIter<'a>
        = ContourResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'b> {
        ContourResourceIterator::with_slice(self)
    }
}

pub struct ShapeResourceIterator<'a, I: IntNumber> {
    slice: &'a [Vec<IntStrokeVertex<I>>],
    index: usize,
}

impl<'a, I: IntNumber> Iterator for ShapeResourceIterator<'a, I> {
    type Item = &'a [IntStrokeVertex<I>];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let path = self.slice.get(self.index)?;
        self.index += 1;
        Some(path.as_slice())
    }

    #[inline]
    fn count(self) -> usize {
        self.slice.len() - self.index
    }
}

impl<I: IntNumber> IntVariableStrokeSource<I> for [Vec<IntStrokeVertex<I>>] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator {
            slice: self,
            index: 0,
        }
    }
}

impl<I: IntNumber, const N: usize> IntVariableStrokeSource<I> for [Vec<IntStrokeVertex<I>>; N] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator {
            slice: self,
            index: 0,
        }
    }
}

impl<I: IntNumber> IntVariableStrokeSource<I> for Vec<Vec<IntStrokeVertex<I>>> {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator {
            slice: self.as_slice(),
            index: 0,
        }
    }
}

impl<'b, I: IntNumber> IntVariableStrokeSource<I> for &'b [Vec<IntStrokeVertex<I>>] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, I>
    where
        I: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'b> {
        ShapeResourceIterator {
            slice: self,
            index: 0,
        }
    }
}
