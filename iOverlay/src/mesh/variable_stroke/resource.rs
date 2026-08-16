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

impl<P: FloatPointCompatible> VariableStrokeSource<P> for [StrokeVertex<P>] {
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

impl<P: FloatPointCompatible, const N: usize> VariableStrokeSource<P> for [StrokeVertex<P>; N] {
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

impl<P: FloatPointCompatible> VariableStrokeSource<P> for Vec<StrokeVertex<P>> {
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

impl<'b, P: FloatPointCompatible> VariableStrokeSource<P> for &'b [StrokeVertex<P>] {
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

impl<'a, P: FloatPointCompatible> Iterator for ShapeResourceIterator<'a, P> {
    type Item = &'a [StrokeVertex<P>];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let path = self.slice.get(self.index)?;
        self.index += 1;
        Some(path.as_slice())
    }

    #[inline]
    fn count(self) -> usize {
        self.slice.len()
    }
}

impl<P: FloatPointCompatible> VariableStrokeSource<P> for [Vec<StrokeVertex<P>>] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator {
            slice: self,
            index: 0,
        }
    }
}

impl<P: FloatPointCompatible, const N: usize> VariableStrokeSource<P> for [Vec<StrokeVertex<P>>; N] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator {
            slice: self,
            index: 0,
        }
    }
}

impl<P: FloatPointCompatible> VariableStrokeSource<P> for Vec<Vec<StrokeVertex<P>>> {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'_> {
        ShapeResourceIterator {
            slice: self.as_slice(),
            index: 0,
        }
    }
}

impl<'b, P: FloatPointCompatible> VariableStrokeSource<P> for &'b [Vec<StrokeVertex<P>>] {
    type ResourceIter<'a>
        = ShapeResourceIterator<'a, P>
    where
        P: 'a,
        Self: 'a;

    #[inline]
    fn iter_variable_paths(&self) -> Self::ResourceIter<'b> {
        ShapeResourceIterator {
            slice: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VariableStrokeSource;
    use crate::mesh::variable_stroke::StrokeVertex;
    use alloc::vec;
    use alloc::vec::Vec;

    type Vertex = StrokeVertex<[f64; 2]>;

    fn path(y: f64) -> Vec<Vertex> {
        vec![
            StrokeVertex::new([0.0, y], 2.0),
            StrokeVertex::new([10.0, y], 4.0),
        ]
    }

    #[test]
    fn contour_resource_forms_yield_exactly_one_path() {
        let array = [
            StrokeVertex::new([0.0, 0.0], 2.0),
            StrokeVertex::new([10.0, 0.0], 4.0),
        ];
        let slice: &[Vertex] = &array;
        let owned = array.to_vec();

        assert_eq!(
            <[Vertex; 2] as VariableStrokeSource<_>>::iter_variable_paths(&array).count(),
            1
        );
        assert_eq!(
            <Vec<Vertex> as VariableStrokeSource<_>>::iter_variable_paths(&owned).count(),
            1
        );

        let mut slice_iter = <[Vertex] as VariableStrokeSource<_>>::iter_variable_paths(slice);
        assert_eq!(slice_iter.next().unwrap().len(), 2);
        assert!(slice_iter.next().is_none());

        let mut reference_iter = <&[Vertex] as VariableStrokeSource<_>>::iter_variable_paths(&slice);
        assert_eq!(reference_iter.next().unwrap()[1].point, [10.0, 0.0]);
        assert!(reference_iter.next().is_none());
    }

    #[test]
    fn shape_resource_forms_preserve_path_order_and_count() {
        let array = [path(0.0), path(10.0)];
        let slice: &[Vec<Vertex>] = &array;
        let owned = array.to_vec();

        assert_eq!(
            <[Vec<Vertex>; 2] as VariableStrokeSource<_>>::iter_variable_paths(&array).count(),
            2
        );
        assert_eq!(
            <[Vec<Vertex>] as VariableStrokeSource<_>>::iter_variable_paths(slice).count(),
            2
        );
        assert_eq!(
            <Vec<Vec<Vertex>> as VariableStrokeSource<_>>::iter_variable_paths(&owned).count(),
            2
        );

        let mut reference_iter = <&[Vec<Vertex>] as VariableStrokeSource<_>>::iter_variable_paths(&slice);
        assert_eq!(reference_iter.next().unwrap()[0].point, [0.0, 0.0]);
        assert_eq!(reference_iter.next().unwrap()[0].point, [0.0, 10.0]);
        assert!(reference_iter.next().is_none());
    }
}
