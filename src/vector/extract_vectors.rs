use crate::bind::segment::IdSegments;
use crate::bind::solver::ShapeBinder;
use crate::id_point::IdPoint;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_rule::OverlayRule;
use crate::core::filter::Filter;
use crate::sort::SmartSort;
use crate::util::EMPTY_INDEX;
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
                let mut path = self.get_vector_path(overlay_rule, i, &mut visited);
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

    fn get_vector_path(&self, overlay_rule: OverlayRule, index: usize, visited: &mut Vec<bool>) -> VectorPath {
        let mut path = VectorPath::new();
        let mut next = index;

        let mut link = self.links[index];

        let mut a = link.a;
        let mut b = link.b;

        // Find a closed tour
        loop {
            path.push(VectorEdge::new(link.fill, a.point, b.point));
            let node = &self.nodes[b.id];

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
        let mut i_points = Vec::with_capacity(holes.len());
        for i in 0..holes.len() {
            let p = holes[i][0].a;
            i_points.push(IdPoint::new(i, p));
        }
        i_points.smart_sort_by(|a, b| a.point.x.cmp(&b.point.x));

        let x_min = i_points[0].point.x;
        let x_max = i_points[i_points.len() - 1].point.x;

        let mut segments = Vec::new();
        for i in 0..self.len() {
            let mut hole_floors = self[i][0].id_segments(i, x_min, x_max);
            segments.append(&mut hole_floors);
        }

        segments.smart_sort_by(|a, b| a.x_segment.a.x.cmp(&b.x_segment.a.x));

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