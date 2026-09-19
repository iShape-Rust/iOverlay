use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::point_location::IntPointContainment;
use i_overlay::core::solver::Solver;

type Shapes = Vec<Vec<Vec<IntPoint>>>;
fn cross(a: IntPoint, b: IntPoint, c: IntPoint) -> i64 {
    (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64
}
fn oracle(shapes: &Shapes, p: IntPoint) -> Option<bool> {
    let mut result = false;
    for shape in shapes {
        let mut inside = false;
        for path in shape {
            for (&a, &b) in path.iter().zip(path.iter().cycle().skip(1)) {
                let c = cross(a, b, p);
                if c == 0
                    && p.x >= a.x.min(b.x)
                    && p.x <= a.x.max(b.x)
                    && p.y >= a.y.min(b.y)
                    && p.y <= a.y.max(b.y)
                {
                    return None;
                }
                if (a.y > p.y) != (b.y > p.y) && ((c > 0) == (b.y > a.y)) {
                    inside = !inside;
                }
            }
        }
        result |= inside;
    }
    Some(result)
}

#[test]
fn dense_boolean_output_has_resolved_edges_and_correct_point_locations() {
    let mut seed = 0x1792_8721_8eda_38fe_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 101) as i32 - 50
    };
    for case in 0..240 {
        let a: Vec<_> = (0..30 + case % 70)
            .map(|_| IntPoint::new(next(), next()))
            .collect();
        let b: Vec<_> = (0..20 + case % 40)
            .map(|_| IntPoint::new(next(), next()))
            .collect();
        let mut overlay = Overlay::from_subj_and_clip(&a, &b);
        overlay.solver = [Solver::LIST, Solver::TREE, Solver::FRAG][case % 3];
        overlay.options.ogc = true;
        let rule = [
            OverlayRule::Union,
            OverlayRule::Intersect,
            OverlayRule::Difference,
            OverlayRule::Xor,
        ][case % 4];
        let fill = [
            FillRule::EvenOdd,
            FillRule::NonZero,
            FillRule::Positive,
            FillRule::Negative,
        ][(case / 4) % 4];
        let shapes = overlay.overlay(rule, fill);
        let edges: Vec<_> = shapes
            .iter()
            .flatten()
            .flat_map(|p| p.iter().copied().zip(p.iter().copied().cycle().skip(1)))
            .collect();
        for (i, &(a, b)) in edges.iter().enumerate() {
            assert_ne!(a, b, "zero length output edge, case={case}");
            for &(c, d) in &edges[i + 1..] {
                let proper = cross(a, b, c).signum() * cross(a, b, d).signum() < 0
                    && cross(c, d, a).signum() * cross(c, d, b).signum() < 0;
                assert!(
                    !proper,
                    "unresolved crossing, case={case}, edges={a:?}..{b:?}, {c:?}..{d:?}"
                );
            }
        }
        let mut queries = Vec::new();
        let mut expected = Vec::new();
        for y in (-52..=52).step_by(3) {
            for x in (-52..=52).step_by(3) {
                let p = IntPoint::new(x, y);
                if let Some(inside) = oracle(&shapes, p) {
                    queries.push(p);
                    expected.push(inside);
                }
            }
        }
        let actual = shapes.contains_points(&queries);
        if let Some(i) = actual.iter().zip(&expected).position(|(a, b)| a != b) {
            panic!(
                "case={case}, point={:?}, actual={}, expected={}",
                queries[i], actual[i], expected[i]
            );
        }
    }
}
