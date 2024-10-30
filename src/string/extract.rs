use i_float::triangle::Triangle;
use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShapes;
use crate::bind::solver::JoinHoles;
use crate::core::extract::StartPathData;
use crate::core::vector_rotation::NearestCCWVector;
use crate::string::rule::StringRule;
use crate::string::split::Split;
use crate::string::graph::StringGraph;

impl StringGraph {
    /// Extracts shapes from the graph based on the specified `StringRule`.
    /// - `string_rule`: The rule used to determine how shapes are extracted.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    #[inline(always)]
    pub fn extract_shapes(&self, string_rule: StringRule) -> IntShapes {
        self.extract_shapes_min_area(string_rule, 0)
    }

    /// Extracts shapes from the graph with a minimum area constraint.
    /// - `string_rule`: The rule used to determine how shapes are extracted.
    /// - `min_area`: The minimum area that a shape must have to be included in the results. Shapes smaller than this will be excluded.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a clockwise order, and holes have a counterclockwise order.
    pub fn extract_shapes_min_area(&self, string_rule: StringRule, min_area: usize) -> IntShapes {
        let mut binding = self.filter(string_rule);
        let visited = binding.as_mut_slice();
        let mut shapes = Vec::new();

        let mut link_index = 0;
        while link_index < visited.len() {
            if visited.is_visited(link_index) {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_link(link_index, visited);
            let link = self.link(left_top_link);

            if visited.count(left_top_link) == 1 {
                let is_hole = string_rule.is_hole(link.fill);
                let start_data = StartPathData::new(is_hole, link, left_top_link);
                let paths = self.get_path(&start_data, visited).split_loops(min_area);
                if is_hole {
                    shapes.join_unsorted_holes(&self.solver, paths);
                } else {
                    for path in paths.into_iter() {
                        shapes.push(vec![path]);
                    }
                }
            } else {
                let start_data = StartPathData::new(false, link, left_top_link);
                let (paths, track) = self.get_path_and_track(&start_data, visited);
                for index in track.into_iter() {
                    visited.reset(index);
                }
                if paths.is_empty() {
                    continue;
                }

                let paths = paths.split_loops(min_area);
                let mut holes = paths.clone();
                for hole in holes.iter_mut() {
                    hole.reverse();
                }

                // add as shapes
                shapes.join_unsorted_holes(&self.solver, holes);

                // add as body
                for path in paths.into_iter() {
                    shapes.push(vec![path]);
                }
            }
        }

        shapes
    }

    #[inline]
    fn get_path(&self, start_data: &StartPathData, visited: &mut [u8]) -> IntPath {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        let mut path = IntPath::new();
        path.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            link_id = self.find_nearest_counter_wise_link_to(link_id, node_id, visited);

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
    fn get_path_and_track(&self, start_data: &StartPathData, visited: &mut [u8]) -> (IntPath, Vec<usize>) {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;
        let mut track = Vec::new();
        visited.visit(link_id);
        track.push(link_id);

        let mut path = IntPath::new();
        path.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            link_id = self.find_nearest_counter_wise_link_to(link_id, node_id, visited);

            let link = self.link(link_id);
            node_id = if link.a.id == node_id {
                path.push(link.a.point);
                link.b.id
            } else {
                path.push(link.b.point);
                link.a.id
            };

            visited.visit(link_id);
            track.push(link_id);
        }

        (path, track)
    }

    #[inline]
    pub(crate) fn find_left_top_link(&self, link_index: usize, visited: &[u8]) -> usize {
        let mut top = self.link(link_index);
        let mut top_index = link_index;
        let node = self.node(top.a.id);

        debug_assert!(top.is_direct());

        // find most top link

        for &i in node.iter() {
            if i == link_index {
                continue;
            }
            let link = self.link(i);
            if !link.is_direct() || Triangle::is_clockwise_point(top.a.point, top.b.point, link.b.point) {
                continue;
            }

            if visited.count(i) == 0 {
                continue;
            }

            top_index = i;
            top = link;
        }

        top_index
    }

    pub(crate) fn find_nearest_counter_wise_link_to(
        &self,
        target_index: usize,
        node_id: usize,
        visited: &[u8],
    ) -> usize {
        let indices = self.node(node_id);

        let mut is_first = true;
        let mut first_index = 0;
        let mut second_index = usize::MAX;
        let mut pos = 0;
        for (i, &link_index) in indices.iter().enumerate() {
            let is_target = link_index == target_index;
            if visited.is_not_visited(link_index) {
                if is_first {
                    first_index = link_index;
                    is_first = is_target; // skip target
                } else if !is_target {
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
}

trait Visit {
    fn count(&self, index: usize) -> u8;
    fn is_visited(&self, index: usize) -> bool;
    fn is_not_visited(&self, index: usize) -> bool;
    fn visit(&mut self, index: usize);
    fn reset(&mut self, index: usize);
}

impl Visit for [u8] {
    #[inline(always)]
    fn count(&self, index: usize) -> u8 {
        unsafe { *self.get_unchecked(index) }
    }

    #[inline(always)]
    fn is_visited(&self, index: usize) -> bool {
        self.count(index) == 0
    }

    #[inline(always)]
    fn is_not_visited(&self, index: usize) -> bool {
        self.count(index) > 0
    }

    #[inline(always)]
    fn visit(&mut self, index: usize) {
        unsafe { *self.get_unchecked_mut(index) -= 1 }
    }

    #[inline(always)]
    fn reset(&mut self, index: usize) {
        unsafe { *self.get_unchecked_mut(index) = 0 }
    }
}