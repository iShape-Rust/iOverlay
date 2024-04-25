#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_overlay::split::cross_solver::CrossResult::PureRound;
    use i_overlay::split::cross_solver::ScanCrossSolver;
    use i_overlay::x_segment::XSegment;

    #[test]
    fn test_0() {
        let target = XSegment::new(IntPoint::new(-1, 0), IntPoint::new(0, 10));
        let other = XSegment::new(IntPoint::new(-2, 0), IntPoint::new(2, 2));

        let result = ScanCrossSolver::scan_cross(&target, &other).unwrap();

        if let PureRound(point) = result {
            assert_eq!(point, IntPoint::new(-1, 0));
        }
    }


    #[test]
    fn test_1() {
        let target = XSegment::new(IntPoint::new(-1, 10), IntPoint::new(0, 0));
        let other = XSegment::new(IntPoint::new(-2, 0), IntPoint::new(2, 2));

        let result = ScanCrossSolver::scan_cross(&target, &other).unwrap();

        if let PureRound(point) = result {
            assert_eq!(point, IntPoint::new(0, 1));
        }
    }

    #[test]
    fn test_2() {
        let target = XSegment::new(IntPoint::new(0, 0), IntPoint::new(1, 10));
        let other = XSegment::new(IntPoint::new(-2, 2), IntPoint::new(2, 0));

        let result = ScanCrossSolver::scan_cross(&target, &other).unwrap();

        if let PureRound(point) = result {
            assert_eq!(point, IntPoint::new(0, 1));
        }
    }

    #[test]
    fn test_3() {
        let target = XSegment::new(IntPoint::new(0, 10), IntPoint::new(1, 0));
        let other = XSegment::new(IntPoint::new(-2, 2), IntPoint::new(2, 0));

        let result = ScanCrossSolver::scan_cross(&target, &other).unwrap();

        if let PureRound(point) = result {
            assert_eq!(point, IntPoint::new(0, 1));
        }
    }
}