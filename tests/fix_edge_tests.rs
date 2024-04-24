#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_overlay::split::cross_solver::{CrossResult, ScanCrossSolver};
    use i_overlay::x_segment::XSegment;

    #[test]
    fn test_simple_cross() {
        let s: i32 = 1024;

        let ea = XSegment::new(IntPoint::new(-s, 0), IntPoint::new(s, 0));
        let eb = XSegment::new(IntPoint::new(0, -s), IntPoint::new(0, s));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::PureExact(point) => {
                assert_eq!(IntPoint::ZERO, point);
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_big_cross_1() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(IntPoint::new(-s, 0), IntPoint::new(s, 0));
        let eb = XSegment::new(IntPoint::new(0, -s), IntPoint::new(0, s));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::PureExact(point) => {
                assert_eq!(IntPoint::ZERO, point);
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_big_cross_2() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(IntPoint::new(-s, 0), IntPoint::new(s, 0));
        let eb = XSegment::new(IntPoint::new(1024, -s), IntPoint::new(1024, s));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::PureExact(point) => {
                assert_eq!(IntPoint::new(1024, 0), point);
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_big_cross_3() {
        let s: i32 = 1024_000_000;
        let q: i32 = s / 2;

        let ea = XSegment::new(IntPoint::new(-s, -s), IntPoint::new(s, s));
        let eb = XSegment::new(IntPoint::new(q, -s), IntPoint::new(q, s));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::PureExact(point) => {
                assert_eq!(IntPoint::new(512_000_000, 512_000_000), point);
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_left_end() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(IntPoint::new(-s, 0), IntPoint::new(s, 0));
        let eb = XSegment::new(IntPoint::new(-s, -s), IntPoint::new(-s, s));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::TargetEndExact(point) => {
                assert_eq!(IntPoint::new(-s, 0), point);
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_right_end() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(IntPoint::new(-s, 0), IntPoint::new(s, 0));
        let eb = XSegment::new(IntPoint::new(s, -s), IntPoint::new(s, s));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::TargetEndExact(point) => {
                assert_eq!(IntPoint::new(s, 0), point);
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_left_top() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(IntPoint::new(-s, s), IntPoint::new(s, s));
        let eb = XSegment::new(IntPoint::new(-s, s), IntPoint::new(-s, -s));

        let result = ScanCrossSolver::scan_cross(&ea, &eb);
        assert!(result.is_none());
    }

    #[test]
    fn test_real_case_1() {
        let ea = XSegment::new(IntPoint::new(7256, -14637), IntPoint::new(7454, -15045));
        let eb = XSegment::new(IntPoint::new(7343, -14833), IntPoint::new(7506, -15144));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::PureRound(_point) => {},
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_real_case_2() {
        let ea = XSegment::new(IntPoint::new(-8555798, -1599355), IntPoint::new(-1024000, 0));
        let eb = XSegment::new(IntPoint::new(-8571363, 1513719), IntPoint::new(-1023948, -10239));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::PureRound(point) => {
                assert_eq!(IntPoint::new(-1048691, -5244), point);
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_real_case_3() {
        let ea = XSegment::new(IntPoint::new(-8555798, -1599355), IntPoint::new(513224, -5243));
        let eb = XSegment::new(IntPoint::new(-8555798, -1599355), IntPoint::new(513224, -5243));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::EndOverlap => {},
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_real_case_4() {
        let ea = XSegment::new(
            IntPoint::new(-276659431, 380789039),
            IntPoint::new(-221915258, 435533212)
        );
        let eb = XSegment::new(
            IntPoint::new(-276659432, 380789038),
            IntPoint::new(-276659430, 380789040)
        );

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::Overlap => {

            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }

    #[test]
    fn test_penetration() {
        let s: i32 = 1024;

        let ea = XSegment::new(IntPoint::new(-s, 0), IntPoint::new(s / 2, 0));
        let eb = XSegment::new(IntPoint::new(0, 0), IntPoint::new(s, 0));

        let result = ScanCrossSolver::scan_cross(&ea, &eb).unwrap();

        match result {
            CrossResult::Overlap => {
            },
            _ => {
                panic!("Fail cross result");
            },
        }
    }
}