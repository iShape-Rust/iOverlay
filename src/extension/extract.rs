use std::slice::Iter;
use i_float::triangle::Triangle;
use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShapes;
use crate::bind::point::ExclusionPathPoint;
use crate::bind::solver::JoinHoles;
use crate::core::extract::{StartPathData, Validate};
use crate::core::vector_rotation::NearestCCWVector;
use crate::extension::rule::ExtRule;
use crate::extension::unstable_graph::UnstableGraph;
use crate::segm::segment::SUBJ_BOTTOM;

impl UnstableGraph {
    #[inline(always)]
    pub fn extract_shapes(&self, ext_rule: ExtRule) -> IntShapes {
        self.extract_shapes_min_area(ext_rule, 0)
    }

    pub fn extract_shapes_min_area(&self, ext_rule: ExtRule, min_area: i64) -> IntShapes {
        let mut binding = self.filter(ext_rule);
        let visited = binding.as_mut_slice();
        let mut holes = Vec::new();
        let mut shapes = Vec::new();
        let mut hole_points = Vec::new();

        let mut link_index = 0;
        while link_index < visited.len() {
            let &count_to_visit = unsafe { visited.get_unchecked(link_index) };
            if count_to_visit == 0 {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_link(link_index, visited);
            let link = self.link(left_top_link);
            let &top_link_visited = unsafe { visited.get_unchecked(left_top_link) };

            if top_link_visited == 1 {
                let is_hole = link.fill & SUBJ_BOTTOM != SUBJ_BOTTOM;
                let start_data = StartPathData::new(is_hole, link, left_top_link);
                let mut path = self.get_path(&start_data, visited);

                if path.validate(min_area) {
                    if is_hole {
                        hole_points.push(ExclusionPathPoint {
                            id: holes.len(),
                            point: start_data.begin,
                            exclusion_path: usize::MAX,
                        });
                        holes.push(path);
                    } else {
                        shapes.push(vec![path]);
                    }
                }
            } else {
                // it's a hole and body at the same time

                let exclusion_path = shapes.len();
                let hole_id = holes.len();

                // extract body
                let body_start_data = StartPathData::new(false, link, left_top_link);
                let mut body_path = self.get_path(&body_start_data, visited);
                if body_path.validate(min_area) {
                    shapes.push(vec![body_path]);
                }

                if visited[left_top_link] > 0 {
                    // in some ca

                }

                // extract hole
                let hole_start_data = StartPathData::new(true, link, left_top_link);
                hole_points.push(ExclusionPathPoint {
                    id: hole_id,
                    point: hole_start_data.begin,
                    exclusion_path,
                });

                let mut hole_path = self.get_path(&hole_start_data, visited);
                if hole_path.validate(min_area) {
                    holes.push(hole_path);
                }
            }
        }

        shapes.join_exclusion_holes(&self.solver, holes, hole_points);

        shapes
    }

    #[inline]
    fn get_path(&self, start_data: &StartPathData, visited: &mut [u8]) -> IntPath {
        let link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        unsafe {
            *visited.get_unchecked_mut(link_id) -= 1;
        };

        let mut path = IntPath::new();
        path.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            let link_id = self.find_nearest_counter_wise_link_to(link_id, node_id, visited);

            let link = self.link(link_id);
            node_id = if link.a.id == node_id {
                path.push(link.a.point);
                link.b.id
            } else {
                path.push(link.b.point);
                link.a.id
            };

            unsafe {
                *visited.get_unchecked_mut(link_id) -= 1;
            };
        }

        path
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

            let &count = unsafe { visited.get_unchecked(i) };
            if count == 0 {
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
        let target = self.link(target_index);
        let (c, a) = if target.a.id == node_id {
            (target.a.point, target.b.point)
        } else { (target.b.point, target.a.point) };

        let mut iter = self.node(node_id).iter();

        let mut best_index = self.next_not_visited(&mut iter, visited).unwrap_or(usize::MAX);

        let second_index = if let Some(index) = self.next_not_visited(&mut iter, visited) {
            index
        } else {
            // only one link
            return best_index;
        };

        // more the one vectors
        let b = self.link(best_index).other(node_id).point;
        let mut vector_solver = NearestCCWVector::new(c, a, b);

        // check the second vector
        if vector_solver.add(self.link(second_index).other(node_id).point) {
            best_index = second_index;
        }

        // check the rest vectors
        while let Some(link_index) = self.next_not_visited(&mut iter, visited) {
            let p = self.links[link_index].other(node_id).point;
            if vector_solver.add(p) {
                best_index = link_index;
            }
        }

        best_index
    }

    #[inline(always)]
    fn next_not_visited(&self, iter: &mut Iter<usize>, visited: &[u8]) -> Option<usize> {
        iter.find_map(|&link_index| {
            let count = unsafe { *visited.get_unchecked(link_index) };
            if count > 0 {
                Some(link_index)
            } else {
                None
            }
        })
    }
}
