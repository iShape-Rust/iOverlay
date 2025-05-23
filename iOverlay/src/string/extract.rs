use alloc::vec;
use alloc::vec::Vec;
use crate::bind::solver::JoinHoles;
use crate::core::nearest_vector::NearestVector;
use crate::core::overlay::{ContourDirection, IntOverlayOptions};
use crate::segm::segment::SUBJ_TOP;
use crate::string::graph::StringGraph;
use crate::string::rule::StringRule;
use crate::string::split::{BinStore, Split};
use i_shape::int::path::{ContourExtension, IntPath};
use i_shape::int::shape::IntShapes;

impl StringGraph<'_> {
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
            Default::default(),
        )
    }

    /// Extracts shapes from the graph with a minimum area constraint.
    /// - `string_rule`: The rule used to determine how shapes are extracted.
    /// - `main_direction`: Winding direction for the **output** main (outer) contour. All hole contours will automatically use the opposite direction. Impact on **output** only!
    /// - `min_area`: The minimum area that a shape must have to be included in the results. Shapes smaller than this will be excluded.
    /// - Returns: A vector of `IntShape`, representing the geometric result of the applied overlay rule.
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
        options: IntOverlayOptions,
    ) -> IntShapes {
        let clockwise = options.output_direction == ContourDirection::Clockwise;
        let mut fills = self.filter(string_rule);
        let mut shapes= Vec::new();
        let mut holes= Vec::new();

        let mut contour_buffer = Vec::new();
        let mut bin_store = BinStore::new();

        let mut link_index = 0;
        while link_index < fills.len() {
            let fill = fills[link_index];
            if fill == 0 {
                link_index += 1;
                continue;
            }

            let direction = fill & SUBJ_TOP == SUBJ_TOP;
            let paths = self
                .get_paths(link_index, direction, &mut fills)
                .split_loops(options.min_output_area, &mut contour_buffer, &mut bin_store);

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

        shapes.join_unsorted_holes(holes, clockwise);

        shapes
    }

    #[inline]
    fn get_paths(&self, start_index: usize, clockwise: bool, fills: &mut [u8]) -> IntPath {
        let start_link = self.link(start_index);

        let mut link_id = start_index;
        let mut node_id = start_link.b.id;
        let last_node_id = start_link.a.id;

        let mut path = IntPath::new();
        path.push(start_link.a.point);

        fills[start_index] = start_link.visit_fill(fills[start_index], start_link.a.id, clockwise);

        // Find a closed tour
        while node_id != last_node_id {

            link_id = self.find_nearest_link_to(link_id, node_id, clockwise, fills);

            let link = self.link(link_id);
            fills[link_id] = link.visit_fill(fills[link_id], node_id, clockwise);

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

    fn find_nearest_link_to(
        &self,
        target_index: usize,
        node_id: usize,
        clockwise: bool,
        fills: &[u8],
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
            if self.link(link_index).is_move_possible(fills[link_index], node_id, clockwise) {
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
            if self.link(target_index).is_move_possible(fills[target_index], node_id, clockwise) {
                return target_index;
            } else {
                panic!("no move found")
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
            if self.link(link_index).is_move_possible(fills[link_index], node_id, clockwise) {
                let p = self.link(link_index).other(node_id).point;
                vector_solver.add(p, link_index);
            }
        }

        vector_solver.best_id
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use crate::core::fill_rule::FillRule;
    use crate::string::slice::IntSlice;
    use i_float::int::point::IntPoint;
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
        let graph = overlay.build_graph_view(FillRule::NonZero).unwrap();

        let r = graph.extract_shapes_custom(
            StringRule::Slice,
            Default::default(),
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
