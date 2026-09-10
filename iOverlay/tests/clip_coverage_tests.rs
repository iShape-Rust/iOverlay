use std::collections::BTreeSet;

use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::string::clip::ClipRule;
use i_overlay::string::overlay::StringOverlay;

type Edge = (i32, i32, i32, i32);

fn unit_edges(paths: &[Vec<IntPoint>]) -> BTreeSet<Edge> {
    let mut edges = BTreeSet::new();
    for path in paths {
        for pair in path.windows(2) {
            let [a, b] = [pair[0], pair[1]];
            assert!(a != b && (a.x == b.x || a.y == b.y));
            let (dx, dy) = ((b.x - a.x).signum(), (b.y - a.y).signum());
            let (mut x, mut y) = (a.x, a.y);
            while (x, y) != (b.x, b.y) {
                assert!(edges.insert((x, y, x + dx, y + dy)), "duplicate directed edge");
                x += dx;
                y += dy;
            }
        }
    }
    edges
}

#[test]
fn grid_clipping_preserves_directed_coverage() {
    let subject = [
        IntPoint::new(0, 0),
        IntPoint::new(10, 0),
        IntPoint::new(10, 10),
        IntPoint::new(0, 10),
    ];
    let mut seed = 0x7189_d91a_c29b_13f5_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 15) as i32 - 2
    };
    for case in 0..1000 {
        let mut lines = Vec::new();
        let mut input = BTreeSet::new();
        for index in 0..40 {
            let (x, y, t) = (next(), next(), next());
            let a = IntPoint::new(x, y);
            let b = if index % 2 == 0 {
                IntPoint::new(t, y)
            } else {
                IntPoint::new(x, t)
            };
            lines.push([a, b]);
            if a != b {
                input.extend(unit_edges(&[vec![a, b]]));
            }
        }
        for boundary_included in [false, true] {
            for invert in [false, true] {
                // Doubled midpoints classify unit edges without floating point
                // or dependence on the clipping implementation.
                let expected: BTreeSet<_> = input
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
                let mut overlay = StringOverlay::with_shape_contour(&subject);
                overlay.add_string_lines(&lines);
                let result = overlay.clip_string_lines(
                    FillRule::NonZero,
                    ClipRule {
                        invert,
                        boundary_included,
                    },
                );
                assert_eq!(
                    unit_edges(&result),
                    expected,
                    "case={case}, invert={invert}, boundary={boundary_included}, lines={lines:?}, result={result:?}"
                );
            }
        }
    }
}
