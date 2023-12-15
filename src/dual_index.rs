use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DualIndex {
    pub major: usize,
    pub minor: usize
}

impl DualIndex {
    pub const EMPTY: Self = Self { major: usize::MAX, minor: usize::MAX };

    pub fn order_asc_major_des_minor(&self, other: &Self) -> Ordering {
        if self.major == other.major {
            if self.minor > other.minor {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            if self.minor == other.minor {
                Ordering::Equal
            } else if self.major < other.major {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}