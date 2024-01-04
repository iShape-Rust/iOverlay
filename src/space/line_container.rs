use crate::space::dual_index::DualIndex;

#[derive(Copy, Clone)]
pub struct LineContainer<Id: Copy> {
    pub id: Id,
    pub index: DualIndex
}