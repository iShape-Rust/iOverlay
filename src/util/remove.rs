pub(crate) trait SwapRemoveIndex<T> {
    fn swap_remove_index(&mut self, index: usize) -> T;
}

impl<T> SwapRemoveIndex<T> for Vec<T> {

    #[inline]
    fn swap_remove_index(&mut self, index: usize) -> T {
        if index + 1 < self.len() {
            self.swap_remove(index)
        } else {
            self.pop().unwrap()
        }
    }
}