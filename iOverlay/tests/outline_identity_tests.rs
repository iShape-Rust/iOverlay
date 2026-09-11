use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::overlay::OverlayOptions;
use i_overlay::mesh::outline::offset::OutlineOffset;
use i_overlay::mesh::style::OutlineStyle;

type Shapes = Vec<Vec<Vec<IntPoint>>>;

#[test]
fn zero_outline_preserves_touching_triangular_hole() {
    let source = vec![
        vec![
            [-1.0, 0.0],
            [0.0, -3.0],
            [1.0, -4.0],
            [1.0, -5.0],
            [2.0, -5.0],
            [1.0, -3.0],
            [1.0, -2.0],
            [2.0, -1.0],
            [0.0, 1.0],
        ],
        vec![[-1.0, 0.0], [0.0, 0.0], [0.0, -2.0]],
    ];
    let mut options = OverlayOptions::default();
    options.ogc = true;
    options.clean_result = false;
    let result = source
        .outline_custom_fixed_scale(&OutlineStyle::new(0.0), options, 100.0)
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].len(), 2, "missing touching hole: {result:?}");
}

fn canonical(mut shapes: Shapes) -> Shapes {
    for shape in &mut shapes {
        for path in shape.iter_mut() {
            let start = (0..path.len())
                .min_by(|&a, &b| {
                    (0..path.len())
                        .map(|j| path[(a + j) % path.len()])
                        .cmp((0..path.len()).map(|j| path[(b + j) % path.len()]))
                })
                .unwrap();
            path.rotate_left(start);
        }
        shape[1..].sort();
    }
    shapes.sort();
    shapes
}

#[test]
fn ogc_output_preserves_geometry_through_zero_outline() {
    let mut seed = 0xd2a1_8934_71bc_2259_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 41) as i32 - 20
    };
    for case in 0..1000 {
        let paths: Vec<Vec<_>> = (0..1 + case % 5)
            .map(|_| {
                (0..3 + case % 10)
                    .map(|_| IntPoint::new(next(), next()))
                    .collect()
            })
            .collect();
        let mut overlay = Overlay::with_contours(&paths, &[]);
        overlay.options.ogc = true;
        let source = overlay.overlay(OverlayRule::Subject, FillRule::EvenOdd);
        for shape in &source {
            for (index, path) in shape.iter().enumerate() {
                let area: i64 = path
                    .iter()
                    .zip(path.iter().cycle().skip(1))
                    .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
                    .sum();
                assert_eq!(
                    area > 0,
                    index == 0,
                    "invalid source winding: case={case}, path={path:?}"
                );
            }
        }
        let float: Vec<Vec<Vec<[f64; 2]>>> = source
            .iter()
            .map(|s| {
                s.iter()
                    .map(|p| p.iter().map(|p| [p.x as f64, p.y as f64]).collect())
                    .collect()
            })
            .collect();
        for clockwise in [false, true] {
            let mut options = OverlayOptions::default();
            options.ogc = true;
            options.clean_result = case % 2 == 0;
            options.output_direction = if clockwise {
                ContourDirection::Clockwise
            } else {
                ContourDirection::CounterClockwise
            };
            let actual = float
                .outline_custom_fixed_scale(&OutlineStyle::new(0.0), options, 100.0)
                .unwrap();
            let actual = actual
                .iter()
                .map(|s| {
                    s.iter()
                        .map(|p| {
                            p.iter()
                                .map(|p| {
                                    assert!(
                                        (p[0] - p[0].round()).abs() < 1e-8
                                            && (p[1] - p[1].round()).abs() < 1e-8
                                    );
                                    IntPoint::new(p[0].round() as i32, p[1].round() as i32)
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect();
            let mut expected = source.clone();
            if clockwise {
                for path in expected.iter_mut().flatten() {
                    path.reverse();
                }
            }
            let actual = canonical(actual);
            let expected = canonical(expected);
            if actual != expected {
                let missing: Vec<_> = expected.iter().filter(|s| !actual.contains(s)).collect();
                let extra: Vec<_> = actual.iter().filter(|s| !expected.contains(s)).collect();
                panic!("case={case}, clockwise={clockwise}, missing={missing:?}, extra={extra:?}");
            }
        }
    }
}
