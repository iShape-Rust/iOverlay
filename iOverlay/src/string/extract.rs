use crate::bind::solver::JoinHoles;
use crate::core::nearest_vector::NearestVector;
use crate::core::overlay::ContourDirection;
use crate::segm::segment::{SUBJ_BOTTOM, SUBJ_TOP};
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;
use crate::string::split::Split;
use i_shape::int::path::{IntPath, PointPathExtension};
use i_shape::int::shape::IntShapes;
use crate::string::filter::NavigationLink;

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
            0,
        )
    }

    /// Extracts shapes from the graph with a minimum area constraint.
    /// - `string_rule`: The rule used to determine how shapes are extracted.
    /// - `main_direction`: Winding direction for the **output** main (outer) contour. All hole contours will automatically use the opposite direction. Impact on **output** only!
    /// - `min_area`: The minimum area that a shape must have to be included in the results. Shapes smaller than this will be excluded.
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
        min_area: usize,
    ) -> IntShapes {
        let clockwise = main_direction == ContourDirection::Clockwise;
        let mut nav_links = self.filter(string_rule);
        let mut shapes= Vec::new();
        let mut holes= Vec::new();

        let mut link_index = 0;
        while link_index < nav_links.len() {
            let link = &nav_links[link_index];
            if link.fill == 0 {
                link_index += 1;
                continue;
            }

            let direction = link.fill & SUBJ_TOP == SUBJ_TOP;
            let paths = self
                .get_paths(link_index, direction, &mut nav_links)
                .split_loops(min_area);

            for mut path in paths.into_iter() {
                let order = path.is_clockwise_ordered();
                let is_hole = order == direction;
                if is_hole {
                    if clockwise == order { // clockwise == direction
                        path.reverse();
                    }
                    holes.push(path);
                } else {
                    if clockwise != order {
                        path.reverse();
                    }
                    shapes.push(vec![path]);
                }
            }
        }

        shapes.join_unsorted_holes(&self.solver, holes);

        shapes
    }

    #[inline]
    fn get_paths(&self, start_index: usize, clockwise: bool, nav_links: &mut [NavigationLink]) -> IntPath {
        let start_link = &mut nav_links[start_index];

        let mut link_id = start_index;
        let mut node_id = start_link.b.id;
        let last_node_id = start_link.a.id;

        let mut path = IntPath::new();
        path.push(start_link.a.point);

        start_link.visit(start_link.a.id, clockwise);

        // Find a closed tour
        while node_id != last_node_id {

            link_id = self.find_nearest_link_to(link_id, node_id, clockwise, nav_links);

            let link = &mut nav_links[link_id];
            link.visit(node_id, clockwise);

            node_id = if link.a.id == node_id {
                path.push(link.a.point);
                link.b.id
            } else {
                path.push(link.b.point);
                link.a.id
            };
        }

        path
    }

    pub(crate) fn find_nearest_link_to(
        &self,
        target_index: usize,
        node_id: usize,
        clockwise: bool,
        nav_links: &[NavigationLink],
    ) -> usize {
        let indices = self.node(node_id);
        let mut is_first = true;
        let mut first_index = usize::MAX;
        let mut second_index = usize::MAX;
        let mut pos = 0;
        for (i, &link_index) in indices.iter().enumerate() {
            if link_index == target_index {
                continue;
            }
            if nav_links[link_index].is_move_possible(node_id, clockwise) {
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

        if first_index == usize::MAX {
            if nav_links[target_index].is_move_possible(node_id, clockwise) {
                return target_index;
            } else {
                panic!("no move found")
            }
        }

        if second_index == usize::MAX {
            return first_index;
        }

        let target = &nav_links[target_index];
        let (c, a) = if target.a.id == node_id {
            (target.a.point, target.b.point)
        } else {
            (target.b.point, target.a.point)
        };

        // more the one vectors
        let b = nav_links[first_index].other(node_id).point;
        let mut vector_solver = NearestVector::new(c, a, b, first_index, clockwise);

        // add second vector
        vector_solver.add(nav_links[second_index].other(node_id).point, second_index);

        // check the rest vectors
        for &link_index in indices.iter().skip(pos + 1) {
            if nav_links[link_index].is_move_possible(node_id, clockwise) {
                let p = nav_links[link_index].other(node_id).point;
                vector_solver.add(p, link_index);
            }
        }

        vector_solver.best_id
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fill_rule::FillRule;
    use crate::string::slice::IntSlice;
    use i_float::int::point::IntPoint;
    use crate::core::overlay::ContourDirection;
    use crate::string::overlay::StringOverlay;
    use crate::string::rule::StringRule;

    #[test]
    fn test_0() {
        let paths = vec![vec![
            IntPoint::new(-10, 10),
            IntPoint::new(-10, -10),
            IntPoint::new(10, -10),
            IntPoint::new(10, 10),
        ]];

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
        let paths = vec![
            vec![
                IntPoint::new(-10, 10),
                IntPoint::new(-10, -10),
                IntPoint::new(10, -10),
                IntPoint::new(10, 10),
            ],
            vec![
                IntPoint::new(-5, -5),
                IntPoint::new(-5, 5),
                IntPoint::new(5, 5),
                IntPoint::new(5, -5),
            ],
        ];

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
        let paths = vec![
            vec![
                IntPoint::new(-10, 10),
                IntPoint::new(-10, -10),
                IntPoint::new(10, -10),
                IntPoint::new(10, 10),
            ],
        ];

        let window = vec![
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 5),
            IntPoint::new(5, 5),
            IntPoint::new(5, -5),
        ];

        let mut overlay = StringOverlay::with_shape(&paths);
        overlay.add_string_contour(&window);
        let graph = overlay.into_graph(FillRule::NonZero);

        let r = graph.extract_shapes_custom(
            StringRule::Slice,
            ContourDirection::CounterClockwise,
            0,
        );

        assert_eq!(r.len(), 2);
    }

    #[test]
    fn test_3() {
        let paths = vec![
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(35, 0),
                IntPoint::new(35, 20),
                IntPoint::new(0, 20),
            ],
            vec![
                IntPoint::new(5, 5),
                IntPoint::new(5, 15),
                IntPoint::new(15, 15),
                IntPoint::new(15, 5),
            ],
            vec![
                IntPoint::new(20, 5),
                IntPoint::new(20, 15),
                IntPoint::new(30, 15),
                IntPoint::new(30, 5),
            ],
        ];

        let result = paths.slice_by_line(
            [IntPoint::new(15, 10), IntPoint::new(20, 10)],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn test_4() {
        let paths = vec![
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(35, 0),
                IntPoint::new(35, 20),
                IntPoint::new(0, 20),
            ],
            vec![
                IntPoint::new(5, 5),
                IntPoint::new(5, 15),
                IntPoint::new(15, 15),
                IntPoint::new(15, 5),
            ],
            vec![
                IntPoint::new(20, 5),
                IntPoint::new(20, 15),
                IntPoint::new(30, 15),
                IntPoint::new(30, 5),
            ],
        ];

        let result = paths.slice_by_lines(
            &vec![
                [IntPoint::new(15, 5), IntPoint::new(20, 5)],
                [IntPoint::new(15, 15), IntPoint::new(20, 15)],
            ],
            FillRule::NonZero,
        );

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 1);
    }

    #[test]
    fn test_5() {
        let paths = vec![
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(35, 0),
                IntPoint::new(35, 35),
                IntPoint::new(0, 35),
            ],
            vec![
                IntPoint::new(5, 5),
                IntPoint::new(5, 15),
                IntPoint::new(15, 15),
                IntPoint::new(15, 5),
            ],
            vec![
                IntPoint::new(20, 5),
                IntPoint::new(20, 15),
                IntPoint::new(30, 15),
                IntPoint::new(30, 5),
            ],
            vec![
                IntPoint::new(5, 20),
                IntPoint::new(5, 30),
                IntPoint::new(15, 30),
                IntPoint::new(15, 20),
            ],
            vec![
                IntPoint::new(20, 20),
                IntPoint::new(20, 30),
                IntPoint::new(30, 30),
                IntPoint::new(30, 20),
            ],
        ];

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
