pub (crate) const EMPTY_INDEX: usize = usize::MAX;

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

pub(crate) trait Int {
    fn log2_sqrt(&self) -> usize;
}

impl Int for usize {
    fn log2_sqrt(&self) -> usize {
        let z = self.leading_zeros();
        let i = (usize::BITS - z) as usize;
        let n = (i + 1) >> 1;
        1 << n
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Int;

    #[test]
    fn test_0() {
        let tests: Vec<[usize; 2]> = vec![
            [0, 1],
            [1, 2],
            [3, 2],
            [15, 4],
            [16, 8],
            [255, 16],
            [256, 32],
        ];

        for test in tests {
            let a = test[0].log2_sqrt();
            let b = test[1];
            assert_eq!(a, b);
        }
    }

}