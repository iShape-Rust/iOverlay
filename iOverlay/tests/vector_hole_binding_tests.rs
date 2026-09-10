use i_float::int::point::IntPoint;
use i_overlay::core::edge_overlay::{EdgeOverlay, InputEdge};
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;

fn check_hole_binding(direction: ContourDirection) {
    let contours = [
        [[0, 0], [10, 0], [10, 10], [0, 10]],
        [[2, 2], [2, 8], [8, 8], [8, 2]],
        [[20, 0], [30, 0], [30, 10], [20, 10]],
    ];
    let mut overlay = EdgeOverlay::<i32, ()>::new(12);
    overlay.options.output_direction = direction;
    for contour in contours {
        for (a, b) in contour.iter().zip(contour.iter().cycle().skip(1)) {
            overlay.add_edge(
                InputEdge {
                    a: IntPoint::new(a[0], a[1]),
                    b: IntPoint::new(b[0], b[1]),
                    data: (),
                },
                ShapeType::Subject,
            );
        }
    }
    let shapes = overlay.build_vector_shapes(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(shapes.len(), 2);
    let left = shapes.iter().find(|s| s[0].iter().any(|e| e.a.x == 0)).unwrap();
    let right = shapes.iter().find(|s| s[0].iter().any(|e| e.a.x == 20)).unwrap();
    assert_eq!(left.len(), 2, "the left square must own the hole");
    assert_eq!(right.len(), 1, "the right square has no hole");
}

#[test]
fn counterclockwise_vector_output_binds_hole_to_its_shape() {
    check_hole_binding(ContourDirection::CounterClockwise);
}

#[test]
fn clockwise_vector_output_binds_hole_to_its_shape() {
    check_hole_binding(ContourDirection::Clockwise);
}
