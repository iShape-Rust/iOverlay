use crate::bind::segment::{ContourIndex, IdSegment};
use crate::bind::solver::{JoinHoles, LeftBottomSegment};
use crate::core::extract::{
    BooleanExtractionBuffer, GraphContour, GraphUtil, StartPathData, Visit, VisitState,
};
use crate::core::graph::OverlayGraph;
use crate::core::overlay::ContourDirection;
use crate::core::overlay_rule::OverlayRule;
use crate::geom::v_segment::VSegment;
use alloc::vec;
use alloc::vec::Vec;
use i_shape::int::shape::IntShapes;
use i_shape::util::reserve::Reserve;

impl OverlayGraph<'_> {
    pub(crate) fn extract_ogc(
        &self,
        overlay_rule: OverlayRule,
        buffer: &mut BooleanExtractionBuffer,
    ) -> IntShapes {
        let is_main_dir_cw = self.options.output_direction == ContourDirection::Clockwise;

        let mut shapes = Vec::new();

        buffer.points.reserve_capacity(buffer.visited.len());
        let mut avg_holes_count = 0;

        let mut link_index = 0;
        while link_index < buffer.visited.len() {
            if buffer.visited.is_visited(link_index) {
                link_index += 1;
                continue;
            }

            let left_top_link = unsafe {
                // Safety: `link_index` walks 0..buffer.visited.len(), and buffer.visited.len() <= self.links.len().
                GraphUtil::find_left_top_link(self.links, self.nodes, link_index, &buffer.visited)
            };

            let link = unsafe {
                // Safety: `left_top_link` originates from `find_left_top_link`, which only returns
                // indices in 0..self.links.len(), so this lookup cannot go out of bounds.
                self.links.get_unchecked(left_top_link)
            };
            let is_hole = overlay_rule.is_fill_top(link.fill);
            let visited_state = [VisitState::HullVisited, VisitState::HoleVisited][is_hole as usize];

            let direction = is_hole == is_main_dir_cw;
            let traversal_direction = !is_main_dir_cw;

            let start_data = StartPathData::new(direction, link, left_top_link);

            if is_hole {
                // we will collect holes in a second pass
                self.skip_contour(
                    &start_data,
                    traversal_direction,
                    visited_state,
                    &mut buffer.visited,
                );
                avg_holes_count += 1;
                continue;
            }

            self.find_contour(&start_data, traversal_direction, visited_state, buffer);
            let (is_valid, _) = buffer.points.validate(
                self.options.min_output_area,
                self.options.preserve_output_collinear,
            );

            if !is_valid {
                link_index += 1;
                continue;
            }

            let contour = buffer.points.as_slice().to_vec();
            shapes.push(vec![contour]);
        }

        if avg_holes_count > 0 {
            // Keep only hole edges; skip everything else for the second pass.
            for state in buffer.visited.iter_mut() {
                *state = match *state {
                    VisitState::HoleVisited => VisitState::Unvisited,
                    _ => VisitState::Skipped,
                };
            }

            let mut holes = Vec::with_capacity(avg_holes_count);
            let mut anchors = Vec::with_capacity(avg_holes_count);
            let mut anchors_already_sorted = true;
            link_index = 0;

            while link_index < buffer.visited.len() {
                if buffer.visited.is_visited(link_index) {
                    link_index += 1;
                    continue;
                }

                let left_top_link = unsafe {
                    // Safety: `link_index` walks 0..buffer.visited.len(), and buffer.visited.len() <= self.links.len().
                    GraphUtil::find_left_top_link(self.links, self.nodes, link_index, &buffer.visited)
                };

                let link = unsafe {
                    // Safety: `left_top_link` originates from `find_left_top_link`, which only returns
                    // indices in 0..self.links.len(), so this lookup cannot go out of bounds.
                    self.links.get_unchecked(left_top_link)
                };

                debug_assert!(overlay_rule.is_fill_top(link.fill));

                let start_data = StartPathData::new(is_main_dir_cw, link, left_top_link);

                self.find_contour(&start_data, is_main_dir_cw, VisitState::HullVisited, buffer);
                let (is_valid, is_modified) = buffer.points.validate(
                    self.options.min_output_area,
                    self.options.preserve_output_collinear,
                );

                if !is_valid {
                    link_index += 1;
                    continue;
                }
                let contour = buffer.points.as_slice().to_vec();

                let mut v_segment = if is_main_dir_cw {
                    VSegment {
                        a: contour[1],
                        b: contour[2],
                    }
                } else {
                    VSegment {
                        a: contour[0],
                        b: contour[contour.len() - 1],
                    }
                };

                if is_modified {
                    let most_left = contour.left_bottom_segment();
                    if most_left != v_segment {
                        v_segment = most_left;
                        anchors_already_sorted = false;
                    }
                };

                debug_assert_eq!(v_segment, contour.left_bottom_segment());
                let id_data = ContourIndex::new_hole(holes.len());
                anchors.push(IdSegment::with_segment(id_data, v_segment));
                holes.push(contour);
            }

            if !anchors_already_sorted {
                anchors.sort_by(|s0, s1| s0.v_segment.a.cmp(&s1.v_segment.a));
            }

            shapes.join_sorted_holes(holes, anchors, is_main_dir_cw);
        }

        shapes
    }

    fn skip_contour(
        &self,
        start_data: &StartPathData,
        clockwise: bool,
        visited_state: VisitState,
        visited: &mut [VisitState],
    ) {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit_edge(link_id, visited_state);

        // Find a closed tour
        while node_id != last_node_id {
            link_id = GraphUtil::next_link(self.links, self.nodes, link_id, node_id, clockwise, visited);

            let link = unsafe {
                // Safety: `link_id` is always derived from a previous in-bounds index or
                // from `find_left_top_link`, so it remains in `0..self.links.len()`.
                self.links.get_unchecked(link_id)
            };

            node_id = if link.a.id == node_id {
                link.b.id
            } else {
                link.a.id
            };

            visited.visit_edge(link_id, visited_state);
        }
    }
}
