#[cfg(test)]
mod tests {
    use i_float::point::Point;
    use i_overlay::x_segment::XSegment;
    use i_overlay::split::shape_edge_cross::EdgeCrossType;

    #[test]
    fn test_simple_cross() {
        let s: i32 = 1024;

        let ea = XSegment::new(Point::new(-s, 0), Point::new(s, 0));
        let eb = XSegment::new(Point::new(0, -s), Point::new(0, s));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(Point::ZERO, result.point);
    }

    #[test]
    fn test_big_cross_1() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(Point::new(-s, 0), Point::new(s, 0));
        let eb = XSegment::new(Point::new(0, -s), Point::new(0, s));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(Point::ZERO, result.point);
    }

    #[test]
    fn test_big_cross_2() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(Point::new(-s, 0), Point::new(s, 0));
        let eb = XSegment::new(Point::new(1024, -s), Point::new(1024, s));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(Point::new(1024, 0), result.point);
    }

    #[test]
    fn test_big_cross_3() {
        let s: i32 = 1024_000_000;
        let q: i32 = s / 2;

        let ea = XSegment::new(Point::new(-s, -s), Point::new(s, s));
        let eb = XSegment::new(Point::new(q, -s), Point::new(q, s));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(Point::new(512_000_000, 512_000_000), result.point);
    }

    #[test]
    fn test_left_end() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(Point::new(-s, 0), Point::new(s, 0));
        let eb = XSegment::new(Point::new(-s, -s), Point::new(-s, s));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::EndA, result.nature);
        assert_eq!(Point::new(-s, 0), result.point);
    }

    #[test]
    fn test_right_end() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(Point::new(-s, 0), Point::new(s, 0));
        let eb = XSegment::new(Point::new(s, -s), Point::new(s, s));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::EndA, result.nature);
        assert_eq!(Point::new(s, 0), result.point);
    }

    #[test]
    fn test_left_top() {
        let s: i32 = 1024_000_000;

        let ea = XSegment::new(Point::new(-s, s), Point::new(s, s));
        let eb = XSegment::new(Point::new(-s, s), Point::new(-s, -s));

        let result = ea.cross(&eb);
        assert!(result.is_none());
    }

    #[test]
    fn test_real_case_1() {
        let ea = XSegment::new(Point::new(7256, -14637), Point::new(7454, -15045));
        let eb = XSegment::new(Point::new(7343, -14833), Point::new(7506, -15144));

        let result = ea.cross(&eb).unwrap();

        assert!(ea.is_box_contain_point(result.point));
        assert!(eb.is_box_contain_point(result.point));

        assert_eq!(EdgeCrossType::Pure, result.nature);
    }

    #[test]
    fn test_real_case_2() {
        let ea = XSegment::new(Point::new(-8555798, -1599355), Point::new(-1024000, 0));
        let eb = XSegment::new(Point::new(-8571363, 1513719), Point::new(-1023948, -10239));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(Point::new(-1048691, -5244), result.point);
    }

    #[test]
    fn test_real_case_3() {
        let ea = XSegment::new(Point::new(-8555798, -1599355), Point::new(513224, -5243));
        let eb = XSegment::new(Point::new(-8555798, -1599355), Point::new(513224, -5243));

        let result = ea.cross(&eb);

        assert!(result.is_none());
    }

    #[test]
    fn test_penetration() {
        let s: i32 = 1024;

        let ea = XSegment::new(Point::new(-s, 0), Point::new(s / 2, 0));
        let eb = XSegment::new(Point::new(0, 0), Point::new(s, 0));

        let result = ea.cross(&eb).unwrap();

        assert_eq!(EdgeCrossType::Penetrate, result.nature);
        assert_eq!(Point::ZERO, result.point);
        assert_eq!(Point::new(512, 0), result.second);
    }
}