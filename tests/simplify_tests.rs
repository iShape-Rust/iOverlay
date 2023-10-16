
#[cfg(test)]
mod tests {
    use i_float::fix_vec::FixVec;
    use i_overlay::ext::simplify::Simplify;

    #[test]
    fn test_0() {
        let path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, -10),
            FixVec::new_number(-10, -10),
            FixVec::new_number(-5, -5),
            FixVec::new_number(-5,  5),
            FixVec::new_number( 5,  5),
            FixVec::new_number( 5, -5),
            FixVec::new_number(-5, -5)
        ].to_vec();

        let shapes = path.simplify();
        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.paths().len(), 2);

        let contour = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, -10)
        ];

        let hole = [
            FixVec::new_number(-5, -5),
            FixVec::new_number( -5, 5),
            FixVec::new_number(5, 5),
            FixVec::new_number(5, -5)
        ];

        assert_eq!(shape.contour().as_slice(), contour);
        assert_eq!(shape.holes()[0].as_slice(), hole);
    }

    #[test]
    fn test_1() {
        let path = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, -10),
            FixVec::new_number(-10, -10),
            FixVec::new_number(-5, -5),
            FixVec::new_number( 5, -5),
            FixVec::new_number( 5,  5),
            FixVec::new_number( -5, 5),
            FixVec::new_number(-5, -5)
        ].to_vec();

        let shapes = path.simplify();
        assert_eq!(shapes.len(), 1);

        let shape = &shapes[0];

        assert_eq!(shape.paths().len(), 2);

        let contour = [
            FixVec::new_number(-10, -10),
            FixVec::new_number(-10, 10),
            FixVec::new_number(10, 10),
            FixVec::new_number(10, -10)
        ];

        let hole = [
            FixVec::new_number(-5, -5),
            FixVec::new_number( -5, 5),
            FixVec::new_number(5, 5),
            FixVec::new_number(5, -5)
        ];

        assert_eq!(shape.contour().as_slice(), contour);
        assert_eq!(shape.holes()[0].as_slice(), hole);
    }

    #[test]
    fn test_2() {
        let path = [
            FixVec::new_number(-15, -5),
            FixVec::new_number(-15, 5),
            FixVec::new_number(-5, 5),
            FixVec::new_number(-5, 0),
            FixVec::new_number(5, 0),
            FixVec::new_number(5, 5),
            FixVec::new_number(15, 5),
            FixVec::new_number(15,  -5),
            FixVec::new_number(5, -5),
            FixVec::new_number(5, 0),
            FixVec::new_number(-5, 0),
            FixVec::new_number(-5, -5),
        ].to_vec();

        let shapes = path.simplify();
        assert_eq!(shapes.len(), 2);

        let shape0 = &shapes[0];
        let shape1 = &shapes[1];

        assert_eq!(shape0.paths().len(), 1);
        assert_eq!(shape1.paths().len(), 1);

        let contour0 = [
            FixVec::new_number(-15, -5),
            FixVec::new_number(-15, 5),
            FixVec::new_number(-5, 5),
            FixVec::new_number(-5, -5)
        ];

        let contour1 = [
            FixVec::new_number(5, -5),
            FixVec::new_number(5, 5),
            FixVec::new_number(15, 5),
            FixVec::new_number(15, -5)
        ];

        assert_eq!(shape0.contour().as_slice(), contour0);
        assert_eq!(shape1.contour().as_slice(), contour1);
    }
}