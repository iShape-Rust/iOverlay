use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use crate::mesh::outline::section::OffsetSection;
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;

pub(super) trait JoinBuilder<P: FloatPointCompatible<T>, T: FloatNumber> {
    fn add_join(
        &self,
        s0: &OffsetSection<P, T>,
        s1: &OffsetSection<P, T>,
        adapter: &FloatPointAdapter<P, T>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    );
    fn capacity(&self) -> usize;
    fn additional_offset(&self, radius: T) -> T;
}
