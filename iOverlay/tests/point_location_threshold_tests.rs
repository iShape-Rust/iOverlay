use i_float::int::point::IntPoint;
use i_overlay::core::point_location::IntPointContainment;

#[test]
fn point_location_matches_analytic_strip_on_both_sides_of_tree_threshold() {
    for intervals in [3998_i32, 3999, 4000] {
        // 7998, 8000, 8002 noncollinear vertices; input cleanup preserves all.
        let mut contour: Vec<_> = (0..=intervals)
            .map(|i| IntPoint::new(4 * i, 100 + 4 * (i % 2)))
            .collect();
        contour.extend(
            (0..=intervals)
                .rev()
                .map(|i| IntPoint::new(4 * i, -100 - 4 * (i % 2))),
        );
        assert_eq!(contour.len(), 2 * (intervals as usize + 1));
        let mut queries = Vec::new();
        let mut expected = Vec::new();
        for x in -1..=4 * intervals + 1 {
            if x == 0 || x == 4 * intervals {
                continue;
            }
            let height = 100 + if (x / 4) % 2 == 0 { x % 4 } else { 4 - x % 4 };
            for y in [-105_i32, -103, -101, -99, 0, 99, 101, 103, 105] {
                if y == height || y == -height {
                    continue;
                }
                queries.push(IntPoint::new(x, y));
                expected.push(x > 0 && x < 4 * intervals && y.abs() < height);
            }
        }
        // Transposition produces thousands of simultaneously active edges.
        for transpose in [false, true] {
            if transpose {
                for p in contour.iter_mut().chain(queries.iter_mut()) {
                    std::mem::swap(&mut p.x, &mut p.y);
                }
            }
            for reverse in [false, true] {
                if reverse {
                    contour.reverse();
                }
                let actual = contour.contains_points(&queries);
                if let Some(index) = actual.iter().zip(&expected).position(|(a, b)| a != b) {
                    panic!(
                        "intervals={intervals}, transpose={transpose}, reverse={reverse}, point={:?}, actual={}, expected={}",
                        queries[index], actual[index], expected[index]
                    );
                }
            }
        }
    }
}
