use super::filter::MaskFilter;
use super::overlay_rule::OverlayRule;
use crate::bind::segment::{ContourIndex, IdSegment};
use crate::bind::solver::{JoinHoles, LeftBottomSegment};
use crate::core::graph::OverlayGraph;
use crate::core::link::OverlayLink;
use crate::core::nearest_vector::NearestVector;
use crate::core::node::OverlayNode;
use crate::core::overlay::{ContourDirection, IntOverlayOptions};
use crate::geom::v_segment::VSegment;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use i_shape::int::path::PointPathExtension;
use i_shape::int::shape::{IntContour, IntShapes};
use i_shape::int::simple::Simplify;

impl OverlayGraph {
    /// Extracts shapes from the overlay graph based on the specified overlay rule. This method is used to retrieve the final geometric shapes after boolean operations have been applied. It's suitable for most use cases where the minimum area of shapes is not a concern.
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    /// - Returns: A vector of `IntShape`, representing the geometric result of the applied overlay rule.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    #[inline(always)]
    pub fn extract_shapes(&self, overlay_rule: OverlayRule) -> IntShapes {
        self.extract_shapes_custom(overlay_rule, Default::default())
    }

    /// Extracts shapes from the overlay graph similar to `extract_shapes`, but with an additional constraint on the minimum area of the shapes. This is useful for filtering out shapes that do not meet a certain size threshold, which can be beneficial for eliminating artifacts or noise from the output.
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `options`: Adjust custom behavior.
    /// - Returns: A vector of `IntShape` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    pub fn extract_shapes_custom(
        &self,
        overlay_rule: OverlayRule,
        options: IntOverlayOptions,
    ) -> IntShapes {
        let visited = self.links.filter_by_rule(overlay_rule);
        self.extract(visited, overlay_rule, options)
    }

    pub(crate) fn extract(
        &self,
        filter: Vec<bool>,
        overlay_rule: OverlayRule,
        options: IntOverlayOptions,
    ) -> IntShapes {
        let clockwise = options.output_direction == ContourDirection::Clockwise;

        let mut buffer = filter;
        let visited = buffer.as_mut_slice();
        let mut shapes = Vec::new();
        let mut holes = Vec::new();
        let mut anchors = Vec::new();

        let mut link_index = 0;
        let mut is_all_anchors_sorted = true;
        while link_index < visited.len() {
            if visited.is_visited(link_index) {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_link(link_index, visited);
            let link = self.link(left_top_link);
            let is_hole = overlay_rule.is_fill_top(link.fill);

            let direction = is_hole == clockwise;
            let start_data = StartPathData::new(direction, link, left_top_link);

            let mut contour = self.get_contour(&start_data, direction, visited);
            let (is_valid, is_modified) =
                contour.validate(options.min_output_area, options.preserve_output_collinear);

            if !is_valid {
                link_index += 1;
                continue;
            }

            if is_hole {
                let mut v_segment = if clockwise {
                    VSegment {
                        a: contour[1],
                        b: contour[2],
                    }
                } else {
                    VSegment {
                        a: contour[0],
                        b: contour[contour.len() - 1],
                    }
                };
                if is_modified {
                    let most_left = contour.left_bottom_segment();
                    if most_left != v_segment {
                        v_segment = most_left;
                        is_all_anchors_sorted = false;
                    }
                };

                debug_assert_eq!(v_segment, contour.left_bottom_segment());
                let id_data = ContourIndex::new_hole(holes.len());
                anchors.push(IdSegment::with_segment(id_data, v_segment));
                holes.push(contour);
            } else {
                shapes.push(vec![contour]);
            }
        }

        if !is_all_anchors_sorted {
            anchors.sort_by(|s0, s1| s0.v_segment.a.cmp(&s1.v_segment.a));
        }

        shapes.join_sorted_holes(&self.solver, holes, anchors, clockwise);

        shapes
    }

    fn get_contour(
        &self,
        start_data: &StartPathData,
        clockwise: bool,
        visited: &mut [bool],
    ) -> IntContour {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        let mut contour = IntContour::new();
        contour.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            let node = self.node(node_id);
            link_id = match node {
                OverlayNode::Bridge(bridge) => {
                    if bridge[0] == link_id {
                        bridge[1]
                    } else {
                        bridge[0]
                    }
                }
                OverlayNode::Cross(indices) => {
                    self.find_nearest_link_to(link_id, node_id, clockwise, indices, visited)
                }
            };

            let link = self.link(link_id);
            node_id = if link.a.id == node_id {
                contour.push(link.a.point);
                link.b.id
            } else {
                contour.push(link.b.point);
                link.a.id
            };

            visited.visit(link_id);
        }

        contour
    }

    #[inline]
    pub(crate) fn find_nearest_link_to(
        &self,
        target_index: usize,
        node_id: usize,
        clockwise: bool,
        indices: &[usize],
        visited: &[bool],
    ) -> usize {
        let mut is_first = true;
        let mut first_index = 0;
        let mut second_index = usize::MAX;
        let mut pos = 0;
        for (i, &link_index) in indices.iter().enumerate() {
            if visited.is_not_visited(link_index) {
                if is_first {
                    first_index = link_index;
                    is_first = false;
                } else {
                    second_index = link_index;
                    pos = i;
                    break;
                }
            }
        }

        if second_index == usize::MAX {
            return first_index;
        }

        let target = self.link(target_index);
        let (c, a) = if target.a.id == node_id {
            (target.a.point, target.b.point)
        } else {
            (target.b.point, target.a.point)
        };

        // more the one vectors
        let b = self.link(first_index).other(node_id).point;
        let mut vector_solver = NearestVector::new(c, a, b, first_index, clockwise);

        // add second vector
        vector_solver.add(self.link(second_index).other(node_id).point, second_index);

        // check the rest vectors
        for &link_index in indices.iter().skip(pos + 1) {
            if visited.is_not_visited(link_index) {
                let p = self.link(link_index).other(node_id).point;
                vector_solver.add(p, link_index);
            }
        }

        vector_solver.best_id
    }

    #[inline]
    pub(crate) fn find_left_top_link(&self, link_index: usize, visited: &[bool]) -> usize {
        let top = self.link(link_index);
        debug_assert!(top.is_direct());

        let node = self.node(top.a.id);

        match node {
            OverlayNode::Bridge(bridge) => self.find_left_top_link_on_bridge(bridge),
            OverlayNode::Cross(indices) => {
                self.find_left_top_link_on_indices(top, link_index, indices, visited)
            }
        }
    }

    #[inline(always)]
    fn find_left_top_link_on_indices(
        &self,
        link: &OverlayLink,
        link_index: usize,
        indices: &[usize],
        visited: &[bool],
    ) -> usize {
        let mut top_index = link_index;
        let mut top = link;

        // find most top link

        for &i in indices.iter() {
            if i == link_index {
                continue;
            }
            let link = self.link(i);
            if !link.is_direct()
                || Triangle::is_clockwise_point(top.a.point, top.b.point, link.b.point)
            {
                continue;
            }

            if visited.is_visited(i) {
                continue;
            }

            top_index = i;
            top = link;
        }

        top_index
    }

    #[inline(always)]
    fn find_left_top_link_on_bridge(&self, bridge: &[usize; 2]) -> usize {
        let l0 = self.link(bridge[0]);
        let l1 = self.link(bridge[1]);
        if Triangle::is_clockwise_point(l0.a.point, l0.b.point, l1.b.point) {
            bridge[0]
        } else {
            bridge[1]
        }
    }

    #[inline(always)]
    pub(crate) fn link(&self, index: usize) -> &OverlayLink {
        unsafe { self.links.get_unchecked(index) }
    }

    #[inline(always)]
    pub(crate) fn node(&self, index: usize) -> &OverlayNode {
        unsafe { self.nodes.get_unchecked(index) }
    }
}

pub(crate) struct StartPathData {
    pub(crate) begin: IntPoint,
    pub(crate) node_id: usize,
    pub(crate) link_id: usize,
    pub(crate) last_node_id: usize,
}

impl StartPathData {
    #[inline(always)]
    pub(crate) fn new(direction: bool, link: &OverlayLink, link_id: usize) -> Self {
        if direction {
            Self {
                begin: link.b.point,
                node_id: link.a.id,
                link_id,
                last_node_id: link.b.id,
            }
        } else {
            Self {
                begin: link.a.point,
                node_id: link.b.id,
                link_id,
                last_node_id: link.a.id,
            }
        }
    }
}

pub(crate) trait Validate {
    fn validate(&mut self, min_output_area: u64, preserve_output_collinear: bool) -> (bool, bool);
}

impl Validate for IntContour {
    #[inline]
    fn validate(&mut self, min_output_area: u64, preserve_output_collinear: bool) -> (bool, bool) {
        let is_modified = if !preserve_output_collinear {
            self.simplify_contour()
        } else {
            false
        };

        if self.len() < 3 {
            return (false, is_modified);
        }

        if min_output_area == 0 {
            return (true, is_modified);
        }
        let area = self.unsafe_area();
        let abs_area = area.unsigned_abs() >> 1;
        let is_valid = abs_area >= min_output_area;

        (is_valid, is_modified)
    }
}

pub(crate) trait Visit {
    fn is_visited(&self, index: usize) -> bool;
    fn is_not_visited(&self, index: usize) -> bool;
    fn visit(&mut self, index: usize);
}

impl Visit for [bool] {
    #[inline(always)]
    fn is_visited(&self, index: usize) -> bool {
        unsafe { *self.get_unchecked(index) }
    }

    #[inline(always)]
    fn is_not_visited(&self, index: usize) -> bool {
        !unsafe { *self.get_unchecked(index) }
    }

    #[inline(always)]
    fn visit(&mut self, index: usize) {
        unsafe { *self.get_unchecked_mut(index) = true }
    }
}
