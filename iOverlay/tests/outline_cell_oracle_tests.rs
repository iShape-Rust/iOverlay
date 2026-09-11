use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::mesh::outline::offset::OutlineOffset;
use i_overlay::mesh::style::{LineJoin, OutlineStyle};

fn contains(shapes: &[Vec<Vec<[f64; 2]>>], x: f64, y: f64) -> bool {
    shapes.iter().any(|shape| {
        let mut inside = false;
        for path in shape {
            for (a, b) in path.iter().zip(path.iter().cycle().skip(1)) {
                assert!(
                    a[0] == b[0] || a[1] == b[1],
                    "miter offsets of orthogonal shapes must stay orthogonal"
                );
                if (a[1] > y) != (b[1] > y) && a[0] > x {
                    inside = !inside;
                }
            }
        }
        inside
    })
}

#[test]
fn orthogonal_miter_offsets_match_cell_dilation_and_erosion() {
    let mut seed = 0x2917_8b42_357a_1de3_u64;
    for case in 0..200 {
        let mut cells = [[false; 6]; 6];
        let mut contours = Vec::new();
        for x in 0..6 {
            for y in 0..6 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                cells[x][y] = (seed >> 32) % 100 < 20 + case % 65;
                if cells[x][y] {
                    let (x, y) = (4 * x as i32, 4 * y as i32);
                    contours.push(vec![
                        IntPoint::new(x, y),
                        IntPoint::new(x + 4, y),
                        IntPoint::new(x + 4, y + 4),
                        IntPoint::new(x, y + 4),
                    ]);
                }
            }
        }
        let mut overlay = Overlay::with_contours(&contours, &[]);
        overlay.options.ogc = true;
        let subject = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);
        let source: Vec<Vec<Vec<_>>> = subject
            .iter()
            .map(|s| {
                s.iter()
                    .map(|p| p.iter().map(|p| [p.x as f64, p.y as f64]).collect())
                    .collect()
            })
            .collect();
        let occupied = |x: i32, y: i32| {
            let (cx, cy) = (x.div_euclid(4), y.div_euclid(4));
            (0..6).contains(&cx) && (0..6).contains(&cy) && cells[cx as usize][cy as usize]
        };
        for offset in [-2_i32, -1, 1, 2] {
            let style = OutlineStyle::new(offset as f64).line_join(LineJoin::Miter(0.1));
            let result = source.outline_fixed_scale(&style, 100.0).unwrap();
            let mut expected_area = 0;
            for x in -3..27 {
                for y in -3..27 {
                    let r = offset.abs();
                    // A miter offset of an orthogonal boundary is dilation or
                    // erosion by an axis-aligned square. Half-unit samples avoid
                    // every output boundary and cover all unit cells exactly.
                    let expected = if offset > 0 {
                        (-r..=r).any(|dx| (-r..=r).any(|dy| occupied(x + dx, y + dy)))
                    } else {
                        (-r..=r).all(|dx| (-r..=r).all(|dy| occupied(x + dx, y + dy)))
                    };
                    expected_area += i32::from(expected);
                    assert_eq!(
                        contains(&result, x as f64 + 0.5, y as f64 + 0.5),
                        expected,
                        "case={case}, offset={offset}, sample=({x}.5,{y}.5), cells={cells:?}, source={source:?}, result={result:?}"
                    );
                }
            }
            let area: f64 = result
                .iter()
                .flatten()
                .map(|p| {
                    p.iter()
                        .zip(p.iter().cycle().skip(1))
                        .map(|(a, b)| 0.5 * (a[0] * b[1] - a[1] * b[0]))
                        .sum::<f64>()
                })
                .sum();
            assert!(
                (area - expected_area as f64).abs() < 1e-6,
                "case={case}, offset={offset}, area={area}, expected={expected_area}"
            );
        }
    }
}
