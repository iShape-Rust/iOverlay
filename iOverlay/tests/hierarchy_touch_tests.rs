use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{ContourDirection, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;

fn contour(points: &[[i32; 2]]) -> Vec<IntPoint> {
    points.iter().map(|p| IntPoint::new(p[0], p[1])).collect()
}

#[test]
fn island_touching_hole_boundary_keeps_its_parent() {
    for clockwise in [false, true] {
        for island in [
            [[2, 10], [6, 8], [8, 10], [6, 12]],
            [[10, 2], [12, 6], [10, 8], [8, 6]],
            [[18, 10], [14, 12], [12, 10], [14, 8]],
            [[10, 18], [8, 14], [10, 12], [12, 14]],
        ] {
            let subject = vec![
                contour(&[[0, 0], [20, 0], [20, 20], [0, 20]]),
                contour(&[[2, 2], [2, 18], [18, 18], [18, 2]]),
                contour(&island),
            ];
            let mut overlay = Overlay::with_contours(&subject, &[]);
            overlay.options.ogc = true;
            overlay.options.output_direction = if clockwise {
                ContourDirection::Clockwise
            } else {
                ContourDirection::CounterClockwise
            };
            let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::NonZero);
            assert_eq!(
                hierarchy.shapes.shape_ranges.len(),
                2,
                "island={island:?}, clockwise={clockwise}, hierarchy={hierarchy:?}"
            );
            assert_eq!(
                hierarchy.links.len(),
                1,
                "island={island:?}, clockwise={clockwise}, hierarchy={hierarchy:?}"
            );
            let link = hierarchy.links[0];
            let parent = &hierarchy.shapes.shape_ranges[link.parent_shape_index];
            let child = &hierarchy.shapes.shape_ranges[link.child_shape_index];
            assert_eq!(parent.len(), 2);
            assert_eq!(child.len(), 1);
            assert_eq!(link.parent_contour_index, parent.start + 1);
        }
    }
}
