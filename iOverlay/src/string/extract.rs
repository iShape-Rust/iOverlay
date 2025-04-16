use crate::bind::solver::JoinHoles;
use crate::core::extract::StartPathData;
use crate::core::link::OverlayLink;
use crate::core::nearest_vector::NearestVector;
use crate::core::overlay::ContourDirection;
use crate::segm::segment::{SUBJ_BOTH, SUBJ_BOTTOM, SUBJ_TOP};
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;
use crate::string::split::Split;
use i_float::triangle::Triangle;
use i_shape::int::path::{IntPath, PointPathExtension};
use i_shape::int::shape::{IntContour, IntShape, IntShapes};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliceResultType {
    All,         // default: carry all holes even if untouched
    TouchedOnly, // discard holes not intersected by shape contours
}

impl StringGraph {
    /// Extracts shapes from the graph based on the specified `StringRule`.
    /// - `string_rule`: The rule used to determine how shapes are extracted.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    #[inline(always)]
    pub fn extract_shapes(&self, string_rule: StringRule) -> IntShapes {
        self.extract_shapes_custom(
            string_rule,
            ContourDirection::CounterClockwise,
            SliceResultType::All,
            0,
        )
    }

    /// Extracts shapes from the graph with a minimum area constraint.
    /// - `string_rule`: The rule used to determine how shapes are extracted.
    /// - `main_direction`: Winding direction for the **output** main (outer) contour. All hole contours will automatically use the opposite direction. Impact on **output** only!
    /// - `result_type`: What to include in a result.
    /// - `min_area`: The minimum area that a shape must have to be included in the results. Shapes smaller than this will be excluded.
    /// # Shape Representation
    /// The output is a `IntShapes`, where:
    /// - The outer `Vec<IntShape>` represents a set of shapes.
    /// - Each shape `Vec<IntContour>` represents a collection of contours, where the first contour is the outer boundary, and all subsequent contours are holes in this boundary.
    /// - Each path `Vec<IntPoint>` is a sequence of points, forming a closed path.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    pub fn extract_shapes_custom(
        &self,
        string_rule: StringRule,
        main_direction: ContourDirection,
        result_type: SliceResultType,
        min_area: usize,
    ) -> IntShapes {
        let clockwise = main_direction == ContourDirection::Clockwise;
        let mut binding = self.filter(string_rule);
        let mut shapes = Vec::new();
        let mut holes = Vec::new();

        self.extract_touching_contours(
            string_rule,
            clockwise,
            min_area,
            &mut binding,
            &mut shapes,
            &mut holes,
        );

        if result_type == SliceResultType::All {
            self.extract_independent_contours(
                clockwise,
                min_area,
                &mut binding,
                &mut shapes,
                &mut holes,
            );
        }

        shapes.join_unsorted_holes(&self.solver, holes);

        shapes
    }

    fn extract_touching_contours(
        &self,
        string_rule: StringRule,
        clockwise: bool,
        min_area: usize,
        visited: &mut [u8],
        shapes: &mut Vec<IntShape>,
        holes: &mut Vec<IntContour>,
    ) {
        let mut link_index = 0;
        while link_index < visited.len() {
            if visited.is_visited(link_index) {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_subj_link(link_index, visited);
            if left_top_link == usize::MAX {
                link_index += 1;
                continue;
            }
            let link = self.link(link_index);

            let is_hole = string_rule.is_hole(link.fill);
            let direction = is_hole == clockwise;
            let start_data = StartPathData::new(direction, link, left_top_link);
            let paths = self
                .get_path(&start_data, direction, visited)
                .split_loops(min_area);
            if is_hole {
                holes.extend_from_slice(&paths);
            } else {
                for path in paths.into_iter() {
                    shapes.push(vec![path]);
                }
            }
        }
    }

    fn extract_independent_contours(
        &self,
        clockwise: bool,
        min_area: usize,
        visited: &mut [u8],
        shapes: &mut Vec<IntShape>,
        holes: &mut Vec<IntContour>,
    ) {
        let mut link_index = 0;
        while link_index < visited.len() {
            if visited.count(link_index) != 2 {
                link_index += 1;
                continue;
            }

            let link = self.link(link_index);
            let start_data = StartPathData::new(false, link, link_index);
            let paths = self
                .get_path(&start_data, false, visited)
                .split_loops(min_area);

            for path in paths.into_iter() {
                let order = path.is_clockwise_ordered();
                let reversed = path.clone().into_reversed();
                if clockwise == order {
                    shapes.push(vec![path]);
                    holes.push(reversed);
                } else {
                    shapes.push(vec![reversed]);
                    holes.push(path);
                }
            }
        }
    }

    #[inline]
    fn get_path(&self, start_data: &StartPathData, clockwise: bool, visited: &mut [u8]) -> IntPath {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        let mut path = IntPath::new();
        path.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            link_id = self.find_nearest_link_to(link_id, node_id, clockwise, visited);

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
    pub(crate) fn find_left_top_subj_link(&self, link_index: usize, visited: &[u8]) -> usize {
        let mut top = self.link(link_index);
        let mut top_index = if top.is_sub_edge() {
            link_index
        } else {
            usize::MAX
        };
        let node = self.node(top.a.id);

        debug_assert!(top.is_direct());

        // find most top subj link

        for &i in node.iter() {
            if i == link_index {
                continue;
            }
            let link = self.link(i);
            if !link.is_direct()
                || !link.is_sub_edge()
                || Triangle::is_clockwise_point(top.a.point, top.b.point, link.b.point)
            {
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

    pub(crate) fn find_nearest_link_to(
        &self,
        target_index: usize,
        node_id: usize,
        clockwise: bool,
        visited: &[u8],
    ) -> usize {
        let indices = self.node(node_id);
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
}

impl OverlayLink {
    #[inline]
    fn is_sub_edge(&self) -> bool {
        let subj = self.fill & SUBJ_BOTH;
        subj == SUBJ_TOP || subj == SUBJ_BOTTOM
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

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::string::slice::IntSlice;
    use i_float::int::point::IntPoint;

    #[test]
    fn test_0() {
        let paths = [[
            IntPoint::new(-10, 10),
            IntPoint::new(-10, -10),
            IntPoint::new(10, -10),
            IntPoint::new(10, 10),
        ]
        .to_vec()]
        .to_vec();

        let result = paths.slice_by_line(
            [IntPoint::new(-20, 0), IntPoint::new(20, 0)],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 4);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 4);
    }

    #[test]
    fn test_1() {
        let paths = [
            [
                IntPoint::new(-10, 10),
                IntPoint::new(-10, -10),
                IntPoint::new(10, -10),
                IntPoint::new(10, 10),
            ]
            .to_vec(),
            [
                IntPoint::new(-5, -5),
                IntPoint::new(-5, 5),
                IntPoint::new(5, 5),
                IntPoint::new(5, -5),
            ]
            .to_vec(),
        ]
        .to_vec();

        let result = paths.slice_by_line(
            [IntPoint::new(-20, 0), IntPoint::new(20, 0)],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0].len(), 8);
        assert_eq!(result[1].len(), 1);
        assert_eq!(result[1][0].len(), 8);
    }

    #[test]
    fn test_2() {
        let paths = [
            [
                IntPoint::new(0, 0),
                IntPoint::new(35, 0),
                IntPoint::new(35, 35),
                IntPoint::new(0, 35),
            ]
            .to_vec(),
            [
                IntPoint::new(5, 5),
                IntPoint::new(5, 15),
                IntPoint::new(15, 15),
                IntPoint::new(15, 5),
            ]
            .to_vec(),
            [
                IntPoint::new(20, 5),
                IntPoint::new(20, 15),
                IntPoint::new(30, 15),
                IntPoint::new(30, 5),
            ]
            .to_vec(),
            [
                IntPoint::new(5, 20),
                IntPoint::new(5, 30),
                IntPoint::new(15, 30),
                IntPoint::new(15, 20),
            ]
            .to_vec(),
            [
                IntPoint::new(20, 20),
                IntPoint::new(20, 30),
                IntPoint::new(30, 30),
                IntPoint::new(30, 20),
            ]
            .to_vec(),
        ]
        .to_vec();

        let result = paths.slice_by_lines(
            &vec![
                [IntPoint::new(10, 15), IntPoint::new(10, 20)],
                [IntPoint::new(25, 15), IntPoint::new(25, 20)],
                [IntPoint::new(15, 10), IntPoint::new(20, 10)],
                [IntPoint::new(15, 25), IntPoint::new(20, 25)],
            ],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }
}
