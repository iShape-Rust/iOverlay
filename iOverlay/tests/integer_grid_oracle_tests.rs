use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;

#[derive(Debug)]
struct Rect {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    sign: i32,
}

impl Rect {
    fn contour(&self) -> Vec<IntPoint> {
        let mut result = vec![
            IntPoint::new(2 * self.x0, 2 * self.y0),
            IntPoint::new(2 * self.x1, 2 * self.y0),
            IntPoint::new(2 * self.x1, 2 * self.y1),
            IntPoint::new(2 * self.x0, 2 * self.y1),
        ];
        if self.sign < 0 {
            result.reverse();
        }
        result
    }
}

fn fill(rects: &[Rect], x: i32, y: i32, rule: FillRule) -> bool {
    let count: i32 = rects
        .iter()
        .filter(|r| r.x0 <= x && x < r.x1 && r.y0 <= y && y < r.y1)
        .map(|r| r.sign)
        .sum();
    match rule {
        FillRule::EvenOdd => count % 2 != 0,
        FillRule::NonZero => count != 0,
        FillRule::Positive => count > 0,
        FillRule::Negative => count < 0,
    }
}

fn contains(shape: &[Vec<IntPoint>], x: i32, y: i32) -> bool {
    let mut inside = false;
    for path in shape {
        let mut a = path[path.len() - 1];
        for &b in path {
            assert!(
                a.x == b.x || a.y == b.y,
                "rectangle overlays must remain orthogonal"
            );
            if (a.y > y) != (b.y > y) && a.x > x {
                inside = !inside;
            }
            a = b;
        }
    }
    inside
}

#[test]
fn rectangle_overlays_match_independent_cell_winding() {
    let mut state = 0x7114_58bb_aa16_5841_u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (state >> 32) as i32 & 0x7fff_ffff
    };
    for case in 0..1000 {
        let mut rects = |n| {
            (0..n)
                .map(|_| {
                    let x0 = next() % 8;
                    let y0 = next() % 8;
                    Rect {
                        x0,
                        y0,
                        x1: x0 + 1 + next() % (8 - x0),
                        y1: y0 + 1 + next() % (8 - y0),
                        sign: if next() % 2 == 0 { 1 } else { -1 },
                    }
                })
                .collect::<Vec<_>>()
        };
        let a = rects(1 + case % 7);
        let b = rects(1 + case % 5);
        let mut overlay = Overlay::with_contours(
            &a.iter().map(Rect::contour).collect::<Vec<_>>(),
            &b.iter().map(Rect::contour).collect::<Vec<_>>(),
        );
        overlay.solver = [Solver::LIST, Solver::TREE, Solver::FRAG][case % 3];
        overlay.options.ogc = case % 2 == 0;
        let clockwise = case % 4 < 2;
        overlay.options.output_direction = if clockwise {
            ContourDirection::Clockwise
        } else {
            ContourDirection::CounterClockwise
        };
        for fill_rule in [
            FillRule::EvenOdd,
            FillRule::NonZero,
            FillRule::Positive,
            FillRule::Negative,
        ] {
            for rule in [
                OverlayRule::Union,
                OverlayRule::Intersect,
                OverlayRule::Difference,
                OverlayRule::Xor,
            ] {
                let result = overlay.overlay(rule, fill_rule);
                let mut cells = 0;
                for x in 0..8 {
                    for y in 0..8 {
                        let sa = fill(&a, x, y, fill_rule);
                        let sb = fill(&b, x, y, fill_rule);
                        let expected = match rule {
                            OverlayRule::Union => sa || sb,
                            OverlayRule::Intersect => sa && sb,
                            OverlayRule::Difference => sa && !sb,
                            OverlayRule::Xor => sa != sb,
                            _ => unreachable!(),
                        };
                        cells += i64::from(expected);
                        let covered = result
                            .iter()
                            .filter(|s| contains(s, 2 * x + 1, 2 * y + 1))
                            .count();
                        assert_eq!(
                            covered,
                            usize::from(expected),
                            "case={case}, cell=({x},{y}), fill={fill_rule:?}, rule={rule:?}, a={a:?}, b={b:?}"
                        );
                    }
                }
                let mut double_area = 0_i64;
                for shape in &result {
                    for (index, path) in shape.iter().enumerate() {
                        let area: i64 = path
                            .iter()
                            .zip(path.iter().cycle().skip(1))
                            .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
                            .sum();
                        assert_eq!(
                            area > 0,
                            (index == 0) != clockwise,
                            "case={case}, ogc={}, clockwise={clockwise}, fill={fill_rule:?}, rule={rule:?}, index={index}, path={path:?}, a={a:?}, b={b:?}",
                            overlay.options.ogc
                        );
                        double_area += area;
                        if overlay.options.ogc {
                            let unique: std::collections::BTreeSet<_> = path.iter().collect();
                            assert_eq!(
                                unique.len(),
                                path.len(),
                                "OGC contour repeats a vertex: case={case}, path={path:?}"
                            );
                        }
                    }
                }
                assert_eq!(
                    double_area.abs(),
                    cells * 8,
                    "case={case}, fill={fill_rule:?}, rule={rule:?}"
                );
            }
        }
    }
}
