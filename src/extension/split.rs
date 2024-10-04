use std::collections::HashMap;
use i_float::point::IntPoint;
use i_shape::int::path::IntPath;

pub(super) trait Split {
    fn split(self) -> Vec<Self> where Self: Sized;
}

impl Split for IntPath {
    fn split(self) -> Vec<Self> {
        let mut result: Vec<IntPath> = Vec::new();
        let mut path: IntPath = Vec::new();
        let mut map: HashMap<IntPoint, usize> = HashMap::new();

        for point in self {
            if let Some(pos) = map.remove(&point) {
                let mut sub_path = path.split_off(pos);
                sub_path.insert(0, point); // we wish it be first
                if sub_path.len() > 2 {
                    result.push(sub_path);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_path() {
        let path: IntPath = vec![];
        let result = path.split();
        assert_eq!(result, vec![] as Vec<IntPath>);
    }

    #[test]
    fn test_single_point() {
        let path = vec![IntPoint::new(0, 0)];
        let result = path.split();
        assert!(result.is_empty());
    }

    #[test]
    fn test_two_points() {
        let path = vec![IntPoint::new(0, 0), IntPoint::new(1, 1)];
        let result = path.split();
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

        let result = path.clone().split();
        assert_eq!(result, vec![path]);
    }

    #[test]
    fn test_2_loops() {
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

        let result = path.split();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], [
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
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

        let result = path.split();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], [
            IntPoint::new(2, 0),
            IntPoint::new(3, 1),
            IntPoint::new(4, 0),
            IntPoint::new(3, -1),
        ].to_vec());
        assert_eq!(result[1], [
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
            IntPoint::new(1, -1),
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

        let result = path.split();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], [
            IntPoint::new(0, 0),
            IntPoint::new(1, 1),
            IntPoint::new(2, 0),
        ].to_vec());
    }
}