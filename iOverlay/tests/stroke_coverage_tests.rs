use i_overlay::mesh::outline::offset::OutlineOffset;
use i_overlay::mesh::stroke::offset::StrokeOffset;
use i_overlay::mesh::style::OutlineStyle;
use i_overlay::mesh::style::{LineCap, LineJoin, StrokeStyle};
use i_overlay::mesh::variable_stroke::offset::VariableStrokeOffset;
use i_overlay::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};

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
    let mut seed = 0x37a2_b951_d477_1011_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 21) as f64 - 10.0
    };
    for case in 0..2000 {
        let path: Vec<_> = (0..3 + case % 8).map(|_| [next(), next()]).collect();
        let style = StrokeStyle::new(4.0)
            .line_join(LineJoin::Round(0.05))
            .start_cap(LineCap::Round(0.05))
            .end_cap(LineCap::Round(0.05));
        let shapes = path.stroke_fixed_scale(style, false, 10000.0).unwrap();
        let vertices: Vec<_> = path.iter().map(|&p| StrokeVertex::new(p, 4.0)).collect();
        assert_vertex_disks_covered(&shapes, &vertices, case);
    }
}

#[test]
fn round_join_covers_a_reversing_vertex() {
    let path = [[0.0, 0.0], [10.0, 0.0], [0.0, 0.0]];
    let style = StrokeStyle::new(4.0)
        .line_join(LineJoin::Round(0.05))
        .start_cap(LineCap::Round(0.05))
        .end_cap(LineCap::Round(0.05));
    let shapes = path.stroke_fixed_scale(style, false, 10000.0).unwrap();
    assert!(
        contains(&shapes, [11.5, 0.0]),
        "round join must cover the turn at x=10"
    );
}

#[test]
fn round_join_covers_a_diagonal_reversing_vertex() {
    let path = [[0.0, 0.0], [6.0, -8.0], [0.0, 0.0]];
    let style = StrokeStyle::new(4.0)
        .line_join(LineJoin::Round(0.05))
        .start_cap(LineCap::Round(0.05))
        .end_cap(LineCap::Round(0.05));
    let shapes = path.stroke_fixed_scale(style, false, 10000.0).unwrap();
    assert!(
        contains(&shapes, [6.9, -9.2]),
        "round join must cover the diagonal turn"
    );
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
