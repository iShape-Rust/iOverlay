use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::solver::Solver;
use i_overlay::string::clip::ClipRule;
use i_overlay::string::overlay::StringOverlay;
use std::collections::BTreeSet;

type Edge = (i64, i64, i64, i64);
fn edges(paths: &[Vec<IntPoint<i64>>], origin: i64) -> BTreeSet<Edge> {
    let mut result = BTreeSet::new();
    let inverse = |p: IntPoint<i64>| {
        let (x, y) = (p.x - origin, p.y + origin);
        assert_eq!(
            (3 * x + 4 * y) % 25,
            0,
            "off-lattice output {p:?}, origin={origin}"
        );
        assert_eq!(
            (-4 * x + 3 * y) % 25,
            0,
            "off-lattice output {p:?}, origin={origin}"
        );
        ((3 * x + 4 * y) / 25, (-4 * x + 3 * y) / 25)
    };
    for path in paths {
        for pair in path.windows(2) {
            let (mut a, b) = (inverse(pair[0]), inverse(pair[1]));
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            assert!(dx != 0 || dy != 0);
            assert!(dx == 0 || dy == 0 || dx == dy, "unexpected segment {a:?}..{b:?}");
            let step = (dx.signum(), dy.signum());
            while a != b {
                let end = (a.0 + step.0, a.1 + step.1);
                assert!(
                    result.insert((a.0, a.1, end.0, end.1)),
                    "duplicated directed segment"
                );
                a = end;
            }
        }
    }
    result
}

#[test]
fn oblique_clipping_preserves_exact_coverage_at_large_translations() {
    let mut seed = 0x6817_f121_29ae_0912_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 17) as i64 - 3
    };
    for case in 0..200 {
        let mut raw = Vec::new();
        for i in 0..40 {
            let (x, y, t) = (next(), next(), next());
            let a = (x, y);
            let b = match i % 3 {
                0 => (t, y),
                1 => (x, t),
                _ => (x + t, y + t),
            };
            raw.push((a, b));
        }
        for origin in [0, 1_000_000_000, 1_i64 << 60, -(1_i64 << 60)] {
            let transform =
                |(x, y): (i64, i64)| IntPoint::new(origin + 3 * x - 4 * y, -origin + 4 * x + 3 * y);
            let contour: Vec<_> = [(0, 0), (10, 0), (10, 10), (0, 10)]
                .into_iter()
                .map(transform)
                .collect();
            let lines: Vec<_> = raw.iter().map(|&(a, b)| [transform(a), transform(b)]).collect();
            let input: Vec<_> = lines
                .iter()
                .filter(|l| l[0] != l[1])
                .map(|l| l.to_vec())
                .collect();
            // Coincident input edges represent one directed edge in the string overlay.
            let mut all = BTreeSet::new();
            for path in input {
                all.extend(edges(&[path], origin));
            }
            for boundary_included in [false, true] {
                for invert in [false, true] {
                    let expected: BTreeSet<_> = all
                        .iter()
                        .copied()
                        .filter(|&(ax, ay, bx, by)| {
                            let (x, y) = (ax + bx, ay + by);
                            let inside = if boundary_included {
                                (0..=20).contains(&x) && (0..=20).contains(&y)
                            } else {
                                (1..20).contains(&x) && (1..20).contains(&y)
                            };
                            inside != invert
                        })
                        .collect();
                    let mut overlay = StringOverlay::with_shape_contour(&contour);
                    overlay.add_string_lines(&lines);
                    let result = overlay.clip_string_lines_with_solver(
                        FillRule::NonZero,
                        ClipRule {
                            invert,
                            boundary_included,
                        },
                        [Solver::LIST, Solver::TREE, Solver::FRAG][case % 3],
                    );
                    assert_eq!(
                        edges(&result, origin),
                        expected,
                        "case={case}, origin={origin}, invert={invert}, boundary={boundary_included}"
                    );
                }
            }
        }
    }
}
