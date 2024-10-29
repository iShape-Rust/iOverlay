use i_float::int::point::IntPoint;
use crate::bind::segment::IdSegments;
use crate::bind::solver::ShapeBinder;
use crate::geom::id_point::IdPoint;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_rule::OverlayRule;
use crate::core::filter::Filter;
use crate::core::overlay_node::OverlayNode;
use crate::core::solver::Solver;
use crate::segm::segment::SegmentFill;
use crate::util::sort::SmartBinSort;
use crate::vector::edge::{VectorEdge, VectorPath, VectorShape};

impl OverlayGraph {
    pub fn extract_separate_vectors(&self) -> Vec<VectorEdge> {
        self.links.iter().map(|link| VectorEdge {
            a: link.a.point,
            b: link.b.point,
            fill: link.fill,
        }).collect()
    }

    pub fn extract_shape_vectors(&self, overlay_rule: OverlayRule) -> Vec<VectorShape> {
        let mut binding = self.links.filter(overlay_rule);
        let visited = binding.as_mut_slice();
        let mut holes = Vec::new();
        let mut shapes = Vec::new();

        let mut link_index = 0;
        while link_index < visited.len() {
            let &is_visited = unsafe { visited.get_unchecked(link_index) };
            if is_visited {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_link(link_index, visited);
            let link = self.link(left_top_link);
            let is_hole = overlay_rule.is_fill_top(link.fill);

            if is_hole {
                let start_data = StartVectorPathData {
                    a: link.b.point,
                    b: link.a.point,
                    node_id: link.a.id,
                    link_id: left_top_link,
                    last_node_id: link.b.id,
                    fill: link.fill,
                };
                let path = self.get_vector_path(start_data, visited);
                holes.push(path);
            } else {
                let start_data = StartVectorPathData {
                    a: link.a.point,
                    b: link.b.point,
                    node_id: link.b.id,
                    link_id: left_top_link,
                    last_node_id: link.a.id,
                    fill: link.fill,
                };
                let path = self.get_vector_path(start_data, visited);
                shapes.push(vec![path]);
            };

            link_index += 1;
        }

        shapes.join(&self.solver, holes);

        shapes
    }

    fn get_vector_path(&self, start_data: StartVectorPathData, visited: &mut [bool]) -> VectorPath {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        unsafe { *visited.get_unchecked_mut(link_id) = true; };

        let mut path = VectorPath::new();
        path.push(VectorEdge::new(start_data.fill, start_data.a, start_data.b));

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
                path.push(VectorEdge::new(link.fill, link.a.point, link.b.point));
                link.b.id
            } else {
                path.push(VectorEdge::new(link.fill, link.b.point, link.a.point));
                link.a.id
            };

            unsafe { *visited.get_unchecked_mut(link_id) = true; };
        }

        path
    }
}

struct StartVectorPathData {
    a: IntPoint,
    b: IntPoint,
    node_id: usize,
    link_id: usize,
    last_node_id: usize,
    fill: SegmentFill,
}

trait JoinHoles {
    fn join(&mut self, solver: &Solver, holes: Vec<VectorPath>);
    fn scan_join(&mut self, solver: &Solver, holes: Vec<VectorPath>);
}

impl JoinHoles for Vec<VectorShape> {
    fn join(&mut self, solver: &Solver, holes: Vec<VectorPath>) {
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

    fn scan_join(&mut self, solver: &Solver, holes: Vec<VectorPath>) {
        let mut i_points = Vec::with_capacity(holes.len());
        for (i, hole) in holes.iter().enumerate() {
            let p = hole.first().unwrap().a;
            i_points.push(IdPoint::new(i, p));
        }

        // TODO this sort probably is not need because we take first point in path
        i_points.smart_bin_sort_by(solver, |a, b| a.point.x.cmp(&b.point.x));

        let x_min = i_points[0].point.x;
        let x_max = i_points[i_points.len() - 1].point.x;

        let mut segments = Vec::new();
        for (i, shape) in self.iter().enumerate() {
            shape[0].append_id_segments(&mut segments, i, x_min, x_max);
        }

        segments.smart_bin_sort_by(solver, |a, b| a.x_segment.a.x.cmp(&b.x_segment.a.x));

        let solution = ShapeBinder::bind(self.len(), i_points, segments);

        for (shape_index, &capacity) in solution.children_count_for_parent.iter().enumerate() {
            self[shape_index].reserve_exact(capacity);
        }

        for (hole_index, hole) in holes.into_iter().enumerate() {
            let shape_index = solution.parent_for_child[hole_index];
            self[shape_index].push(hole);
        }
    }
}