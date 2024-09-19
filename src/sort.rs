#[cfg(feature = "allow_multithreading")]
use rayon::prelude::*;
use std::cmp::Ordering;
use i_key_sort::index::BinKey;
use crate::core::solver::Solver;
use i_key_sort::key_sort::KeyBinSort;

pub(crate) trait SmartSort {
    fn smart_sort_by<F>(&mut self, solver: &Solver, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering + Sync;

    type Item: Send;
}

impl<T: BinKey + Clone + Send> SmartSort for [T] {
    fn smart_sort_by<F>(&mut self, _solver: &Solver, compare: F)
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
    use i_key_sort::index::BinLayout;
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

    impl BinKey for Point {
        #[inline(always)]
        fn key(&self) -> i64 {
            self.x as i64
        }

        #[inline(always)]
        fn bin(&self, layout: &BinLayout) -> usize {
            layout.index(self.x.into())
        }
    }

    #[test]
    fn test_sort_by() {
        let mut data = vec![
            Point {x: 5, y: 1},
            Point {x: 3, y: 1},
            Point {x: 1, y: 1},
            Point {x: 4, y: 1},
            Point {x: 2, y: 1}
        ];
        data.smart_sort_by(&Solver::AUTO, |a, b| a.cmp(&b));

        assert_eq!(data, vec![
            Point {x: 1, y: 1},
            Point {x: 2, y: 1},
            Point {x: 3, y: 1},
            Point {x: 4, y: 1},
            Point {x: 5, y: 1}
        ]);
    }
}