use i_float::fix_float::{FIX_FRACTION_BITS, FixFloat};
use i_shape::fix_path::{FixPath, FixPathExtension};
use i_float::point::Point;
use i_shape::fix_shape::FixShape;
use crate::geom::floor::Floors;
use crate::geom::holes_solver::HolesSolver;
use crate::geom::id_point::IdPoint;
use crate::geom::x_order::XOrder;
use crate::index::EMPTY_INDEX;
use crate::layout::overlay_graph::OverlayGraph;
use crate::space::line_range::LineRange;

use super::overlay_rule::OverlayRule;
use super::filter::Filter;

impl OverlayGraph {

    /// Extracts shapes from the overlay graph based on the specified overlay rule. This method is used to retrieve the final geometric shapes after boolean operations have been applied. It's suitable for most use cases where the minimum area of shapes is not a concern.
    /// - `overlay_rule`: The boolean operation rule to apply when extracting shapes from the graph, such as union or intersection.
    /// - Returns: A vector of `FixShape`, representing the geometric result of the applied overlay rule.
    pub fn extract_shapes(&self, overlay_rule: OverlayRule) -> Vec<FixShape> {
        self.extract_shapes_min_area(overlay_rule, 0)
    }

    /// Extracts shapes from the overlay graph similar to `extract_shapes`, but with an additional constraint on the minimum area of the shapes. This is useful for filtering out shapes that do not meet a certain size threshold, which can be beneficial for eliminating artifacts or noise from the output.
    /// - `overlay_rule`: The boolean operation rule to apply, determining how shapes are combined or subtracted.
    /// - `min_area`: The minimum area threshold for shapes to be included in the result. Shapes with an area smaller than this value will be excluded.
    /// - Returns: A vector of `FixShape` that meet the specified area criteria, representing the cleaned-up geometric result.
    pub fn extract_shapes_min_area(&self, overlay_rule: OverlayRule, min_area: FixFloat) -> Vec<FixShape> {
        let mut visited = self.links.filter(overlay_rule);

        let mut holes = Vec::new();
        let mut shapes = Vec::new();

        let mut j = 0;
        // nodes array is sorted by link.a
        while j < self.nodes.len() {
            let i = self.find_first_link(j, &visited);
            if i == EMPTY_INDEX {
                j += 1;
            } else {
                let is_hole = overlay_rule.is_fill_top(self.links[i].fill);
                let mut path = self.get_vector_path(overlay_rule, i, &mut visited);
                if path.validate(min_area, is_hole) {
                    if is_hole {
                        holes.push(path);
                    } else {
                        shapes.push(FixShape { paths: [path].to_vec() });
                    }
                }
            }
        }

        shapes.join(holes);

        shapes
    }

    fn get_vector_path(&self, overlay_rule: OverlayRule, index: usize, visited: &mut Vec<bool>) -> FixPath {
        let mut path = FixPath::new();
        let mut next = index;

        let mut link = self.links[index];

        let mut a = link.a;
        let mut b = link.b;

        // Find a closed tour
        loop {
            path.push(a.point);
            let node = &self.nodes[b.index];

            if node.indices.len() == 2 {
                next = node.other(next);
            } else {
                let is_fill_top = overlay_rule.is_fill_top(link.fill);
                let is_cw = Self::is_clockwise(a.point, b.point, is_fill_top);
                next = self.find_nearest_link_to(a, b, next, is_cw, visited);
            }

            link = self.links[next];
            a = b;
            b = link.other(b);

            visited[next] = true;

            if next == index {
                break;
            }
        }

        visited[index] = true;

        path
    }
}

trait JoinHoles {
    fn join(&mut self, holes: Vec<FixPath>);
    fn scan_join(&mut self, holes: Vec<FixPath>);
}

impl JoinHoles for Vec<FixShape> {
    fn join(&mut self, holes: Vec<FixPath>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].paths.reserve_exact(holes.len());
            let mut hole_paths = holes;
            self[0].paths.append(&mut hole_paths);
        } else {
            self.scan_join(holes);
        }
    }

    fn scan_join(&mut self, holes: Vec<FixPath>) {
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut i_points = Vec::with_capacity(holes.len());
        for i in 0..holes.len() {
            let p = holes[i][0];
            let x = p.x as i32;
            let y = p.y as i32;
            i_points.push(IdPoint::new(i, Point::new(x, y)));
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
        i_points.sort_by(|a, b| a.point.order_by_x(&b.point));

        let x_min = i_points[0].point.x;
        let x_max = i_points[i_points.len() - 1].point.x;

        let mut floors = Vec::new();
        for i in 0..self.len() {
            let mut hole_floors = self[i].contour().floors(i, x_min, x_max, &mut y_min, &mut y_max);
            floors.append(&mut hole_floors);
        }

        floors.sort_by(|a, b| a.seg.a.order_by_x(&b.seg.a));

        let y_range = LineRange { min: y_min, max: y_max };
        let solution = HolesSolver::solve(self.len(), y_range, i_points, floors);

        for shape_index in 0..solution.hole_counter.len() {
            let capacity = solution.hole_counter[shape_index];
            self[shape_index].paths.reserve_exact(capacity);
        }

        let mut hole_index = 0;
        for hole in holes.into_iter() {
            let shape_index = solution.hole_shape[hole_index];
            self[shape_index].paths.push(hole);
            hole_index += 1;
        }
    }
}

trait Validate {
    fn validate(&mut self, min_area: FixFloat, is_hole: bool) -> bool;
}

impl Validate for FixPath {
    fn validate(&mut self, min_area: FixFloat, is_hole: bool) -> bool {
        self.remove_degenerates();

        if self.len() < 3 {
            return false;
        }

        let area = self.area_x2();
        let fix_abs_area = area.abs() >> (FIX_FRACTION_BITS + 1);

        if fix_abs_area < min_area {
            return false;
        } else if is_hole && area > 0 || !is_hole && area < 0 {
            // for holes must be negative and for contour must be positive
            self.reverse();
        }

        true
    }
}