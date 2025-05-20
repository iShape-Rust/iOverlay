#[cfg(feature = "allow_multithreading")]
use rayon::prelude::*;
use core::cmp::Ordering;
use i_key_sort::bin_key::index::{BinKey, BinLayoutOp};
use i_key_sort::sort::key_sort::KeyBinSort;
use crate::core::solver::Solver;

pub(crate) trait SmartBinSort<U> {
    fn smart_bin_sort_by<F>(&mut self, solver: &Solver, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering + Sync;

    type Item: Send;
}

impl<T, U> SmartBinSort<U> for [T]
where
    T: BinKey<U> + Clone + Send,
    U: Copy + Ord + BinLayoutOp,
{
    fn smart_bin_sort_by<F>(&mut self, _solver: &Solver, compare: F)
    where
        F: Fn(&T, &T) -> Ordering + Sync,
    {
        #[cfg(feature = "allow_multithreading")]
        {
            if let Some(multithreading) = _solver.multithreading {
                if self.len() > multithreading.par_sort_min_size {
                    self.par_sort_unstable_by(compare);
                    return;
                }
            }
        }

        // Fallback to standard sort if multithreading is not enabled
        self.sort_with_bins(compare)
    }

    type Item = T;
}
#[cfg(test)]
mod tests {
    use alloc::vec;
    use i_key_sort::bin_key::index::BinLayout;
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl PartialOrd for Point {
        #[inline(always)]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Point {
        #[inline(always)]
        fn cmp(&self, other: &Self) -> Ordering {
            let x = self.x == other.x;
            if x && self.y == other.y {
                Ordering::Equal
            } else if self.x < other.x || x && self.y < other.y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }

    impl BinKey<i32> for Point {
        #[inline(always)]
        fn bin_key(&self) -> i32 {
            self.x
        }

        #[inline(always)]
        fn bin_index(&self, layout: &BinLayout<i32>) -> usize {
            layout.index(self.x.into())
        }
    }

    #[test]
    fn test_sort_by() {
        let mut data = vec![
            Point { x: 5, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 4, y: 1 },
            Point { x: 2, y: 1 },
        ];
        data.smart_bin_sort_by(&Solver::AUTO, |a, b| a.cmp(&b));

        assert_eq!(data, vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 4, y: 1 },
            Point { x: 5, y: 1 },
        ]);
    }
}