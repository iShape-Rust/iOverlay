use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::hierarchy::ChildLink;
use i_overlay::core::overlay::{ContourDirection, IntOverlayOptions, Overlay};
use i_overlay::core::overlay_rule::OverlayRule;

fn contour(points: &[[i32; 2]]) -> Vec<IntPoint> {
    points.iter().map(|p| IntPoint::new(p[0], p[1])).collect()
}

#[test]
fn hierarchy_preserves_links_and_honors_collinear_output_option() {
    let subject = vec![
        contour(&[[0, 0], [10, 0], [20, 0], [20, 20], [0, 20]]),
        contour(&[[2, 2], [2, 10], [2, 18], [18, 18], [18, 2]]),
        contour(&[[4, 4], [6, 4], [8, 4], [8, 8], [4, 8]]),
    ];
    for ogc in [false, true] {
        for preserve in [false, true] {
            for direction in [ContourDirection::CounterClockwise, ContourDirection::Clockwise] {
                let options = IntOverlayOptions {
                    preserve_input_collinear: true,
                    preserve_output_collinear: preserve,
                    output_direction: direction,
                    ogc,
                    ..Default::default()
                };
                let mut overlay = Overlay::from_subj_custom(&subject, options, Default::default());
                let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::NonZero);
                let shapes = overlay.overlay(OverlayRule::Subject, FillRule::NonZero);

                assert_eq!(hierarchy.shapes.to_shapes(), shapes);
                assert_eq!(hierarchy.shapes.shape_ranges, vec![0..2, 2..3]);
                assert!(
                    hierarchy
                        .shapes
                        .contour_ranges
                        .iter()
                        .all(|r| r.len() == if preserve { 5 } else { 4 })
                );
                assert_eq!(
                    hierarchy.links,
                    vec![ChildLink {
                        parent_shape_index: 0,
                        parent_contour_index: 1,
                        child_shape_index: 1,
                    }]
                );
            }
        }
    }
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
            let mut overlay = Overlay::from_subj(&subject);
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
#[test]
fn holes_sharing_one_vertex_keep_their_own_islands() {
    let outer = contour(&[[0, 0], [40, 0], [40, 40], [0, 40]]);
    let mut subject = vec![outer];
    for rotation in 0..4 {
        let rotate = |mut p: [i32; 2]| {
            for _ in 0..rotation {
                p = [40 - p[1], p[0]];
            }
            p
        };
        // Clockwise holes touch at (20,20), with one CCW island per hole.
        subject.push(contour(&[[20, 20], [14, 28], [20, 36], [26, 28]].map(rotate)));
        subject.push(contour(&[[19, 27], [21, 27], [21, 29], [19, 29]].map(rotate)));
    }
    for clockwise in [false, true] {
        for reversed_input_order in [false, true] {
            if reversed_input_order {
                subject.reverse();
            }
            let mut overlay = Overlay::from_subj(&subject);
            overlay.options.ogc = true;
            overlay.options.output_direction = if clockwise {
                ContourDirection::Clockwise
            } else {
                ContourDirection::CounterClockwise
            };
            let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::NonZero);
            let flat = &hierarchy.shapes;
            assert_eq!(flat.shape_ranges.len(), 5, "{hierarchy:?}");
            assert_eq!(hierarchy.links.len(), 4, "{hierarchy:?}");
            let mut total_area_two = 0_i64;
            for shape in &flat.shape_ranges {
                for (local, range) in flat.contour_ranges[shape.clone()].iter().enumerate() {
                    let path = &flat.points[range.clone()];
                    let area_two: i64 = path
                        .iter()
                        .zip(path.iter().cycle().skip(1))
                        .map(|(a, b)| i64::from(a.x) * i64::from(b.y) - i64::from(a.y) * i64::from(b.x))
                        .sum();
                    assert_eq!(area_two < 0, clockwise != (local != 0));
                    total_area_two += area_two;
                    let unique: std::collections::BTreeSet<_> = path.iter().map(|p| (p.x, p.y)).collect();
                    assert_eq!(unique.len(), path.len(), "OGC contour repeats a vertex");
                }
            }
            assert_eq!(total_area_two.abs(), 2 * (1600 - 4 * 96 + 4 * 4));
            let mut owners = std::collections::BTreeSet::new();
            for link in &hierarchy.links {
                assert!(owners.insert(link.parent_contour_index));
                let parent = &flat.shape_ranges[link.parent_shape_index];
                assert_eq!(parent.len(), 5);
                assert!(link.parent_contour_index > parent.start && link.parent_contour_index < parent.end);
                let child = &flat.shape_ranges[link.child_shape_index];
                assert_eq!(child.len(), 1);
                let child_points = &flat.points[flat.contour_ranges[child.start].clone()];
                let hole_points = &flat.points[flat.contour_ranges[link.parent_contour_index].clone()];
                // Every island vertex must lie strictly inside its convex hole.
                for p in child_points {
                    let crosses: Vec<_> = hole_points
                        .iter()
                        .zip(hole_points.iter().cycle().skip(1))
                        .map(|(a, b)| (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x))
                        .collect();
                    assert!(
                        crosses.iter().all(|&c| c > 0) || crosses.iter().all(|&c| c < 0),
                        "wrong hole owner: {hierarchy:?}"
                    );
                }
            }
        }
    }
}
