#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use i_float::f64_vec::F64Vec;
    use i_shape::fix_shape::FixShape;
    use i_overlay::bool::fill_rule::FillRule;
    use i_overlay::bool::overlay_rule::OverlayRule;
    use i_overlay::layout::overlay::{Overlay, ShapeType};

    #[test]
    fn test_0() {
        let mut r = 0.9;

        while r < 1.2 {
            let mut a = 0.0;
            while a < PI {
                let subj = create_star(1.0, r, 7, a);
                let clip = create_star(1.0, 2.0, 7, 0.0);

                let mut overlay = Overlay::new(2);
                overlay.add_shape(&subj, ShapeType::Subject);
                overlay.add_shape(&clip, ShapeType::Clip);

                let graph = overlay.build_graph(FillRule::NonZero);
                let result = graph.extract_shapes(OverlayRule::Union);
                assert!(result.len() > 0);
                a += 0.001
            }

            println!("{}", r);
            r += 0.01
        }
    }

    fn create_star(r0: f64, r1: f64, count: usize, angle: f64) -> FixShape {
        let da = PI / count as f64;
        let mut a = angle;

        let mut points = Vec::new();

        for _ in 0..count {
            let xr0 = r0 * a.cos();
            let yr0 = r0 * a.sin();

            a += da;

            let xr1 = r1 * a.cos();
            let yr1 = r1 * a.sin();

            a += da;

            points.push(F64Vec::new(xr0, yr0).to_fix());
            points.push(F64Vec::new(xr1, yr1).to_fix());
        }

        FixShape::new_with_contour(points)
    }

}