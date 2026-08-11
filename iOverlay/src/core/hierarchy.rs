use crate::bind::segment::{ContourIndex, IdSegment, IdSegments};
use crate::bind::solver::{LeftBottomSegment, ShapeBinder, SortByAngle};
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;
use i_key_sort::sort::key::SortKey;
use i_shape::flat::buffer::FlatShapesBuffer;
use i_shape::int::count::PointsCount;
use i_shape::int::shape::IntShapes;
use i_tree::Expiration;

/// A direct relationship between a hole contour and a shape nested inside it.
///
/// All indices address the flat buffers in [`FlatShapeHierarchy::shapes`].
/// `parent_contour_index` is a global index into
/// [`FlatShapesBuffer::contour_ranges`], not an index local to the parent shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChildLink {
    pub parent_shape_index: usize,
    pub parent_contour_index: usize,
    pub child_shape_index: usize,
}

/// Flat boolean shapes together with their immediate nesting relationships.
///
/// Shapes that do not occur in `links` are standalone one-node trees. A root
/// of a non-trivial tree occurs as a parent but never as a child.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatShapeHierarchy<I: IntNumber> {
    pub shapes: FlatShapesBuffer<I>,
    pub links: Vec<ChildLink>,
}

impl<I: IntNumber> Default for FlatShapeHierarchy<I> {
    fn default() -> Self {
        Self {
            shapes: FlatShapesBuffer::default(),
            links: Vec::new(),
        }
    }
}

impl<I> FlatShapeHierarchy<I>
where
    I: IntNumber + Expiration + SortKey,
{
    pub(crate) fn from_shapes(shapes: IntShapes<I>, clockwise: bool) -> Self {
        let links = Self::bind_links(&shapes, clockwise);
        let shapes = Self::flatten(shapes);

        Self { shapes, links }
    }

    fn bind_links(shapes: &IntShapes<I>, clockwise: bool) -> Vec<ChildLink> {
        let shape_count = shapes.len();
        let hole_count = shapes.iter().map(|shape| shape.len().saturating_sub(1)).sum();
        if shape_count == 0 || hole_count == 0 {
            return Vec::new();
        }

        let mut hole_owners = Vec::with_capacity(hole_count);
        let mut contour_index = 0;
        for (shape_index, shape) in shapes.iter().enumerate() {
            for local_contour_index in 1..shape.len() {
                hole_owners.push((shape_index, contour_index + local_contour_index));
            }
            contour_index += shape.len();
        }

        let mut anchors = Vec::with_capacity(shape_count);
        for (shape_index, shape) in shapes.iter().enumerate() {
            let contour = &shape[0];
            anchors.push(IdSegment::with_segment(
                ContourIndex::new_hole(shape_index),
                contour.left_bottom_segment(),
            ));
        }
        anchors.sort_by_a_then_by_angle();

        let x_min = anchors[0].v_segment.a.x;
        let x_max = anchors[anchors.len() - 1].v_segment.a.x;
        let mut segments = Vec::with_capacity(shapes.points_count() / 2);
        let mut hole_index = 0;

        for (shape_index, shape) in shapes.iter().enumerate() {
            shape[0].append_id_segments(
                &mut segments,
                ContourIndex::new_hole(shape_index),
                x_min,
                x_max,
                !clockwise,
            );

            for hole in shape.iter().skip(1) {
                hole.append_id_segments(
                    &mut segments,
                    ContourIndex::new_shape(hole_index),
                    x_min,
                    x_max,
                    !clockwise,
                );
                hole_index += 1;
            }
        }

        segments.sort_by_a_then_by_angle();
        let solution = ShapeBinder::bind_optional(hole_count, anchors, segments);
        let mut links = Vec::with_capacity(shape_count.saturating_sub(1));

        for (child_shape_index, parent_hole_index) in solution.parent_for_child.into_iter().enumerate() {
            if parent_hole_index == usize::MAX {
                continue;
            }

            let (parent_shape_index, parent_contour_index) = hole_owners[parent_hole_index];
            links.push(ChildLink {
                parent_shape_index,
                parent_contour_index,
                child_shape_index,
            });
        }

        links.sort_unstable();
        links
    }

    fn flatten(shapes: IntShapes<I>) -> FlatShapesBuffer<I> {
        let points_count = shapes.points_count();
        let contour_count = shapes.iter().map(Vec::len).sum();
        let shape_count = shapes.len();
        let mut flat = FlatShapesBuffer::with_capacity(points_count, contour_count, shape_count);

        for shape in shapes {
            let shape_start = flat.contour_ranges.len();
            for contour in shape {
                let point_start = flat.points.len();
                flat.points.extend(contour);
                flat.contour_ranges.push(point_start..flat.points.len());
            }
            flat.shape_ranges.push(shape_start..flat.contour_ranges.len());
        }

        flat
    }
}

#[cfg(test)]
mod tests {
    use super::ChildLink;
    use crate::core::fill_rule::FillRule;
    use crate::core::overlay::{ContourDirection, Overlay};
    use crate::core::overlay_rule::OverlayRule;
    use alloc::vec;
    use i_shape::int_shape;

    #[test]
    fn nested_shapes_form_a_link_chain() {
        #[rustfmt::skip]
        let subject = int_shape![
            [[0, 0], [100, 0], [100, 100], [0, 100]],
            [[10, 10], [10, 90], [90, 90], [90, 10]],
            [[20, 20], [80, 20], [80, 80], [20, 80]],
            [[30, 30], [30, 70], [70, 70], [70, 30]],
            [[40, 40], [60, 40], [60, 60], [40, 60]],
        ];

        let mut overlay = Overlay::with_contours(&subject, &[]);
        let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::EvenOdd);
        let mut regular_overlay = Overlay::with_contours(&subject, &[]);
        let regular_shapes = regular_overlay.overlay(OverlayRule::Subject, FillRule::EvenOdd);

        assert_eq!(hierarchy.shapes.to_shapes(), regular_shapes);
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
    fn one_hole_can_have_multiple_children() {
        #[rustfmt::skip]
        let subject = int_shape![
            [[0, 0], [100, 0], [100, 100], [0, 100]],
            [[10, 10], [10, 90], [90, 90], [90, 10]],
            [[20, 20], [30, 20], [30, 30], [20, 30]],
            [[60, 60], [70, 60], [70, 70], [60, 70]],
        ];

        let mut overlay = Overlay::with_contours(&subject, &[]);
        let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::EvenOdd);

        assert_eq!(hierarchy.shapes.shape_ranges, vec![0..2, 2..3, 3..4]);
        assert_eq!(
            hierarchy.links,
            vec![
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
            ]
        );
    }

    #[test]
    fn standalone_shape_is_absent_from_links() {
        #[rustfmt::skip]
        let subject = int_shape![
            [[0, 0], [100, 0], [100, 100], [0, 100]],
            [[10, 10], [10, 90], [90, 90], [90, 10]],
            [[20, 20], [30, 20], [30, 30], [20, 30]],
            [[200, 0], [210, 0], [210, 10], [200, 10]],
        ];

        let mut overlay = Overlay::with_contours(&subject, &[]);
        overlay.options.output_direction = ContourDirection::Clockwise;
        let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::EvenOdd);

        assert_eq!(hierarchy.shapes.shape_ranges.len(), 3);
        assert_eq!(hierarchy.links.len(), 1);

        let linked = &hierarchy.links[0];
        assert_eq!(linked.parent_shape_index, 0);
        assert_eq!(linked.parent_contour_index, 1);
        assert_eq!(linked.child_shape_index, 1);
        assert!(
            hierarchy
                .links
                .iter()
                .all(|link| link.parent_shape_index != 2 && link.child_shape_index != 2)
        );
    }
}
