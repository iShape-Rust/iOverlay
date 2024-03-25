use i_float::point::Point;
use crate::bool::filter::Filter;
use crate::bool::overlay_rule::OverlayRule;
use crate::geom::floor::Floors;
use crate::geom::holes_solver::HolesSolver;
use crate::geom::id_point::IdPoint;
use crate::geom::x_order::XOrder;
use crate::index::EMPTY_INDEX;
use crate::layout::overlay_graph::OverlayGraph;
use crate::space::line_range::LineRange;
use crate::vector::vector::{VectorEdge, VectorPath, VectorShape};

impl OverlayGraph {
    pub fn extract_vectors(&self, overlay_rule: OverlayRule) -> Vec<VectorShape> {
        let mut visited = self.links.filter(overlay_rule);

        let mut holes = Vec::new();
        let mut shapes = Vec::new();

        let mut j = 0;
        while j < self.nodes.len() {
            let i = self.find_first_link(j, &visited);
            if i == EMPTY_INDEX {
                j += 1;
            } else {
                let is_hole = overlay_rule.is_fill_top(self.links[i].fill);
                let mut path = self.get_path(overlay_rule, i, &mut visited);
                path.validate(is_hole);
                if is_hole {
                    holes.push(path);
                } else {
                    shapes.push([path].to_vec());
                }
            }
        }

        shapes.join(holes);

        shapes
    }

    fn get_path(&self, overlay_rule: OverlayRule, index: usize, visited: &mut Vec<bool>) -> VectorPath {
        let mut path = VectorPath::new();
        let mut next = index;

        let mut link = self.links[index];

        let mut a = link.a;
        let mut b = link.b;

        // Find a closed tour
        loop {
            path.push(VectorEdge::new(link.fill, a.point, b.point));
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
    fn join(&mut self, holes: Vec<VectorPath>);
    fn scan_join(&mut self, holes: Vec<VectorPath>);
}

impl JoinHoles for Vec<VectorShape> {
    fn join(&mut self, holes: Vec<VectorPath>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].reserve_exact(holes.len());
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
        } else {
            self.scan_join(holes);
        }
    }

    fn scan_join(&mut self, holes: Vec<VectorPath>) {
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut i_points = Vec::with_capacity(holes.len());
        for i in 0..holes.len() {
            let p = holes[i][0].a;
            let x = p.x as i32;
            let y = p.y as i32;
            i_points.push(IdPoint::new(i, Point::new(x, y)));
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
        i_points.sort_by(|a, b| a.point.order_by_x(b.point));

        let x_min = i_points[0].point.x;
        let x_max = i_points[i_points.len() - 1].point.x;

        let mut floors = Vec::new();
        for i in 0..self.len() {
            let mut hole_floors = self[i][0].floors(i, x_min, x_max, &mut y_min, &mut y_max);
            floors.append(&mut hole_floors);
        }

        floors.sort_by(|a, b| a.seg.a.order_by_x(b.seg.a));

        let y_range = LineRange { min: y_min, max: y_max };
        let solution = HolesSolver::solve(self.len(), y_range, i_points, floors);

        for shape_index in 0..solution.hole_counter.len() {
            let capacity = solution.hole_counter[shape_index];
            self[shape_index].reserve_exact(capacity);
        }

        let mut hole_index = 0;
        for hole in holes.into_iter() {
            let shape_index = solution.hole_shape[hole_index];
            self[shape_index].push(hole);
            hole_index += 1;
        }
    }
}

trait Validate {
    fn validate(&mut self, is_hole: bool);
    fn is_positive(&self) -> bool;
}

impl Validate for VectorPath {
    fn validate(&mut self, is_hole: bool) {
        let is_positive = self.is_positive();
        if is_hole && !is_positive || !is_hole && is_positive {
            for v in self.iter_mut() {
                v.reverse();
            }
        }
    }

    fn is_positive(&self) -> bool {
        let mut area: i64 = 0;
        for v in self {
            area += v.a.cross_product(v.b);
        }
        area >= 0
    }
}