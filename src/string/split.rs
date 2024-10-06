use std::collections::HashMap;
use i_float::point::IntPoint;
use i_shape::int::path::{IntPath, PointPathExtension};

pub(super) trait Split {
    fn split_loops(self, min_area: usize) -> Vec<Self>
    where
        Self: Sized;
}

impl Split for IntPath {
    fn split_loops(self, min_area: usize) -> Vec<Self> {
        let mut result: Vec<IntPath> = Vec::new();
        let mut path: IntPath = Vec::new();
        let mut map: HashMap<IntPoint, usize> = HashMap::new();

        for point in self {
            if let Some(&pos) = map.get(&point) {
                let tail_len = path.len() - pos;
                if tail_len < 2 {
                    // tail is too small
                    path.truncate(pos);
                } else {
                    let mut tail = path.split_off(pos);
                    tail.push(point);
                    if tail.validate_area(min_area) {
                        result.push(tail);
                    }
                }
            } else {
                path.push(point);
                let pos = path.len();
                map.insert(point, pos);
            }
        }

        if path.len() > 2 {
            result.push(path);
        }

        result
    }
}

trait ValidateArea {
    fn validate_area(&self, min_area: usize) -> bool;
}

impl ValidateArea for IntPath {
    #[inline]
    fn validate_area(&self, min_area: usize) -> bool {
        if min_area == 0 {
            return true;
        }
        let abs_area = self.unsafe_area().unsigned_abs() as usize >> 1;
        abs_area < min_area
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_path() {
        let path: IntPath = vec![];
        let result = path.split_loops(0);
        assert_eq!(result, vec![] as Vec<IntPath>);
    }

    #[test]
    fn test_single_point() {
        let path = vec![IntPoint::new(0, 0)];
        let result = path.split_loops(0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_two_points() {
        let path = vec![IntPoint::new(0, 0), IntPoint::new(1, 1)];
        let result = path.split_loops(0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_no_repeated_points() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(0, 1),
            IntPoint::new(1, 1),
            IntPoint::new(1, 0),
        ];

        let result = path.clone().split_loops(0);
        assert_eq!(result, vec![path]);
    }

    #[test]
    fn test_2_loops_0() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0), // same point
            IntPoint::new(1, -1),
        ];

        let result = path.split_loops(0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], [
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0),
        ].to_vec());
        assert_eq!(result[1], [
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(1, -1),
        ].to_vec());
    }

    #[test]
    fn test_2_loops_1() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0), // same point
            IntPoint::new(1, -1),
        ];

        let result = path.split_loops(0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], [
            IntPoint::new(3, 1),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0),
        ].to_vec());
        assert_eq!(result[1], [
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(1, -1),
        ].to_vec());
    }

    #[test]
    fn test_2_loops_with_tails() {
        let path = vec![
            IntPoint::new(-1, 0),
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(5, 0),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0), // same point
            IntPoint::new(1, -1),
            IntPoint::new(0, 0),
        ];

        let result = path.split_loops(0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], [
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
            IntPoint::new(2, 0),
        ].to_vec());
        assert_eq!(result[1], [
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(1, -1),
            IntPoint::new(0, 0),
        ].to_vec());
    }

    #[test]
    fn test_single_loop() {
        let path = vec![
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(0, 0), // same point, forms a loop
        ];

        let result = path.split_loops(0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], [
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(0, 0),
        ].to_vec());
    }

    #[test]
    fn test_cross() {
        let path = vec![
            IntPoint::new(-2, -1),
            IntPoint::new(-2, 1),
            IntPoint::new(0, 0),
            IntPoint::new(-1, 2),
            IntPoint::new(1, 2),
            IntPoint::new(0, 0), // same point, forms a loop
            IntPoint::new(2, 1),
            IntPoint::new(2, -1),
            IntPoint::new(0, 0), // same point, forms a loop
            IntPoint::new(1, -2),
            IntPoint::new(-1, -2),
            IntPoint::new(0, 0), // same point, forms a loop
        ];

        let result = path.split_loops(0);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], [
            IntPoint::new(-1, 2),
            IntPoint::new(1, 2),
            IntPoint::new(0, 0),
        ].to_vec());
        assert_eq!(result[1], [
            IntPoint::new(2, 1),
            IntPoint::new(2, -1),
            IntPoint::new(0, 0),
        ].to_vec());
        assert_eq!(result[2], [
            IntPoint::new(1, -2),
            IntPoint::new(-1, -2),
            IntPoint::new(0, 0),
        ].to_vec());
        assert_eq!(result[3], [
            IntPoint::new(-2, -1),
            IntPoint::new(-2, 1),
            IntPoint::new(0, 0),
        ].to_vec());
    }
}