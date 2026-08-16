use crate::core::hierarchy::{ChildLink, FlatShapeHierarchy};
use alloc::vec;
use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::int::number::int::IntNumber;
use i_shape::flat::float::FloatFlatShapesBuffer;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::simple::SimplifyContour;

/// Flat float shapes together with their immediate nesting relationships.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatFlatShapeHierarchy<P> {
    pub shapes: FloatFlatShapesBuffer<P>,
    pub links: Vec<ChildLink>,
}

impl<P> Default for FloatFlatShapeHierarchy<P> {
    fn default() -> Self {
        Self {
            shapes: FloatFlatShapesBuffer::with_capacity(0, 0, 0),
            links: Vec::new(),
        }
    }
}

impl<P: FloatPointCompatible> FloatFlatShapeHierarchy<P> {
    pub(crate) fn from_int<I: IntNumber>(
        hierarchy: FlatShapeHierarchy<I>,
        adapter: &FloatPointAdapter<P, I>,
        clean_result: bool,
        preserve_output_collinear: bool,
    ) -> Self {
        let int_shapes = hierarchy.shapes;
        if !clean_result {
            let mut shapes = FloatFlatShapesBuffer::with_capacity(
                int_shapes.points.len(),
                int_shapes.contour_ranges.len(),
                int_shapes.shape_ranges.len(),
            );
            let points = int_shapes.points.iter().map(|point| adapter.int_to_float(point));
            shapes.set_with_iter(points, &int_shapes.contour_ranges, &int_shapes.shape_ranges);
            return Self {
                shapes,
                links: hierarchy.links,
            };
        }

        let mut shapes = FloatFlatShapesBuffer::with_capacity(
            int_shapes.points.len(),
            int_shapes.contour_ranges.len(),
            int_shapes.shape_ranges.len(),
        );
        let mut shape_map = vec![usize::MAX; int_shapes.shape_ranges.len()];
        let mut contour_map = vec![usize::MAX; int_shapes.contour_ranges.len()];

        for (old_shape_index, old_shape_range) in int_shapes.shape_ranges.iter().enumerate() {
            let new_shape_start = shapes.contour_ranges.len();
            let mut hull_is_empty = false;

            for old_contour_index in old_shape_range.clone() {
                let point_range = int_shapes.contour_ranges[old_contour_index].clone();
                let mut contour: Vec<P> = int_shapes.points[point_range]
                    .iter()
                    .map(|point| adapter.int_to_float(point))
                    .collect();

                if preserve_output_collinear {
                    contour.despike_contour(adapter);
                } else {
                    contour.simplify_contour(adapter);
                }

                if contour.is_empty() {
                    if old_contour_index == old_shape_range.start {
                        hull_is_empty = true;
                        break;
                    }
                    continue;
                }

                contour_map[old_contour_index] = shapes.contour_ranges.len();
                let point_start = shapes.points.len();
                shapes.points.extend(contour);
                shapes.contour_ranges.push(point_start..shapes.points.len());
            }

            if hull_is_empty {
                shapes.points.truncate(
                    shapes
                        .contour_ranges
                        .get(new_shape_start)
                        .map_or(shapes.points.len(), |range| range.start),
                );
                shapes.contour_ranges.truncate(new_shape_start);
                for old_contour_index in old_shape_range.clone() {
                    contour_map[old_contour_index] = usize::MAX;
                }
                continue;
            }

            shape_map[old_shape_index] = shapes.shape_ranges.len();
            shapes
                .shape_ranges
                .push(new_shape_start..shapes.contour_ranges.len());
        }

        let mut links = Vec::with_capacity(hierarchy.links.len());
        for link in hierarchy.links {
            let parent_shape_index = shape_map[link.parent_shape_index];
            let parent_contour_index = contour_map[link.parent_contour_index];
            let child_shape_index = shape_map[link.child_shape_index];

            if parent_shape_index == usize::MAX
                || parent_contour_index == usize::MAX
                || child_shape_index == usize::MAX
            {
                continue;
            }

            links.push(ChildLink {
                parent_shape_index,
                parent_contour_index,
                child_shape_index,
            });
        }
        debug_assert!(links.windows(2).all(|pair| pair[0] <= pair[1]));

        Self { shapes, links }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::extract::BooleanExtractionBuffer;
    use crate::core::fill_rule::FillRule;
    use crate::core::hierarchy::{ChildLink, FlatShapeHierarchy};
    use crate::core::overlay_rule::OverlayRule;
    use crate::core::solver::Solver;
    use crate::float::hierarchy::FloatFlatShapeHierarchy;
    use crate::float::overlay::{FloatOverlay, OverlayOptions};
    use alloc::vec;
    use alloc::vec::Vec;
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;
    use i_float::int::point::IntPoint;
    use i_shape::flat::buffer::FlatShapesBuffer;

    #[test]
    fn float_overlay_exports_nested_hierarchy() {
        let subject = nested_subject::<f64>();
        let clip: Vec<Vec<[f64; 2]>> = Vec::new();
        let mut overlay = FloatOverlay::with_subj_and_clip(&subject, &clip);
        let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::EvenOdd);

        assert_eq!(hierarchy.shapes.shape_ranges, vec![0..2, 2..4, 4..5]);
        assert_eq!(
            hierarchy.links,
            vec![
                ChildLink {
                    parent_shape_index: 0,
                    parent_contour_index: 1,
                    child_shape_index: 1,
                },
                ChildLink {
                    parent_shape_index: 1,
                    parent_contour_index: 3,
                    child_shape_index: 2,
                },
            ]
        );
    }

    #[test]
    fn float_graph_exports_nested_hierarchy_with_clean_result() {
        let subject = nested_subject::<f32>();
        let clip: Vec<Vec<[f32; 2]>> = Vec::new();
        let mut overlay = FloatOverlay::with_subj_and_clip(&subject, &clip);
        let graph = overlay.build_graph_view(FillRule::EvenOdd).unwrap();
        let mut buffer = BooleanExtractionBuffer::default();
        let hierarchy = graph.extract_shape_hierarchy(OverlayRule::Subject, &mut buffer);

        assert_eq!(hierarchy.shapes.shape_ranges, vec![0..2, 2..4, 4..5]);
        assert_eq!(hierarchy.links.len(), 2);
    }

    #[test]
    fn float_graph_hierarchy_covers_other_cleaning_modes() {
        let subject = nested_subject::<f32>();
        let mut options = OverlayOptions::<f32>::default();
        options.preserve_output_collinear = true;
        let mut overlay = FloatOverlay::from_subj_custom(&subject, options, Solver::default());
        let graph = overlay.build_graph_view(FillRule::EvenOdd).unwrap();
        let mut buffer = BooleanExtractionBuffer::default();
        let preserved = graph.extract_shape_hierarchy(OverlayRule::Subject, &mut buffer);

        assert_eq!(preserved.shapes.shape_ranges, vec![0..2, 2..4, 4..5]);
        assert_eq!(preserved.links.len(), 2);

        let subject = nested_subject::<f64>();
        let mut overlay = FloatOverlay::<[f64; 2], i32>::from_subj(&subject);
        let graph = overlay.build_graph_view(FillRule::EvenOdd).unwrap();
        let mut buffer = BooleanExtractionBuffer::default();
        let uncleaned = graph.extract_shape_hierarchy(OverlayRule::Subject, &mut buffer);

        assert_eq!(uncleaned.shapes.shape_ranges, vec![0..2, 2..4, 4..5]);
        assert_eq!(uncleaned.links.len(), 2);
    }

    #[test]
    fn default_hierarchy_is_empty() {
        let hierarchy = FloatFlatShapeHierarchy::<[f64; 2]>::default();

        assert!(hierarchy.shapes.points.is_empty());
        assert!(hierarchy.shapes.contour_ranges.is_empty());
        assert!(hierarchy.shapes.shape_ranges.is_empty());
        assert!(hierarchy.links.is_empty());
    }

    #[test]
    fn clean_result_preserves_valid_collinear_mode_contour() {
        let int_hierarchy = FlatShapeHierarchy {
            shapes: FlatShapesBuffer {
                points: vec![
                    IntPoint::new(0, 0),
                    IntPoint::new(10, 0),
                    IntPoint::new(10, 10),
                    IntPoint::new(0, 10),
                ],
                contour_ranges: vec![0..4],
                shape_ranges: vec![0..1],
            },
            links: vec![],
        };
        let adapter =
            FloatPointAdapter::<[f64; 2], i32>::with_scale(FloatRect::new(-10.0, 20.0, -10.0, 20.0), 1.0);

        let hierarchy = FloatFlatShapeHierarchy::from_int(int_hierarchy, &adapter, true, true);

        assert_eq!(hierarchy.shapes.shape_ranges, vec![0..1]);
        assert_eq!(hierarchy.shapes.contour_ranges, vec![0..4]);
        assert!(hierarchy.links.is_empty());
    }

    #[test]
    fn clean_result_drops_empty_hull_and_remaps_surviving_link() {
        let int_hierarchy = FlatShapeHierarchy {
            shapes: FlatShapesBuffer {
                points: vec![
                    IntPoint::new(0, 0),
                    IntPoint::new(20, 0),
                    IntPoint::new(20, 20),
                    IntPoint::new(0, 20),
                    IntPoint::new(5, 5),
                    IntPoint::new(5, 15),
                    IntPoint::new(15, 15),
                    IntPoint::new(15, 5),
                    IntPoint::new(6, 6),
                    IntPoint::new(7, 6),
                    IntPoint::new(8, 6),
                    IntPoint::new(7, 7),
                    IntPoint::new(9, 7),
                    IntPoint::new(9, 9),
                    IntPoint::new(7, 9),
                ],
                contour_ranges: vec![0..4, 4..8, 8..11, 11..15],
                shape_ranges: vec![0..2, 2..3, 3..4],
            },
            links: vec![
                ChildLink {
                    parent_shape_index: 0,
                    parent_contour_index: 1,
                    child_shape_index: 1,
                },
                ChildLink {
                    parent_shape_index: 0,
                    parent_contour_index: 1,
                    child_shape_index: 2,
                },
                ChildLink {
                    parent_shape_index: 1,
                    parent_contour_index: 2,
                    child_shape_index: 2,
                },
            ],
        };
        let adapter =
            FloatPointAdapter::<[f64; 2], i32>::with_scale(FloatRect::new(-10.0, 30.0, -10.0, 30.0), 1.0);

        let hierarchy = FloatFlatShapeHierarchy::from_int(int_hierarchy, &adapter, true, false);

        assert_eq!(hierarchy.shapes.shape_ranges, vec![0..2, 2..3]);
        assert_eq!(
            hierarchy.links,
            vec![ChildLink {
                parent_shape_index: 0,
                parent_contour_index: 1,
                child_shape_index: 1,
            }]
        );
    }

    #[test]
    fn clean_result_remaps_removed_hole() {
        let int_hierarchy = FlatShapeHierarchy {
            shapes: FlatShapesBuffer {
                points: vec![
                    IntPoint::new(0, 0),
                    IntPoint::new(10, 0),
                    IntPoint::new(10, 10),
                    IntPoint::new(0, 10),
                    IntPoint::new(2, 2),
                    IntPoint::new(3, 2),
                    IntPoint::new(4, 2),
                    IntPoint::new(3, 3),
                    IntPoint::new(4, 3),
                    IntPoint::new(4, 4),
                    IntPoint::new(3, 4),
                ],
                contour_ranges: vec![0..4, 4..7, 7..11],
                shape_ranges: vec![0..2, 2..3],
            },
            links: vec![ChildLink {
                parent_shape_index: 0,
                parent_contour_index: 1,
                child_shape_index: 1,
            }],
        };
        let adapter =
            FloatPointAdapter::<[f64; 2], i32>::with_scale(FloatRect::new(-10.0, 20.0, -10.0, 20.0), 1.0);

        let hierarchy = FloatFlatShapeHierarchy::from_int(int_hierarchy, &adapter, true, false);

        assert_eq!(hierarchy.shapes.shape_ranges, vec![0..1, 1..2]);
        assert_eq!(hierarchy.shapes.contour_ranges.len(), 2);
        assert!(hierarchy.links.is_empty());
    }

    fn nested_subject<F: From<f32> + Copy>() -> Vec<Vec<[F; 2]>> {
        [
            [[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]],
            [[10.0, 10.0], [10.0, 90.0], [90.0, 90.0], [90.0, 10.0]],
            [[20.0, 20.0], [80.0, 20.0], [80.0, 80.0], [20.0, 80.0]],
            [[30.0, 30.0], [30.0, 70.0], [70.0, 70.0], [70.0, 30.0]],
            [[40.0, 40.0], [60.0, 40.0], [60.0, 60.0], [40.0, 60.0]],
        ]
        .into_iter()
        .map(|contour| {
            contour
                .into_iter()
                .map(|[x, y]| [F::from(x), F::from(y)])
                .collect()
        })
        .collect()
    }
}
