#[derive(Debug)]
pub(crate) enum OverlayNode {
    Bridge([usize; 2]),
    Cross(Vec<usize>),
}

impl OverlayNode {
    #[inline]
    pub(super) fn new(indices: &[usize]) -> Self {
        if indices.len() == 2 {
            Self::Bridge(unsafe { [*indices.get_unchecked(0), *indices.get_unchecked(1)] })
        } else {
            Self::Cross(indices.to_vec())
        }
    }
}