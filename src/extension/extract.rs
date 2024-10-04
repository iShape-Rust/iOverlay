use std::slice::Iter;
use i_float::triangle::Triangle;
use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShapes;
use crate::bind::point::{ExclusionPathPoint, PathPoint};
use crate::bind::solver::JoinHoles;
use crate::core::extract::{StartPathData, Validate};
use crate::core::overlay_link::OverlayLink;
use crate::core::overlay_node::OverlayNode;
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

                // extract hole
                let hole_start_data = StartPathData::new(true, link, left_top_link);
                hole_points.push(ExclusionPathPoint {
                    id: holes.len(),
                    point: hole_start_data.begin,
                    exclusion_path: shapes.len(),
                });

                let mut hole_path = self.get_path(&hole_start_data, visited);
                if hole_path.validate(min_area) {
                    holes.push(hole_path);
                }

                // extract body
                let body_start_data = StartPathData::new(false, link, left_top_link);
                let mut body_path = self.get_path(&body_start_data, visited);
                if body_path.validate(min_area) {
                    shapes.push(vec![body_path]);
                }
            }
        }

        shapes.join_exclusion_holes(&self.solver, holes, hole_points);

        shapes
    }

    #[inline]
    fn get_path(&self, start_data: &StartPathData, visited: &mut [u8]) -> IntPath {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        unsafe {
            *visited.get_unchecked_mut(link_id) -= 1;
        };

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

            unsafe {
                *visited.get_unchecked_mut(link_id) -= 1;
            };
        }

        path
    }

    #[inline]
    pub(crate) fn find_left_top_link(&self, link_index: usize, visited: &[u8]) -> usize {
        let top = self.link(link_index);
        debug_assert!(top.is_direct());

        let node = self.node(top.a.id);

        match node {
            OverlayNode::Bridge(bridge) => {
                self.find_left_top_link_on_bridge(bridge)
            }
            OverlayNode::Cross(indices) => {
                self.find_left_top_link_on_indices(top, link_index, indices, visited)
            }
        }
    }

    #[inline(always)]
    fn find_left_top_link_on_indices(&self, link: &OverlayLink, link_index: usize, indices: &[usize], visited: &[u8]) -> usize {
        let mut top_index = link_index;
        let mut top = link;

        // find most top link

        for &i in indices.iter() {
            if i == link_index {
                continue;
            }
            let link = self.link(i);
            if !link.is_direct() || Triangle::is_clockwise_point(top.a.point, top.b.point, link.b.point) {
                continue;
            }

            let &is_visit = unsafe { visited.get_unchecked(i) };
            if is_visit {
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
    ) -> Option<usize> {
        let target = self.link(target_index);
        let (c, a) = if target.a.id == node_id {
            (target.a.point, target.b.point)
        } else { (target.b.point, target.a.point) };

        let mut iter = self.node(node_id).iter();

        let mut best_index = self.next_not_visited(&mut iter, visited)?;
        let mut link_index = self.next_not_visited(&mut iter, visited)?;

        if link_index >= self.links.len() {
            // no more links
            return best_index;
        }

        let va = a.subtract(c);
        let b = self.link(best_index).other(node_id).point;
        let mut vb = b.subtract(c);
        let mut more_180 = va.cross_product(vb) <= 0;

        while link_index < self.links.len() {
            let link = &self.links[link_index];
            let p = link.other(node_id).point;
            let vp = p.subtract(c);
            let new_more_180 = va.cross_product(vp) <= 0;

            if new_more_180 == more_180 {
                // both more 180 or both less 180
                let is_clock_wise = vp.cross_product(vb) > 0;
                if is_clock_wise {
                    best_index = link_index;
                    vb = vp;
                }
            } else if more_180 {
                // new less 180
                more_180 = false;
                best_index = link_index;
                vb = vp;
            }

            link_index = indices.next_link(&mut it_index, visited);
        }

        Some(best_index)
    }

    #[inline(always)]
    fn next_not_visited(&self, iter: &mut Iter<usize>, visited: &[u8]) -> Option<usize> {
        for link_index in iter {}
        None
    }
        /*
        let mut it_index = 0;
        while it_index < self.len() {
            let link_index = self[it_index];
            it_index += 1;
            let &is_visit = unsafe { visited.get_unchecked(link_index) };
            if !is_visit {
                return (it_index, link_index);
            }
        }
        unreachable!("The loop should always return");


    #[inline(always)]
    fn next_link(&self, it_index: &mut usize, visited: &[bool]) -> Option<usize> {
        while *it_index < self.len() {
            let link_index = self[*it_index];
            *it_index += 1;
            let &is_visit = unsafe { visited.get_unchecked(link_index) };
            if !is_visit {
                return link_index;
            }
        }

        None
    }
    */
}