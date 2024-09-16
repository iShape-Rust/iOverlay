use i_float::point::IntPoint;
use i_float::triangle::Triangle;
use i_shape::int::path::{IntPath, PointPathExtension};
use i_shape::int::shape::{IntShape, IntShapes};
use i_shape::int::simple::Simple;
use crate::bind::segment::IdSegments;
use crate::bind::solver::ShapeBinder;
use crate::id_point::IdPoint;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::solver::Solver;
use crate::sort::SmartSort;

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
    pub fn extract_shapes_min_area(&self, overlay_rule: OverlayRule, min_area: i64) -> IntShapes {
        let mut visited = self.links.filter(overlay_rule);

        let mut holes = Vec::new();
        let mut shapes = Vec::new();

        let mut link_index = 0;
        while link_index < visited.len() {
            let &is_visited = unsafe { visited.get_unchecked(link_index) };
            if is_visited {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_link(overlay_rule, link_index, &visited);
            let mut path = self.get_path(&left_top_link, &mut visited);
            if path.validate(min_area) {
                if left_top_link.is_hole {
                    holes.push(path);
                } else {
                    shapes.push(vec![path]);
                }
            }

            link_index += 1;
        }

        shapes.join(&self.solver, holes);

        shapes
    }

    fn get_path(&self, left_top_link: &StartPathData, visited: &mut [bool]) -> IntPath {
        let mut node_id = left_top_link.next_node_id;
        let mut link_id = left_top_link.link_id;

        *unsafe { visited.get_unchecked_mut(link_id) } = true;

        let mut path = IntPath::new();
        path.push(left_top_link.point);

        // Find a closed tour
        while node_id != left_top_link.last_node_id {
            let node = self.node(node_id);
            if node.indices.len() == 2 {
                link_id = node.other(link_id);
            } else {
                link_id = self.find_nearest_counter_wise_link_to(link_id, node_id, visited);
            }
            let link = self.link(link_id);
            node_id = if link.a.id == node_id {
                path.push(link.a.point);
                link.b.id
            } else {
                path.push(link.b.point);
                link.a.id
            };

            *unsafe { visited.get_unchecked_mut(link_id) } = true;
        }

        path
    }

    pub(crate) fn find_first_link(&self, node_index: usize, visited: &Vec<bool>) -> Option<usize> {
        let node = unsafe { self.nodes.get_unchecked(node_index) };

        let mut iter = node.indices.iter();

        let mut j = if let Some(index) = iter
            .find(|&&i| {
                let is_visited = unsafe { *visited.get_unchecked(i) };
                !is_visited
            }) {
            *index
        } else {
            return None;
        };

        for &i in iter {
            let is_visited = unsafe { *visited.get_unchecked(i) };
            if is_visited {
                continue;
            }

            let (a, bi, bj) = unsafe {
                let link = self.links.get_unchecked(j);
                let bi = self.links.get_unchecked(i).b.point;
                (link.a.point, bi, link.b.point)
            };
            if Triangle::is_clockwise_point(a, bi, bj) {
                j = i;
            }
        }

        Some(j)
    }
}

pub(crate) struct StartPathData {
    pub(crate) next_node_id: usize,
    pub(crate) last_node_id: usize,
    pub(crate) link_id: usize,
    pub(crate) point: IntPoint,
    pub(crate) is_hole: bool,
}

impl OverlayGraph {
    #[inline(always)]
    pub(crate) fn find_left_top_link(&self, overlay_rule: OverlayRule, link_index: usize, visited: &[bool]) -> StartPathData {
        let mut top_index = link_index;
        let mut top = self.link(link_index);
        debug_assert!(top.is_direct());

        let node = self.node(top.a.id);

        // find most top link

        for &i in node.indices.iter() {
            if i == link_index {
                continue;
            }
            let link = self.link(i);
            if !link.is_direct() || Triangle::is_clockwise_point(top.a.point, top.b.point, link.b.point) {
                continue;
            }

            let &is_visited = unsafe { visited.get_unchecked(i) };
            if is_visited {
                continue;
            }

            top_index = i;
            top = link;
        }

        let is_hole = overlay_rule.is_fill_top(top.fill);

        if is_hole {
            StartPathData {
                next_node_id: top.a.id,
                last_node_id: top.b.id,
                link_id: top_index,
                point: top.b.point,
                is_hole: true,

            }
        } else {
            StartPathData {
                next_node_id: top.b.id,
                last_node_id: top.a.id,
                link_id: top_index,
                point: top.a.point,
                is_hole: false,
            }
        }
    }
}

trait JoinHoles {
    fn join(&mut self, solver: &Solver, holes: Vec<IntPath>);
    fn scan_join(&mut self, solver: &Solver, holes: Vec<IntPath>);
}

impl JoinHoles for Vec<IntShape> {
    #[inline]
    fn join(&mut self, solver: &Solver, holes: Vec<IntPath>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].reserve_exact(holes.len());
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
        } else {
            self.scan_join(solver, holes);
        }
    }

    fn scan_join(&mut self, solver: &Solver, holes: Vec<IntPath>) {
        let mut i_points: Vec<_> = holes.iter().enumerate()
            .map(|(i, path)| IdPoint::new(i, path.first().unwrap().clone()))
            .collect();

        i_points.smart_sort_by(solver, |a, b| a.point.x.cmp(&b.point.x));

        let x_min = i_points[0].point.x;
        let x_max = i_points[i_points.len() - 1].point.x;

        let capacity = self.iter().fold(0, |s, it| s + it[0].len()) / 2;
        let mut segments = Vec::with_capacity(capacity);
        for (i, shape) in self.iter().enumerate() {
            shape[0].append_id_segments(&mut segments, i, x_min, x_max);
        }

        segments.smart_sort_by(solver, |a, b| a.x_segment.a.x.cmp(&b.x_segment.a.x));

        let solution = ShapeBinder::bind(self.len(), i_points, segments);

        for shape_index in 0..solution.children_count_for_parent.len() {
            let capacity = solution.children_count_for_parent[shape_index];
            self[shape_index].reserve_exact(capacity);
        }

        let mut hole_index = 0;
        for hole in holes.into_iter() {
            let shape_index = solution.parent_for_child[hole_index];
            self[shape_index].push(hole);
            hole_index += 1;
        }
    }
}

trait Validate {
    fn validate(&mut self, min_area: i64) -> bool;
}

impl Validate for IntPath {
    fn validate(&mut self, min_area: i64) -> bool {
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
        let abs_area = area.abs() >> 1;

        abs_area < min_area
    }
}