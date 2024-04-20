#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_shape::int::shape::IntShape;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::simplify::Simplify;

    #[test]
    fn test_0() {
        let paths =
            [
                [
                    IntPoint::new(10614, 4421),
                    IntPoint::new(10609, 4421),
                    IntPoint::new(10609, 4415),
                    IntPoint::new(10614, 4415)
                ].to_vec()
            ].to_vec();

        let simplified = paths.simplify(FillRule::NonZero);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].len(), 1);
    }

    #[test]
    fn test_1() {
        let paths =
            [
                square(IntPoint::new(-10, -10)),
                square(IntPoint::new(-10, 0)),
                square(IntPoint::new(-10, 10)),
                square(IntPoint::new(0, -10)),
                square(IntPoint::new(0, 10)),
                square(IntPoint::new(10, -10)),
                square(IntPoint::new(10, 0)),
                square(IntPoint::new(10, 10))
            ].to_vec();

        let simplified = paths.simplify(FillRule::NonZero);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].len(), 2);
    }

    #[test]
    fn test_2() {
        let shapes =
            [
                square_shape(IntPoint::new(-10, -10)),
                square_shape(IntPoint::new(-10, 0)),
                square_shape(IntPoint::new(-10, 10)),
                square_shape(IntPoint::new(0, -10)),
                square_shape(IntPoint::new(0, 10)),
                square_shape(IntPoint::new(10, -10)),
                square_shape(IntPoint::new(10, 0)),
                square_shape(IntPoint::new(10, 10))
            ].to_vec();

        let simplified = shapes.simplify(FillRule::NonZero);

        assert_eq!(simplified.len(), 1);
        assert_eq!(simplified[0].len(), 2);
    }

    fn square(pos: IntPoint) -> Vec<IntPoint> {
        [
            IntPoint::new(-5 + pos.x, -5 + pos.y),
            IntPoint::new(-5 + pos.x, 5 + pos.y),
            IntPoint::new(5 + pos.x, 5 + pos.y),
            IntPoint::new(5 + pos.x, -5 + pos.y)
        ].to_vec()
    }

    fn square_shape(pos: IntPoint) -> IntShape {
        [square(pos)].to_vec()
    }
}