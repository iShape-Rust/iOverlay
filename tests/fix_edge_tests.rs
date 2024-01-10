use i_float::fix_vec::FixVec;

#[cfg(test)]
mod tests {
    use i_overlay::split::shape_count::ShapeCount;
    use i_overlay::split::shape_edge::ShapeEdge;
    use i_overlay::split::shape_edge_cross::EdgeCrossType;
    use super::*;
    
    #[test]
    fn test_simple_cross() {
        let s: i64 = 1024;
        
        let ea = ShapeEdge::new(FixVec::new(-s, 0), FixVec::new(s, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(0, -s), FixVec::new(0, s), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::ZERO, result.point);
    }

    #[test]
    fn test_big_cross_1() {
        let s: i64 = 1024_000_000;
        
        let ea = ShapeEdge::new(FixVec::new(-s, 0), FixVec::new(s, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(0, -s), FixVec::new(0, s), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::ZERO, result.point);
    }

    #[test]
    fn test_big_cross_2() {
        let s: i64 = 1024_000_000;
        
        let ea = ShapeEdge::new(FixVec::new(-s, 0), FixVec::new(s, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(1024, -s), FixVec::new(1024, s), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::new(1024, 0), result.point);
    }

    #[test]
    fn test_big_cross_3() {
        let s: i64 = 1024_000_000;
        let q: i64 = s / 2;
        
        let ea = ShapeEdge::new(FixVec::new(-s, -s), FixVec::new(s, s), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(q, -s), FixVec::new(q, s), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::new(512_000_000, 512_000_000), result.point);
    }

    #[test]
    fn test_left_end() {
        let s: i64 = 1024_000_000;
        
        let ea = ShapeEdge::new(FixVec::new(-s, 0), FixVec::new(s, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(-s, -s), FixVec::new(-s, s), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::EndA, result.nature);
        assert_eq!(FixVec::new(-s, 0), result.point);
    }

    #[test]
    fn test_right_end() {
        let s: i64 = 1024_000_000;
        
        let ea = ShapeEdge::new(FixVec::new(-s, 0), FixVec::new(s, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(s, -s), FixVec::new(s, s), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::EndA, result.nature);
        assert_eq!(FixVec::new(s, 0), result.point);
    }

    #[test]
    fn test_left_top() {
        let s: i64 = 1024_000_000;
        
        let ea = ShapeEdge::new(FixVec::new(-s, s), FixVec::new(s, s), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(-s, s), FixVec::new(-s, -s), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb);
        assert!(result.is_none());
    }

    #[test]
    fn test_real_case_1() {
        let ea = ShapeEdge::new(FixVec::new(7256, -14637), FixVec::new(7454, -15045), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(7343, -14833), FixVec::new(7506, -15144), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert!(ea.is_box_contain_point(result.point));
        assert!(eb.is_box_contain_point(result.point));
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
    }

    #[test]
    fn test_real_case_2() {
        let ea = ShapeEdge::new(FixVec::new(-8555798, -1599355), FixVec::new(-1024000, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(-8571363, 1513719), FixVec::new(-1023948, -10239), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::Pure, result.nature);
        assert_eq!(FixVec::new(-1048691, -5244), result.point);
    }

    #[test]
    fn test_real_case_3() {
        let ea = ShapeEdge::new(FixVec::new(-8555798, -1599355), FixVec::new(513224, -5243), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(-8555798, -1599355), FixVec::new(513224, -5243), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb);

        assert!(result.is_none());
    }

    #[test]
    fn test_penetration() {
        let s: i64 = 1024;
        
        let ea = ShapeEdge::new(FixVec::new(-s, 0), FixVec::new(s / 2, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(0, 0), FixVec::new(s, 0), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::Penetrate, result.nature);
        assert_eq!(FixVec::ZERO, result.point);
        assert_eq!(FixVec::new(512, 0), result.second);
    }

    #[test]
    fn test_full_overlay() {
        let ea = ShapeEdge::new(FixVec::new(-2, 0), FixVec::new(2, 0), ShapeCount::new(0, 0));
        let eb = ShapeEdge::new(FixVec::new(-1, 0), FixVec::new(1, 0), ShapeCount::new(0, 0));
        
        let result = ea.cross(&eb).unwrap();
        
        assert_eq!(EdgeCrossType::OverlayB, result.nature);
    }
    

}