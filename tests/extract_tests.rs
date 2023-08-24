use i_overlay::bool::fill_rule::FillRule;
use i_overlay::layout::overlay::Overlay;


#[cfg(test)]
mod tests {
    use i_float::fix_vec::FixVec;
    use i_overlay::fill::shape_type::ShapeType;

    use super::*;
    
    #[test]
    fn test_sub() {
        let i: usize = 0;
        let j = i.wrapping_sub(1);

        assert_eq!(j, std::usize::MAX );
    }

    #[test]
    fn test_self_intersect_0() {
        let mut overlay = Overlay::new(1);
        
        let path = [
            FixVec::new_number(0, 0),
            FixVec::new_number(0, 1),
            FixVec::new_number(1, 1),
            FixVec::new_number(1, 0)
        ];


        overlay.add_path(path.to_vec(), ShapeType::SUBJECT);
        let graph = overlay.build_graph();

        let shapes = graph.extract_shapes(FillRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].contour().as_slice(), path.as_ref());
    }
}
