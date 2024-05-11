pub(super) trait RangeSearch {
    fn find_index(&self, x: i32) -> u32;
}

impl RangeSearch for Vec<i32> {

    fn find_index(&self, x: i32) -> u32 {
        let mut left = 0;
        let mut right = self.len();

        while left < right {
            let mid = left + ((right - left) >> 1);
            unsafe {
                let val = *self.get_unchecked(mid);
                if val == x {
                    return mid as u32;
                } else if val < x {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }

        left as u32
    }
}