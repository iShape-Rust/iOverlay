use super::*;
use crate::mesh::int::arc::ArcOptions;
use crate::mesh::int::variable_stroke::offset::IntVariableStrokeOffset;

fn vertex(x: i32, y: i32, width: i32) -> IntStrokeVertex<i32> {
    IntStrokeVertex::new(IntPoint::new(x, y), width)
}

fn output<'a>(
    arc: &'a mut ArcBuilder<i32>,
    segments: &'a mut Vec<Segment<ShapeCountBoolean, i32>>,
) -> SegmentBuilder<'a, i32> {
    SegmentBuilder {
        arc,
        segments,
        #[cfg(feature = "variable_stroke_debug")]
        debug_edges: None,
        #[cfg(feature = "variable_stroke_debug")]
        debug_path_index: 0,
    }
}

#[test]
fn covered_break_closes_only_the_larger_side() {
    for (small, large, end_cap, start_cap) in [
        (4000, 20000, Cap::Butt, Cap::Round),
        (20000, 4000, Cap::Round, Cap::Butt),
    ] {
        let path = [
            vertex(-20000, 0, small),
            vertex(0, 0, small),
            vertex(2000, 0, large),
            vertex(22000, 0, large),
        ];
        let parts = VariableStrokeBuilder::find_subsegments(&path);
        assert_eq!(parts.len(), 2);
        assert_eq!((parts[0].start, parts[0].end, parts[0].end_cap), (0, 1, end_cap));
        assert_eq!(
            (parts[1].start, parts[1].end, parts[1].start_cap),
            (2, 3, start_cap)
        );
    }
}

#[test]
fn near_covered_sections_stay_connected() {
    let path = [vertex(0, 0, 600), vertex(757, 386, 1800), vertex(1920, 712, 4200)];
    assert_eq!(
        VariableStrokeBuilder::find_subsegments(&path),
        [SubSegment {
            start: 0,
            end: 2,
            start_cap: Cap::Round,
            end_cap: Cap::Round
        }]
    );
}

#[test]
fn covered_zero_length_butt_section_emits_nothing() {
    let path = [
        vertex(-2000, 0, 20000),
        vertex(0, 0, 2000),
        vertex(2000, 0, 20000),
    ];
    let parts = VariableStrokeBuilder::find_subsegments(&path);
    assert_eq!(parts.len(), 3);
    assert_eq!(
        parts[1],
        SubSegment {
            start: 1,
            end: 1,
            start_cap: Cap::Butt,
            end_cap: Cap::Butt
        }
    );
    let mut arc = ArcBuilder::default();
    let mut segments = Vec::new();
    VariableStrokeBuilder::add_subsegment(&parts[1], &path, &mut output(&mut arc, &mut segments));
    assert!(segments.is_empty());
}

#[test]
fn coverage_requires_a_larger_circle() {
    let c = vertex(50000, 0, 20000);
    for (width, covered) in [(20000, false), (40000, true)] {
        assert_eq!(
            VariableStrokeBuilder::circle_is_covered_by_section(
                &vertex(0, 0, width),
                &vertex(100000, 0, width),
                &c
            ),
            covered
        );
    }
}

#[test]
fn joins_preserve_contacts_and_select_all_exposed_arcs() {
    let cases = [
        (
            [
                vertex(-10000, 0, 4000),
                vertex(0, 0, 10000),
                vertex(10000, 0, 4000),
            ],
            2,
        ),
        (
            [
                vertex(-10000, 0, 4000),
                vertex(0, 0, 4000),
                vertex(0, 10000, 4000),
            ],
            1,
        ),
        (
            [
                vertex(-86000, 2000, 10000),
                vertex(100000, 0, 100000),
                vertex(99000, -45000, 10000),
            ],
            2,
        ),
        (
            [
                vertex(0, 0, 22000),
                vertex(100000, 0, 220000),
                vertex(100000, -100000, 22000),
            ],
            2,
        ),
        (
            [
                vertex(0, 0, 8800),
                vertex(100000, 0, 88000),
                vertex(100000, -100000, 8800),
            ],
            1,
        ),
    ];
    for (path, expected) in cases {
        let prev = Section::try_new(&path[0], &path[1]).unwrap();
        let next = Section::try_new(&path[1], &path[2]).unwrap();
        let mut arc = ArcBuilder::new(IntVariableStrokeStyle::default().arc);
        let mut segments = Vec::new();
        assert_eq!(output(&mut arc, &mut segments).add_join(&prev, &next), expected);
        for p in [prev.b_left, prev.b_right, next.a_left, next.a_right] {
            assert!(
                segments.iter().any(|s| s.x_segment.a == p || s.x_segment.b == p),
                "lost tangent contact {p:?}"
            );
        }
        assert_eq!(
            path.variable_stroke(IntVariableStrokeStyle::default())
                .unwrap()
                .len(),
            1
        );
    }
}

#[test]
fn coarse_arc_preserves_exact_contacts_and_full_major_arc() {
    let center = IntPoint::new(0, 0);
    let from = IntPoint::new(10000, 0);
    let to = IntPoint::new(9950, 998);
    let mut arc = ArcBuilder::default();
    let mut segments = Vec::new();
    assert!(output(&mut arc, &mut segments).add_arc_ccw(
        &center,
        &from,
        &to,
        ArcSweep::Minor,
        #[cfg(feature = "variable_stroke_debug")]
        VariableStrokeDebugEdgeKind::JoinArc
    ));
    assert_eq!(segments.len(), 1);
    let edge = segments[0].x_segment;
    assert!((edge.a == from && edge.b == to) || (edge.b == from && edge.a == to));
    segments.clear();
    assert!(output(&mut arc, &mut segments).add_arc_ccw(
        &center,
        &from,
        &from,
        ArcSweep::Major,
        #[cfg(feature = "variable_stroke_debug")]
        VariableStrokeDebugEdgeKind::JoinArc
    ));
    assert!(segments.len() >= 8);
}

#[test]
fn reversals_close_both_sections_without_an_interior_tooth() {
    for sign in [-1, 1] {
        let path = [
            vertex(-86000, 2000 * sign, 21800),
            vertex(100000, 0, 218000),
            vertex(-20700, -16030 * sign, 21800),
        ];
        let parts = VariableStrokeBuilder::find_subsegments(&path);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].end_cap, Cap::Round);
        assert_eq!((parts[1].start_cap, parts[1].end_cap), (Cap::Butt, Cap::Butt));
        let style = IntVariableStrokeStyle::new().arc(ArcOptions {
            max_step: Angle::from_bits(512_673_957),
            ..ArcOptions::default()
        });
        let shapes = path.variable_stroke(style).unwrap();
        assert_eq!(shapes.len(), 1);
        assert!(!shapes.iter().flatten().flatten().any(|p| {
            let dx = i64::from(p.x) - 100000;
            let dy = i64::from(p.y);
            p.x > 20000 && p.y * sign < -70000 && dx * dx + dy * dy < 108500_i64.pow(2)
        }));
    }
}

#[test]
fn duplicates_and_small_widths_never_emit_zero_length_edges() {
    for width in [0, 1, 2, 4, 1024] {
        for end in [(10, 10), (0, 0), (20, 0)] {
            let path = [
                vertex(0, 0, width),
                vertex(0, 0, width),
                vertex(10, 0, 2 * width),
                vertex(end.0, end.1, width),
            ];
            let mut builder = VariableStrokeBuilder::new(IntVariableStrokeStyle::default());
            let mut segments = Vec::new();
            builder.build(&path, &mut segments);
            assert!(segments.iter().all(|s| s.x_segment.a < s.x_segment.b));
            if width >= 4 {
                assert!(!segments.is_empty());
            }
        }
    }
}

#[test]
fn short_sections_keep_tangent_precision_at_large_widths() {
    for (x, y) in [(1, 1), (2, 2), (3, 2), (1, 7)] {
        for delta in [0, 1] {
            let a = vertex(0, 0, 8192);
            let b = vertex(x, y, 8192 - 2 * delta);
            let section = Section::try_new(&a, &b).unwrap();
            let sq = f64::from(x * x + y * y);
            let tangent = (sq - f64::from(delta * delta)).sqrt();
            for (left, p) in [(true, section.a_left), (false, section.a_right)] {
                let sign = if left { 1.0 } else { -1.0 };
                let expected_x = 4096.0 * (f64::from(delta * x) - sign * tangent * f64::from(y)) / sq;
                let expected_y = 4096.0 * (f64::from(delta * y) + sign * tangent * f64::from(x)) / sq;
                assert!(
                    (f64::from(p.x) - expected_x).abs() <= 0.51,
                    "x={x},y={y},delta={delta}"
                );
                assert!(
                    (f64::from(p.y) - expected_y).abs() <= 0.51,
                    "x={x},y={y},delta={delta}"
                );
            }
        }
    }
}
