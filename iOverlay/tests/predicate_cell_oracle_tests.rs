use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::ShapeType;
use i_overlay::core::relate::PredicateOverlay;
use i_overlay::core::solver::Solver;
use std::collections::BTreeSet;

fn filled(count: i32, rule: FillRule) -> bool {
    match rule {
        FillRule::EvenOdd => count % 2 != 0,
        FillRule::NonZero => count != 0,
        FillRule::Positive => count > 0,
        FillRule::Negative => count < 0,
    }
}

fn closure(cells: &BTreeSet<(i32, i32)>) -> (BTreeSet<(i32, i32)>, BTreeSet<(i32, i32, i32, i32)>) {
    let mut vertices = BTreeSet::new();
    let mut edges = BTreeSet::new();
    for &(x, y) in cells {
        vertices.extend([(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]);
        edges.extend([
            (x, y, x + 1, y),
            (x, y + 1, x + 1, y + 1),
            (x, y, x, y + 1),
            (x + 1, y, x + 1, y + 1),
        ]);
    }
    (vertices, edges)
}

#[test]
fn predicates_match_filled_cells_after_contour_cancellation() {
    let mut seed = 0x3587_89ad_182a_4b12_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (seed >> 32) as usize
    };
    for case in 0..1200 {
        let mut contours = [Vec::new(), Vec::new()];
        let mut counts = [[0_i32; 64]; 2];
        for side in 0..2 {
            for _ in 0..1 + next() % 12 {
                let x = next() % 8;
                let y = next() % 8;
                let x1 = x + 1 + next() % (8 - x);
                let y1 = y + 1 + next() % (8 - y);
                let sign = if next() % 2 == 0 { 1 } else { -1 };
                let mut path = vec![
                    IntPoint::new(x as i32, y as i32),
                    IntPoint::new(x1 as i32, y as i32),
                    IntPoint::new(x1 as i32, y1 as i32),
                    IntPoint::new(x as i32, y1 as i32),
                ];
                if sign < 0 {
                    path.reverse();
                }
                let copies = 1 + next() % 3;
                for _ in 0..copies {
                    contours[side].push(path.clone());
                }
                for yy in y..y1 {
                    for xx in x..x1 {
                        counts[side][8 * yy + xx] += sign * copies as i32;
                    }
                }
            }
        }
        let mut overlay = PredicateOverlay::new(0);
        overlay.solver = [Solver::LIST, Solver::TREE, Solver::FRAG][case % 3];
        overlay.add_source(&contours[0], ShapeType::Subject);
        overlay.add_source(&contours[1], ShapeType::Clip);
        for rule in [
            FillRule::EvenOdd,
            FillRule::NonZero,
            FillRule::Positive,
            FillRule::Negative,
            FillRule::EvenOdd,
        ] {
            overlay.fill_rule = rule;
            let cells: Vec<BTreeSet<_>> = counts
                .iter()
                .map(|c| {
                    c.iter()
                        .enumerate()
                        .filter(|(_, n)| filled(**n, rule))
                        .map(|(i, _)| ((i % 8) as i32, (i / 8) as i32))
                        .collect()
                })
                .collect();
            let (av, ae) = closure(&cells[0]);
            let (bv, be) = closure(&cells[1]);
            let interior = !cells[0].is_disjoint(&cells[1]);
            let intersects = !av.is_disjoint(&bv);
            let point = intersects && !interior && ae.is_disjoint(&be);
            let within = !cells[0].is_empty() && cells[0].is_subset(&cells[1]);
            let expected = (intersects, interior, intersects && !interior, point, within);
            for repeat in 0..2 {
                let actual = (
                    overlay.intersects(),
                    overlay.interiors_intersect(),
                    overlay.touches(),
                    overlay.point_intersects(),
                    overlay.within(),
                );
                assert_eq!(
                    actual, expected,
                    "case={case}, rule={rule:?}, repeat={repeat}, contours={contours:?}"
                );
            }
        }
    }
}
