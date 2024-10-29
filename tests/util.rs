#[cfg(test)]
pub mod overlay {
    use i_float::int::point::IntPoint;
    use i_shape::int::shape::IntShape;

    pub trait CircleCompare {
        fn are_equal(&self, other: &Self) -> bool;
    }

    impl CircleCompare for Vec<IntPoint> {
        fn are_equal(&self, other: &Self) -> bool {
            if self.len() != other.len() {
                return false;
            }

            let len = other.len();

            for shift in 0..len {
                let mut is_equal = true;
                for i in 0..len {
                    if self[(i + shift) % len] != other[i] {
                        is_equal = false;
                        break;
                    }
                }
                if is_equal {
                    return true;
                }
            }

            false
        }
    }

    impl CircleCompare for Vec<IntShape> {
        fn are_equal(&self, other: &Self) -> bool {
            if self.len() != other.len() {
                return false;
            }

            for i in 0..self.len() {
                let shape_0 = &self[i];
                let shape_1 = &other[i];
                if shape_0.len() != shape_1.len() {
                    return false;
                }

                for j in 0..shape_0.len() {
                    let path_0 = &shape_0[j];
                    let path_1 = &shape_1[j];
                    if !path_0.are_equal(path_1) {
                        return false;
                    }
                }
            }

            true
        }
    }
}

