use alloc::vec;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_key_sort::sort::key_sort::KeyBinSort;
use crate::bind::segment::{ContourIndex, IdSegment, IdSegments};
use crate::bind::solver::ShapeBinder;
use crate::core::extract::GraphUtil;
use crate::core::graph::OverlayGraph;
use crate::core::link::OverlayLinkFilter;
use crate::core::overlay_rule::OverlayRule;
use crate::geom::v_segment::VSegment;
use crate::segm::segment::SegmentFill;
use crate::vector::edge::{VectorEdge, VectorPath, VectorShape};

impl OverlayGraph<'_> {
    pub fn extract_separate_vectors(&self) -> Vec<VectorEdge> {
        self.links
            .iter()
            .map(|link| VectorEdge {
                a: link.a.point,
                b: link.b.point,
                fill: link.fill,
            })
            .collect()
    }

    pub fn extract_shape_vectors(&self, overlay_rule: OverlayRule) -> Vec<VectorShape> {
        let mut binding = self.links.filter_by_overlay(overlay_rule);
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

            let left_top_link = GraphUtil::find_left_top_link(
                self.links,
                self.nodes,
                link_index,
                visited
            );
            let link = unsafe { self.links.get_unchecked(left_top_link) };
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
                let path = self.get_vector_path(start_data, true, visited);
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
                let path = self.get_vector_path(start_data, false, visited);
                shapes.push(vec![path]);
            };

            link_index += 1;
        }

        shapes.join(holes, true);

        shapes
    }

    fn get_vector_path(
        &self,
        start_data: StartVectorPathData,
        clockwise: bool,
        visited: &mut [bool],
    ) -> VectorPath {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        unsafe {
            *visited.get_unchecked_mut(link_id) = true;
        };

        let mut contour = VectorPath::new();
        contour.push(VectorEdge::new(start_data.fill, start_data.a, start_data.b));

        // Find a closed tour
        while node_id != last_node_id {
            link_id = GraphUtil::next_link(
                self.links,
                self.nodes,
                link_id,
                node_id,
                clockwise,
                visited,
            );

            let link = unsafe { self.links.get_unchecked(link_id) };
            node_id = if link.a.id == node_id {
                contour.push(VectorEdge::new(link.fill, link.a.point, link.b.point));
                link.b.id
            } else {
                contour.push(VectorEdge::new(link.fill, link.b.point, link.a.point));
                link.a.id
            };

            unsafe {
                *visited.get_unchecked_mut(link_id) = true;
            };
        }

        contour
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
    fn join(&mut self, holes: Vec<VectorPath>, clockwise: bool);
    fn scan_join(&mut self, holes: Vec<VectorPath>, clockwise: bool);
}

impl JoinHoles for Vec<VectorShape> {
    fn join(&mut self, holes: Vec<VectorPath>, clockwise: bool) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].reserve_exact(holes.len());
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
        } else {
            self.scan_join(holes, clockwise);
        }
    }

    fn scan_join(&mut self, holes: Vec<VectorPath>, clockwise: bool) {
        let hole_segments: Vec<_> = holes
            .iter()
            .enumerate()
            .map(|(id, path)| {
                let v = path[1];
                let v_segment = if v.a < v.b {
                    VSegment { a: v.a, b: v.b }
                } else {
                    VSegment { a: v.b, b: v.a }
                };
                debug_assert_eq!(v_segment, most_left_bottom(path));
                let id_data = ContourIndex::new_hole(id);
                IdSegment::with_segment(id_data, v_segment)
            })
            .collect();

        debug_assert!(is_sorted(&hole_segments));

        let x_min = hole_segments[0].v_segment.a.x;
        let x_max = hole_segments[hole_segments.len() - 1].v_segment.a.x;

        let mut segments = Vec::new();
        for (i, shape) in self.iter().enumerate() {
            shape[0].append_id_segments(&mut segments, ContourIndex::new_shape(i), x_min, x_max, clockwise);
        }

        segments.sort_with_bins(|a, b| a.v_segment.a.x.cmp(&b.v_segment.a.x));

        let solution = ShapeBinder::bind(self.len(), hole_segments, segments);

        for (shape_index, &capacity) in solution.children_count_for_parent.iter().enumerate() {
            self[shape_index].reserve_exact(capacity);
        }

        for (hole_index, hole) in holes.into_iter().enumerate() {
            let shape_index = solution.parent_for_child[hole_index];
            self[shape_index].push(hole);
        }
    }
}

#[inline]
fn most_left_bottom(path: &VectorPath) -> VSegment {
    let mut index = 0;
    let mut a = path[0].a;
    for (i, &e) in path.iter().enumerate().skip(1) {
        if e.a < a {
            a = e.a;
            index = i;
        }
    }
    let n = path.len();
    let b0 = path[index].b;
    let b1 = path[(index + n - 1) % n].a;

    let s0 = VSegment { a, b: b0 };
    let s1 = VSegment { a, b: b1 };

    if s0.is_under_segment(&s1) { s0 } else { s1 }
}

#[inline]
fn is_sorted(segments: &[IdSegment]) -> bool {
    segments
        .windows(2)
        .all(|slice| slice[0].v_segment.a <= slice[1].v_segment.a)
}
