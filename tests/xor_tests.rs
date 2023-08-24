use i_overlay::bool::fill_rule::FillRule;
use i_overlay::layout::overlay::Overlay;


#[cfg(test)]
mod tests {
    use i_float::fix_vec::FixVec;
    use i_overlay::fill::shape_type::ShapeType;

    use super::*;

    #[test]
    fn test_0() {
        let mut overlay = Overlay::new(1);
        
        let subj = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10,  10),
            FixVec::new_number( 10,  10),
            FixVec::new_number( 10, -10)
        ];

        let clip = [
            FixVec::new_number(-5, -5),
            FixVec::new_number(-5, 15),
            FixVec::new_number(15, 15),
            FixVec::new_number(15, -5)
        ];


        overlay.add_path(subj.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(clip.to_vec(), ShapeType::CLIP);
        let graph = overlay.build_graph();

        let shapes = graph.extract_shapes(FillRule::Xor);

        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].count(), 1);
        assert_eq!(shapes[1].count(), 1);

        let path0 = [
            FixVec::new_i64(-10240, -10240),
            FixVec::new_i64(-10240, 10240),
            FixVec::new_i64(-5120, 10240),
            FixVec::new_i64(-5120, -5120),
            FixVec::new_i64(10240, -5120),
            FixVec::new_i64(10240, -10240)
        ];

        assert_eq!(shapes[0].paths()[0].as_slice(), path0.as_ref());

        let path1 = [
            FixVec::new_i64(-5120, 10240),
            FixVec::new_i64(-5120, 15360),
            FixVec::new_i64(15360, 15360),
            FixVec::new_i64(15360, -5120),
            FixVec::new_i64(10240, -5120),
            FixVec::new_i64(10240, 10240)
        ];

        assert_eq!(shapes[1].paths()[0].as_slice(), path1.as_ref());
    }

    #[test]
    fn test_1() {
        let mut overlay = Overlay::new(1);
        
        let subj0 = [
            FixVec::new_number(-20, -16),
            FixVec::new_number(-20,  16),
            FixVec::new_number(20,  16),
            FixVec::new_number(20, -16)
        ];

        let subj1 = [
            FixVec::new_number(-12, -8),
            FixVec::new_number(-12,  8),
            FixVec::new_number(12,  8),
            FixVec::new_number(12, -8)
        ];


        let clip = [
            FixVec::new_number(-4, -24),
            FixVec::new_number(-4,  24),
            FixVec::new_number(4,  24),
            FixVec::new_number(4, -24)
        ];


        overlay.add_path(subj0.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(subj1.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(clip.to_vec(), ShapeType::CLIP);
        let graph = overlay.build_graph();

        let shapes = graph.extract_shapes(FillRule::Xor);

        assert_eq!(shapes.len(), 5);
        assert_eq!(shapes[0].count(), 1);
        assert_eq!(shapes[1].count(), 1);
        assert_eq!(shapes[2].count(), 1);
        assert_eq!(shapes[3].count(), 1);
        assert_eq!(shapes[4].count(), 1);

        let path0 = [
            FixVec::new_i64(-20480, -16384),
            FixVec::new_i64(-20480, 16384),
            FixVec::new_i64(-4096, 16384),
            FixVec::new_i64(-4096, 8192),
            FixVec::new_i64(-12288, 8192),
            FixVec::new_i64(-12288, -8192),
            FixVec::new_i64(-4096, -8192),
            FixVec::new_i64(-4096, -16384)
        ];

        assert_eq!(shapes[0].contour().as_slice(), path0.as_ref());

        let path1 = [
            FixVec::new_i64(-4096, -24576),
            FixVec::new_i64(-4096, -16384),
            FixVec::new_i64(4096, -16384),
            FixVec::new_i64(4096, -24576)
        ];

        assert_eq!(shapes[1].contour().as_slice(), path1.as_ref());

        let path2 = [
            FixVec::new_i64(-4096, -8192),
            FixVec::new_i64(-4096, 8192),
            FixVec::new_i64(4096, 8192),
            FixVec::new_i64(4096, -8192)
        ];

        assert_eq!(shapes[2].contour().as_slice(), path2.as_ref());

        let path3 = [
            FixVec::new_i64(-4096, 16384),
            FixVec::new_i64(-4096, 24576),
            FixVec::new_i64(4096, 24576),
            FixVec::new_i64(4096, 16384)
        ];

        assert_eq!(shapes[3].contour().as_slice(), path3.as_ref());

        let path4 = [
            FixVec::new_i64(4096, -16384),
            FixVec::new_i64(4096, -8192),
            FixVec::new_i64(12288, -8192),
            FixVec::new_i64(12288, 8192),
            FixVec::new_i64(4096, 8192),
            FixVec::new_i64(4096, 16384),
            FixVec::new_i64(20480, 16384),
            FixVec::new_i64(20480, -16384)
        ];

        assert_eq!(shapes[4].contour().as_slice(), path4.as_ref());
    }

    #[test]
    fn test_2() {
        let mut overlay = Overlay::new(1);
        
        let subj0 = [
            FixVec::new_number(-30, -30),
            FixVec::new_number(-30, 30),
            FixVec::new_number(30, 30),
            FixVec::new_number(30, -30)
        ];

        let subj1 = [
            FixVec::new_number(-20, -20),
            FixVec::new_number(-20, 20),
            FixVec::new_number(20, 20),
            FixVec::new_number(20, -20)
        ];


        let clip = [
            FixVec::new_number(-5, -5),
            FixVec::new_number(-5,  5),
            FixVec::new_number( 5,  5),
            FixVec::new_number( 5, -5)
        ];


        overlay.add_path(subj0.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(subj1.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(clip.to_vec(), ShapeType::CLIP);
        let graph = overlay.build_graph();

        let shapes = graph.extract_shapes(FillRule::Xor);

        assert_eq!(shapes.len(), 2);

        assert_eq!(shapes[0].count(), 2);
        assert_eq!(shapes[1].count(), 1);

        let path0 = [
            FixVec::new_i64(-30720, -30720),
            FixVec::new_i64(-30720, 30720),
            FixVec::new_i64(30720, 30720),
            FixVec::new_i64(30720, -30720)
        ];

        assert_eq!(shapes[0].contour().as_slice(), path0.as_ref());

        let path1 = [
            FixVec::new_i64(-20480, -20480),
            FixVec::new_i64(-20480, 20480),
            FixVec::new_i64(20480, 20480),
            FixVec::new_i64(20480, -20480)
        ];

        assert_eq!(shapes[0].paths()[1].as_slice(), path1.as_ref());

        let path2 = [
            FixVec::new_i64(-5120, -5120),
            FixVec::new_i64(-5120, 5120),
            FixVec::new_i64(5120, 5120),
            FixVec::new_i64(5120, -5120)
        ];

        assert_eq!(shapes[1].contour().as_slice(), path2.as_ref());

    }

    #[test]
    fn test_3() {
        let mut overlay = Overlay::new(1);
        
        let subj0 = [
            FixVec::new_number(-20, -20),
            FixVec::new_number(-20, 20),
            FixVec::new_number(20, 20),
            FixVec::new_number(20, -20)
        ];

        let subj1 = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 0),
            FixVec::new_number(0, 0),
            FixVec::new_number(0, -10)
        ];

        let subj2 = [
            FixVec::new_number(0, -10),
            FixVec::new_number(0, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, -10)
        ];

        let clip = [
            FixVec::new_number(-5, -5),
            FixVec::new_number(-5, 5),
            FixVec::new_number(5, 5),
            FixVec::new_number(5, -5)
        ];

        overlay.add_path(subj0.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(subj1.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(subj2.to_vec(), ShapeType::SUBJECT);
        overlay.add_path(clip.to_vec(), ShapeType::CLIP);
        let graph = overlay.build_graph();

        let shapes = graph.extract_shapes(FillRule::Xor);

        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].count(), 2);
        assert_eq!(shapes[1].count(), 1);

        let path0 = [
            FixVec::new_i64(-20480, -20480),
            FixVec::new_i64(-20480, 20480),
            FixVec::new_i64(20480, 20480),
            FixVec::new_i64(20480, -20480)
        ];

        assert_eq!(shapes[0].contour().as_slice(), path0.as_ref());

        let path1 = [
            FixVec::new_i64(-10240, -10240),
            FixVec::new_i64(-10240, 0),
            FixVec::new_i64(-5120, 0),
            FixVec::new_i64(-5120, 5120),
            FixVec::new_i64(0, 5120),
            FixVec::new_i64(0, 10240),
            FixVec::new_i64(10240, 10240),
            FixVec::new_i64(10240, -10240)
        ];

        assert_eq!(shapes[0].paths()[1].as_slice(), path1.as_ref());

        let path2 = [
            FixVec::new_i64(-5120, -5120),
            FixVec::new_i64(-5120, 0),
            FixVec::new_i64(0, 0),
            FixVec::new_i64(0, 5120),
            FixVec::new_i64(5120, 5120),
            FixVec::new_i64(5120, -5120)
        ];

        assert_eq!(shapes[1].contour().as_slice(), path2.as_ref());
    }
}
