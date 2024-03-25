pub(crate) trait SwapRemoveIndex {
    fn swap_remove_index(&mut self, index: usize);
}

impl<T> SwapRemoveIndex for Vec<T> {
    fn swap_remove_index(&mut self, index: usize) {
        if index + 1 < self.len() {
            self.swap_remove(index);
        } else {
            _ = self.pop()
        }
    }
}