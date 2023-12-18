#[cfg(test)]
mod tests {
    use i_float::fix_vec::FixVec;
    use i_shape::fix_shape::FixShape;
    use i_overlay::bool::fill_rule::FillRule;
    use i_overlay::ext::simplify::Simplify;

    #[test]
    fn test_0() {

        let paths =
            [
                square(FixVec::new_f64(-10.0, -10.0)),
                square(FixVec::new_f64(-10.0,  0.0)),
                square(FixVec::new_f64(-10.0,  10.0)),
                square(FixVec::new_f64(0.0, -10.0)),
                square(FixVec::new_f64( 0.0,  10.0)),
                square(FixVec::new_f64(10.0, -10.0)),
                square(FixVec::new_f64(10.0,  0.0)),
                square(FixVec::new_f64( 10.0,  10.0))
            ].to_vec();


        let simplified = paths.simplify(FillRule::NonZero);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].paths.len(), 2);
    }

    #[test]
    fn test_1() {

        let shapes =
            [
                square_shape(FixVec::new_f64(-10.0, -10.0)),
                square_shape(FixVec::new_f64(-10.0,  0.0)),
                square_shape(FixVec::new_f64(-10.0,  10.0)),
                square_shape(FixVec::new_f64(0.0, -10.0)),
                square_shape(FixVec::new_f64( 0.0,  10.0)),
                square_shape(FixVec::new_f64(10.0, -10.0)),
                square_shape(FixVec::new_f64(10.0,  0.0)),
                square_shape(FixVec::new_f64( 10.0,  10.0))
            ].to_vec();


        let simplified = shapes.simplify(FillRule::NonZero);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].paths.len(), 2);
    }

    fn square(pos: FixVec) -> Vec<FixVec> {
        [
            FixVec::new_f64(-5.0, -5.0) + pos,
            FixVec::new_f64(-5.0,  5.0) + pos,
            FixVec::new_f64( 5.0,  5.0) + pos,
            FixVec::new_f64( 5.0, -5.0) + pos
        ].to_vec()
    }

    fn square_shape(pos: FixVec) -> FixShape {
        FixShape::new_with_contour(square(pos))
    }

}