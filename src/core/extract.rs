use i_float::point::IntPoint;
use i_float::triangle::Triangle;
use i_shape::int::path::{IntPath, PointPathExtension};
use i_shape::int::shape::IntShapes;
use i_shape::int::simple::Simple;
use crate::bind::solver::JoinHoles;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_link::OverlayLink;
use crate::core::overlay_node::OverlayNode;
use crate::core::vector_rotation::NearestCCWVector;

use super::overlay_rule::OverlayRule;
use super::filter::Filter;

impl OverlayGraph {
    /// Extracts shapes from the overlay graph based on the specified overlay rule. This method is used to retrieve the final geometric shapes after boolean operations have been applied. It's suitable for most use cases where the minimum area of shapes is not a concern.
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    /// - Returns: A vector of `IntShape`, representing the geometric result of the applied overlay rule.
    /// # Shape Representation
    /// The output is a `Vec<Vec<Vec<IntPoint>>>`, where:
    /// - The outer `Vec<Shape>` represents a set of shapes.
    /// - Each shape `Vec<Path>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline(always)]
    pub fn extract_shapes(&self, overlay_rule: OverlayRule) -> IntShapes {
        self.extract_shapes_min_area(overlay_rule, 0)
    }

    /// Extracts shapes from the overlay graph similar to `extract_shapes`, but with an additional constraint on the minimum area of the shapes. This is useful for filtering out shapes that do not meet a certain size threshold, which can be beneficial for eliminating artifacts or noise from the output.
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `min_area`: The minimum area threshold for shapes to be included in the result. Shapes with an area smaller than this value will be excluded.
    /// - Returns: A vector of `IntShape` that meet the specified area criteria, representing the cleaned-up geometric result.
    /// # Shape Representation
    /// The output is a `Vec<Vec<Vec<IntPoint>>>`, where:
    /// - The outer `Vec<Shape>` represents a set of shapes.
    /// - Each shape `Vec<Path>` represents a collection of paths, where the first path is the outer boundary, and all subsequent paths are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    pub fn extract_shapes_min_area(&self, overlay_rule: OverlayRule, min_area: usize) -> IntShapes {
        let mut binding = self.links.filter(overlay_rule);
        let visited = binding.as_mut_slice();
        let mut shapes = Vec::new();
        let mut holes = Vec::new();

        let mut link_index = 0;
        while link_index < visited.len() {
            if visited.is_visited(link_index) {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_link(link_index, visited);
            let link = self.link(left_top_link);
            let is_hole = overlay_rule.is_fill_top(link.fill);

            let start_data = StartPathData::new(is_hole, link, left_top_link);

            let mut path = self.get_path(&start_data, visited);

            if path.validate(min_area) {
                if is_hole {
                    holes.push(path);
                } else {
                    shapes.push(vec![path]);
                }
            }
        }

        shapes.join_sorted_holes(&self.solver, holes);

        shapes
    }

    #[inline]
    fn get_path(&self, start_data: &StartPathData, visited: &mut [bool]) -> IntPath {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        let mut path = IntPath::new();
        path.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            let node = self.node(node_id);
            link_id = match node {
                OverlayNode::Bridge(bridge) => {
                    if bridge[0] == link_id { bridge[1] } else { bridge[0] }
                }
                OverlayNode::Cross(indices) => {
                    self.find_nearest_counter_wise_link_to(link_id, node_id, indices, visited)
                }
            };

            let link = self.link(link_id);
            node_id = if link.a.id == node_id {
                path.push(link.a.point);
                link.b.id
            } else {
                path.push(link.b.point);
                link.a.id
            };

            visited.visit(link_id);
        }

        path
    }

    #[inline]
    pub(crate) fn find_nearest_counter_wise_link_to(
        &self,
        target_index: usize,
        node_id: usize,
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
        } else { (target.b.point, target.a.point) };

        // more the one vectors
        let b = self.link(first_index).other(node_id).point;
        let mut vector_solver = NearestCCWVector::new(c, a, b, first_index);

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
            OverlayNode::Bridge(bridge) => {
                self.find_left_top_link_on_bridge(bridge)
            }
            OverlayNode::Cross(indices) => {
                self.find_left_top_link_on_indices(top, link_index, indices, visited)
            }
        }
    }

    #[inline(always)]
    fn find_left_top_link_on_indices(&self, link: &OverlayLink, link_index: usize, indices: &[usize], visited: &[bool]) -> usize {
        let mut top_index = link_index;
        let mut top = link;

        // find most top link

        for &i in indices.iter() {
            if i == link_index {
                continue;
            }
            let link = self.link(i);
            if !link.is_direct() || Triangle::is_clockwise_point(top.a.point, top.b.point, link.b.point) {
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
    pub(crate) fn new(is_hole: bool, link: &OverlayLink, link_id: usize) -> Self {
        if is_hole {
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
    fn validate(&mut self, min_area: usize) -> bool;
}

impl Validate for IntPath {
    #[inline]
    fn validate(&mut self, min_area: usize) -> bool {
        let slice = self.as_slice();
        if !slice.is_simple() {
            let simple = slice.to_simple();
            let _ = std::mem::replace(self, simple);
        }

        if self.len() < 3 {
            return false;
        }

        if min_area == 0 {
            return true;
        }

        let area = self.unsafe_area();
        let abs_area = area.unsigned_abs() as usize >> 1;

        abs_area < min_area
    }
}

trait Visit {
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