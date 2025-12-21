use super::overlay_rule::OverlayRule;
use crate::core::extract::{
    BooleanExtractionBuffer, GraphContour, GraphUtil, StartPathData, Visit, VisitState,
};
use crate::core::graph::OverlayGraph;
use crate::core::link::OverlayLinkFilter;
use crate::core::overlay::ContourDirection;
use alloc::vec;
use alloc::vec::Vec;
use i_shape::int::shape::IntShapes;

impl OverlayGraph<'_> {
    /// Extracts shapes while ensuring OGC-valid connected interiors by splitting composite holes.
    #[inline]
    pub fn extract_shapes_ocg(
        &self,
        overlay_rule: OverlayRule,
        buffer: &mut BooleanExtractionBuffer,
    ) -> IntShapes {
        self.links
            .filter_by_overlay_into(overlay_rule, &mut buffer.visited);
        self.extract_ocg(overlay_rule, buffer)
    }

    fn extract_ocg(
        &self,
        overlay_rule: OverlayRule,
        buffer: &mut BooleanExtractionBuffer,
    ) -> IntShapes {
        let mut shapes = self.extract(overlay_rule, buffer);

        // Keep only hole edges; skip everything else for the second pass.
        for state in buffer.visited.iter_mut() {
            *state = match *state {
                VisitState::HoleVisited => VisitState::Unvisited,
                _ => VisitState::Skipped,
            };
        }

        self.extract_inner_polygons_into(overlay_rule, buffer, &mut shapes);

        shapes
    }

    fn skip_contour(
        &self,
        start_data: &StartPathData,
        clockwise: bool,
        visited_state: VisitState,
        visited: &mut Vec<VisitState>,
    ) {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit_edge(link_id, visited_state);

        // Find a closed tour
        while node_id != last_node_id {
            link_id = GraphUtil::next_link(
                self.links, self.nodes, link_id, node_id, clockwise, &visited,
            );

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

    fn extract_inner_polygons_into(
        &self,
        overlay_rule: OverlayRule,
        buffer: &mut BooleanExtractionBuffer,
        shapes: &mut IntShapes,
    ) {
        let clockwise = self.options.output_direction == ContourDirection::Clockwise;

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
            let visited_state =
                [VisitState::HullVisited, VisitState::HoleVisited][is_hole as usize];

            let direction = is_hole == clockwise;
            let start_data = StartPathData::new(direction, link, left_top_link);

            if is_hole {
                // reverse hole visiting order
                self.skip_contour(&start_data, !direction, visited_state, &mut buffer.visited);
                continue;
            }

            self.find_contour(&start_data, direction, visited_state, buffer);
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
    }
}
