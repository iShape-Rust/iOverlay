#[cfg(feature = "allow_multithreading")]
use rayon::prelude::*;
use std::cmp::Ordering;
use crate::core::solver::Solver;


pub(crate) trait SmartSort {
    fn smart_sort_by<F>(&mut self, solver: &Solver, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering + Sync;

    type Item: Send;
}

impl<T: Send> SmartSort for [T] {
    fn smart_sort_by<F>(&mut self, solver: &Solver, compare: F)
    where
        F: Fn(&T, &T) -> Ordering + Sync,
    {
        #[cfg(feature = "allow_multithreading")]
        {
            if let Some(multithreading) = solver.multithreading {
                if self.len() > multithreading.par_sort_min_size {
                    self.par_sort_unstable_by(compare);
                    return;
                }
            }
        }

        // Fallback to standard sort_unstable_by if multithreading is not enabled or the data is below the threshold
        self.sort_by(compare);
    }

    type Item = T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by() {
        let mut data = vec![5, 3, 1, 4, 2];
        data.smart_sort_by(&Solver::AUTO, |a, b| a.cmp(b));
        assert_eq!(data, vec![1, 2, 3, 4, 5]);

        let mut large_data: Vec<i32> = (0..200_000).rev().collect();
        large_data.smart_sort_by(&Solver::AUTO, |a, b| a.cmp(b));
        let sorted_large_data: Vec<i32> = (0..200_000).collect();
        assert_eq!(large_data, sorted_large_data);
    }
}