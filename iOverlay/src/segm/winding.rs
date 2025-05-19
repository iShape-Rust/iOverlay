use crate::core::overlay::ShapeType;

pub(crate) trait WindingCount
where
    Self: Clone + Copy + Send,
{
    fn is_not_empty(&self) -> bool;
    fn new(subj: i32, clip: i32) -> Self;
    fn with_shape_type(shape_type: ShapeType) -> (Self, Self);
    fn add(self, count: Self) -> Self;
    fn apply(&mut self, count: Self);
    fn invert(self) -> Self;
}