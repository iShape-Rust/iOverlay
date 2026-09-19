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
                let mut overlay = StringOverlay::from_shape(&subject);
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

#[test]
fn clipping_overlapping_contours_uses_the_resolved_boundary() {
    let mut seed = 0x173b_d902_aa9f_4ee1_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (seed >> 32) as u32
    };
    let mut lines = Vec::new();
    for k in -1..=9 {
        for line in [
            [IntPoint::new(-1, k), IntPoint::new(9, k)],
            [IntPoint::new(k, -1), IntPoint::new(k, 9)],
        ] {
            lines.push(line);
            lines.push([line[1], line[0]]);
        }
    }
    let input = unit_edges(&lines.iter().map(|p| p.to_vec()).collect::<Vec<_>>());
    for case in 0..300 {
        let mut contours = Vec::new();
        let mut winding = [[0_i32; 8]; 8];
        for _ in 0..1 + case % 8 {
            let (x0, y0) = ((next() % 8) as usize, (next() % 8) as usize);
            let (x1, y1) = (
                x0 + 1 + next() as usize % (8 - x0),
                y0 + 1 + next() as usize % (8 - y0),
            );
            let sign = if next() % 2 == 0 { 1 } else { -1 };
            for column in &mut winding[x0..x1] {
                for count in &mut column[y0..y1] {
                    *count += sign;
                }
            }
            let mut contour = vec![
                IntPoint::new(x0 as i32, y0 as i32),
                IntPoint::new(x1 as i32, y0 as i32),
                IntPoint::new(x1 as i32, y1 as i32),
                IntPoint::new(x0 as i32, y1 as i32),
            ];
            if sign < 0 {
                contour.reverse();
            }
            contours.push(contour);
        }
        for fill in [
            FillRule::EvenOdd,
            FillRule::NonZero,
            FillRule::Positive,
            FillRule::Negative,
        ] {
            let filled = |x: i32, y: i32| {
                let count = if (0..8).contains(&x) && (0..8).contains(&y) {
                    winding[x as usize][y as usize]
                } else {
                    0
                };
                match fill {
                    FillRule::EvenOdd => count % 2 != 0,
                    FillRule::NonZero => count != 0,
                    FillRule::Positive => count > 0,
                    FillRule::Negative => count < 0,
                }
            };
            for boundary_included in [false, true] {
                for invert in [false, true] {
                    let expected: BTreeSet<_> = input
                        .iter()
                        .copied()
                        .filter(|&(ax, ay, bx, by)| {
                            let (x, y) = (ax.min(bx), ay.min(by));
                            let (a, b) = if ay == by {
                                (filled(x, y), filled(x, y - 1))
                            } else {
                                (filled(x, y), filled(x - 1, y))
                            };
                            // A boundary separates a filled cell from an empty one.
                            // An input edge with filled cells on both sides is interior.
                            let inside = if boundary_included { a || b } else { a && b };
                            inside != invert
                        })
                        .collect();
                    let mut overlay = StringOverlay::from_shape(&contours);
                    overlay.add_string_lines(&lines);
                    let actual = overlay.clip_string_lines(
                        fill,
                        ClipRule {
                            invert,
                            boundary_included,
                        },
                    );
                    assert_eq!(
                        unit_edges(&actual),
                        expected,
                        "case={case}, fill={fill:?}, invert={invert}, boundary={boundary_included}, contours={contours:?}"
                    );
                }
            }
        }
    }
}
