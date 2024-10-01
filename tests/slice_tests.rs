#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::extension::slice::Slice;

    #[test]
    fn test_miss_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_line(
            &[IntPoint::new(-15, -20), IntPoint::new(-15, 20)],
            FillRule::NonZero, 0
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_edge_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_line(
            &[IntPoint::new(-10, -20), IntPoint::new(-10, 20)],
            FillRule::NonZero, 0
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], path);
    }

    #[test]
    fn test_middle_slice() {
        let path = [
            IntPoint::new(-10, -10),
            IntPoint::new(-10, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, -10)
        ].to_vec();

        let result = path.slice_by_line(
            &[IntPoint::new(0, -20), IntPoint::new(0, 20)],
            FillRule::NonZero, 0
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 1);
    }
}