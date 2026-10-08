use i_overlay::mesh::float::outline::offset::OutlineOffset;
use i_overlay::mesh::float::stroke::offset::StrokeOffset;
use i_overlay::mesh::float::style::OutlineStyle;
use i_overlay::mesh::float::style::{LineCap, LineJoin, StrokeStyle};
use i_overlay::mesh::float::variable_stroke::offset::VariableStrokeOffset;
use i_overlay::mesh::float::variable_stroke::{StrokeVertex, VariableStrokeStyle};
use i_overlay::mesh::math::MathMode;

fn contains(shapes: &[Vec<Vec<[f64; 2]>>], p: [f64; 2]) -> bool {
    shapes.iter().any(|shape| {
        let mut inside = false;
        for contour in shape {
            let mut a = contour[contour.len() - 1];
            for &b in contour {
                if (a[1] > p[1]) != (b[1] > p[1]) {
                    let x = a[0] + (p[1] - a[1]) * (b[0] - a[0]) / (b[1] - a[1]);
                    if p[0] < x {
                        inside = !inside;
                    }
                }
                a = b;
            }
        }
        inside
    })
}

#[test]
fn wide_strokes_cover_short_segment_rectangles() {
    // Issue #95: the inner join must not cut triangles out of either segment's rectangle.
    for math in [MathMode::Integer, MathMode::Float] {
        for join in [LineJoin::Bevel, LineJoin::Round(0.05), LineJoin::Miter(0.1)] {
            for radius in [0.5, 1.025, 2.0, 3.0] {
                for sign in [-1.0, 1.0] {
                    for reversed in [false, true] {
                        let mut path = [[0.0, 0.0], [1.0, 0.0], [1.0, sign]];
                        if reversed {
                            path.reverse();
                        }
                        let style = StrokeStyle::new(2.0 * radius)
                            .math(math)
                            .line_join(join.clone())
                            .start_cap(LineCap::Butt)
                            .end_cap(LineCap::Butt);
                        let shapes = path.stroke(style, false);
                        let context =
                            format!("{math:?}, {join:?}, radius={radius}, sign={sign}, reversed={reversed}");
                        assert_eq!(shapes.len(), 1, "{context}");
                        assert_eq!(shapes[0].len(), 1, "{context}");

                        for i in 0..10 {
                            for j in 0..10 {
                                let along = (i as f64 + 0.5) / 10.0;
                                let across = radius * (2.0 * (j as f64 + 0.5) / 10.0 - 1.0);
                                for point in [[along, across], [1.0 + across, sign * along]] {
                                    assert!(
                                        contains(&shapes, point),
                                        "uncovered rectangle: {point:?}, {context}"
                                    );
                                }
                            }
                        }
                        if radius > 1.0 {
                            // Sample the triangles near the caps, even just above the failure threshold.
                            let d = 0.25 * (radius - 1.0);
                            for point in [[-d, sign * d], [1.0 - d, sign * (1.0 + d)]] {
                                assert!(
                                    contains(&shapes, point),
                                    "uncovered overhang: {point:?}, {context}"
                                );
                            }
                        }
                        if matches!(join, LineJoin::Bevel | LineJoin::Miter(_)) {
                            let area: f64 = shapes
                                .iter()
                                .flatten()
                                .map(|contour| {
                                    contour
                                        .iter()
                                        .zip(contour.iter().cycle().skip(1))
                                        .take(contour.len())
                                        .map(|(a, b)| a[0] * b[1] - a[1] * b[0])
                                        .sum::<f64>()
                                        * 0.5
                                })
                                .sum();
                            let join_area = if matches!(join, LineJoin::Bevel) {
                                0.5 * radius * radius
                            } else {
                                radius * radius
                            };
                            // Two segment rectangles, minus their overlap, plus the outer join.
                            let expected_area = 4.0 * radius - radius.min(1.0).powi(2) + join_area;
                            assert!(
                                (area - expected_area).abs() < 1e-6,
                                "area={area}, expected={expected_area}, {context}"
                            );
                        }
                    }
                }
            }
        }
    }
}

fn assert_vertex_disks_covered(shapes: &[Vec<Vec<[f64; 2]>>], path: &[StrokeVertex<[f64; 2]>], case: usize) {
    for (index, vertex) in path.iter().enumerate() {
        for sample in 0..16 {
            let angle = sample as f64 * core::f64::consts::TAU / 16.0;
            // Stay well inside the disk to exclude tessellation and grid error.
            let radius = 0.4 * vertex.width;
            let point = [
                vertex.point[0] + radius * angle.cos(),
                vertex.point[1] + radius * angle.sin(),
            ];
            assert!(
                contains(shapes, point),
                "uncovered disk: case={case}, vertex={index}, sample={sample}, point={point:?}, path={path:?}"
            );
        }
    }
}

#[test]
fn round_strokes_cover_vertex_disks() {
    for math in [MathMode::Integer, MathMode::Float] {
        let mut seed = 0x37a2_b951_d477_1011_u64;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((seed >> 32) % 21) as f64 - 10.0
        };
        for case in 0..1000 {
            let path: Vec<_> = (0..3 + case % 8).map(|_| [next(), next()]).collect();
            let style = StrokeStyle::new(4.0)
                .math(math)
                .line_join(LineJoin::Round(0.05))
                .start_cap(LineCap::Round(0.05))
                .end_cap(LineCap::Round(0.05));
            let shapes = path.stroke_fixed_scale(style, false, 10000.0).unwrap();
            let vertices: Vec<_> = path.iter().map(|&p| StrokeVertex::new(p, 4.0)).collect();
            assert_vertex_disks_covered(&shapes, &vertices, case);
        }
    }
}

#[test]
fn round_join_covers_a_reversing_vertex() {
    for math in [MathMode::Integer, MathMode::Float] {
        let path = [[0.0, 0.0], [10.0, 0.0], [0.0, 0.0]];
        let style = StrokeStyle::new(4.0)
            .math(math)
            .line_join(LineJoin::Round(0.05))
            .start_cap(LineCap::Round(0.05))
            .end_cap(LineCap::Round(0.05));
        let shapes = path.stroke_fixed_scale(style, false, 10000.0).unwrap();
        assert!(
            contains(&shapes, [11.5, 0.0]),
            "round join must cover the turn at x=10"
        );
    }
}

#[test]
fn round_join_covers_a_diagonal_reversing_vertex() {
    for math in [MathMode::Integer, MathMode::Float] {
        let path = [[0.0, 0.0], [6.0, -8.0], [0.0, 0.0]];
        let style = StrokeStyle::new(4.0)
            .math(math)
            .line_join(LineJoin::Round(0.05))
            .start_cap(LineCap::Round(0.05))
            .end_cap(LineCap::Round(0.05));
        let shapes = path.stroke_fixed_scale(style, false, 10000.0).unwrap();
        assert!(
            contains(&shapes, [6.9, -9.2]),
            "round join must cover the diagonal turn"
        );
    }
}

#[test]
fn round_outline_covers_a_diagonal_spike() {
    let path = [
        [0.0, 0.0],
        [0.0, 10.0],
        [-10.0, 10.0],
        [-10.0, 0.0],
        [0.0, 0.0],
        [6.0, -8.0],
    ];
    let style = OutlineStyle::new(2.0).line_join(LineJoin::Round(0.05));
    let shapes = path.outline_fixed_scale(&style, 10000.0).unwrap();
    assert!(
        contains(&shapes, [6.9, -9.2]),
        "round outline must cover the diagonal tip"
    );
}

#[test]
fn variable_strokes_cover_vertex_disks() {
    let mut seed = 0x37a2_b951_d477_1011_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 21) as f64 - 10.0
    };
    for case in 0..2000 {
        let path: Vec<_> = (0..3 + case % 8)
            .map(|_| StrokeVertex::new([next(), next()], next() + 11.0))
            .collect();
        let shapes = path
            .variable_stroke_fixed_scale(VariableStrokeStyle::new().round_angle(0.05), 10000.0)
            .unwrap();
        assert_vertex_disks_covered(&shapes, &path, case);
    }
}

#[test]
fn variable_strokes_cover_interpolated_disks() {
    let mut seed = 0xa153_61b9_911d_7283_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 41) as f64 - 20.0
    };
    for case in 0..1800 {
        let path: Vec<_> = (0..2 + case % 8)
            .map(|_| StrokeVertex::new([next(), next()], next() + 20.0))
            .collect();
        let shapes = path
            .variable_stroke_fixed_scale(VariableStrokeStyle::new().round_angle(0.05), 10000.0)
            .unwrap();
        let mut samples = Vec::new();
        for pair in path.windows(2) {
            for step in 1..4 {
                let t = step as f64 / 4.0;
                let width = pair[0].width * (1.0 - t) + pair[1].width * t;
                if width == 0.0 {
                    continue;
                }
                samples.push(StrokeVertex::new(
                    [
                        pair[0].point[0] * (1.0 - t) + pair[1].point[0] * t,
                        pair[0].point[1] * (1.0 - t) + pair[1].point[1] * t,
                    ],
                    width,
                ));
            }
        }
        assert_vertex_disks_covered(&shapes, &samples, case);
    }
}

// Minimize squared distance minus squared interpolated radius along a
// centerline segment. This oracle does not construct joins or tangent edges.
fn within_variable_stroke(path: &[StrokeVertex<[f64; 2]>], p: [f64; 2]) -> bool {
    path.windows(2).any(|pair| {
        let a = pair[0];
        let b = pair[1];
        let dx = b.point[0] - a.point[0];
        let dy = b.point[1] - a.point[1];
        let x = p[0] - a.point[0];
        let y = p[1] - a.point[1];
        // Allow a margin for integer snapping and arc approximation.
        let r = 0.5 * a.width + 0.03;
        let dr = 0.5 * (b.width - a.width);
        let qa = dx * dx + dy * dy - dr * dr;
        let qb = -2.0 * (x * dx + y * dy + r * dr);
        let qc = x * x + y * y - r * r;
        let mut minimum = qc.min(qa + qb + qc);
        if qa > 0.0 {
            let t = (-qb / (2.0 * qa)).clamp(0.0, 1.0);
            minimum = minimum.min((qa * t + qb) * t + qc);
        }
        minimum <= 0.0
    })
}

#[test]
fn variable_strokes_do_not_fill_outside_interpolated_disks() {
    let mut seed = 0xa153_61b9_911d_7283_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 41) as f64 - 20.0
    };
    for case in 0..350 {
        let path: Vec<_> = (0..2 + case % 8)
            .map(|_| StrokeVertex::new([next(), next()], next() + 20.0))
            .collect();
        let shapes = path
            .variable_stroke_fixed_scale(VariableStrokeStyle::new().round_angle(0.05), 10000.0)
            .unwrap();
        for x in -40..=40 {
            for y in -40..=40 {
                let p = [x as f64 + 0.37, y as f64 + 0.29];
                if !within_variable_stroke(&path, p) {
                    assert!(
                        !contains(&shapes, p),
                        "excess area: case={case}, p={p:?}, path={path:?}, shapes={shapes:?}"
                    );
                }
            }
        }
    }
}
