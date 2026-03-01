use crate::bind::segment::{ContourIndex, IdSegment};
use crate::bind::solver::{JoinHoles, LeftBottomSegment};
use crate::core::extract::{
    BooleanExtractionBuffer, GraphContour, GraphUtil, StartPathData, Visit, VisitState,
};
use crate::core::link::OverlayLinkFilter;
use crate::core::overlay::ContourDirection;
use crate::core::overlay_rule::OverlayRule;
use crate::geom::v_segment::VSegment;
use crate::mesh::graph::OffsetGraph;
use crate::segm::segment::SUBJ_TOP;
use alloc::vec;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::int::shape::IntShapes;
use i_shape::util::reserve::Reserve;

impl OffsetGraph<'_> {
    pub(crate) fn extract_offset(
        &self,
        main_direction: ContourDirection,
        min_area: u64,
        buffer: &mut BooleanExtractionBuffer,
    ) -> IntShapes {
        self.links
            .filter_by_overlay_into(OverlayRule::Subject, &mut buffer.visited);
        self.extract_offset_shapes(main_direction, min_area, buffer)
    }

    #[inline]
    pub(crate) fn extract_contours_into(
        &self,
        main_direction: ContourDirection,
        min_area: u64,
        buffer: &mut BooleanExtractionBuffer,
        output: &mut FlatContoursBuffer,
    ) {
        self.links
            .filter_by_overlay_into(OverlayRule::Subject, &mut buffer.visited);
        self.extract_contours(main_direction, min_area, buffer, output);
    }

    fn extract_offset_shapes(
        &self,
        main_direction: ContourDirection,
        min_area: u64,
        buffer: &mut BooleanExtractionBuffer,
    ) -> IntShapes {
        let clockwise = main_direction == ContourDirection::Clockwise;
        let len = buffer.visited.len();
        buffer.points.reserve_capacity(len);

        let mut shapes = Vec::new();
        let mut holes = Vec::new();
        let mut anchors = Vec::new();

        let mut link_index = 0;
        let mut is_all_anchors_sorted = true;
        while link_index < len {
            if buffer.visited.is_visited(link_index) {
                link_index += 1;
                continue;
            }
            let left_top_link = unsafe {
                // Safety: `link_index` walks 0..buffer.visited.len(), and buffer.visited.len() <= self.links.len().
                GraphUtil::find_left_top_link(self.links, self.nodes, link_index, &buffer.visited)
            };

            let link = unsafe {
                // SAFETY: `left_top_link` originates from `find_left_top_link`, which only returns
                // indices in 0..self.links.len(), so this lookup cannot go out of bounds.
                self.links.get_unchecked(left_top_link)
            };
            let is_hole = link.fill & SUBJ_TOP == SUBJ_TOP;
            let mut bold = link.is_bold();
            let direction = is_hole == clockwise;

            let start_data = StartPathData::new(direction, link, left_top_link);

            self.find_contour(
                &start_data,
                direction,
                &mut bold,
                &mut buffer.visited,
                &mut buffer.points,
            );

            if !bold {
                link_index += 1;
                continue;
            }

            let (is_valid, is_modified) = buffer.points.validate(min_area, true);

            if !is_valid {
                link_index += 1;
                continue;
            }

            let contour = buffer.points.to_vec();

            if is_hole {
                let mut v_segment = if clockwise {
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
                        is_all_anchors_sorted = false;
                    }
                };

                debug_assert_eq!(v_segment, contour.left_bottom_segment());
                let id = ContourIndex::new_hole(holes.len());
                anchors.push(IdSegment::with_segment(id, v_segment));
                holes.push(contour);
            } else {
                shapes.push(vec![contour]);
            }
        }

        if !is_all_anchors_sorted {
            anchors.sort_by(|s0, s1| s0.v_segment.a.cmp(&s1.v_segment.a));
        }

        shapes.join_sorted_holes(holes, anchors, clockwise);

        shapes
    }

    fn extract_contours(
        &self,
        main_direction: ContourDirection,
        min_area: u64,
        buffer: &mut BooleanExtractionBuffer,
        output: &mut FlatContoursBuffer,
    ) {
        let clockwise = main_direction == ContourDirection::Clockwise;
        let len = buffer.visited.len();
        buffer.points.reserve_capacity(len);
        output.clear_and_reserve(len, 4);

        let mut link_index = 0;
        while link_index < len {
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
            let is_hole = link.fill & SUBJ_TOP == SUBJ_TOP;
            let mut bold = link.is_bold();
            let direction = is_hole == clockwise;

            let start_data = StartPathData::new(direction, link, left_top_link);

            self.find_contour(
                &start_data,
                direction,
                &mut bold,
                &mut buffer.visited,
                &mut buffer.points,
            );

            if !bold {
                link_index += 1;
                continue;
            }

            let (is_valid, _) = buffer.points.validate(min_area, true);

            if !is_valid {
                link_index += 1;
                continue;
            }

            output.add_contour(buffer.points.as_slice());
        }
    }

    fn find_contour(
        &self,
        start_data: &StartPathData,
        clockwise: bool,
        bold: &mut bool,
        visited: &mut [VisitState],
        points: &mut Vec<IntPoint>,
    ) {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        points.clear();
        points.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            link_id = GraphUtil::next_link(self.links, self.nodes, link_id, node_id, clockwise, visited);

            let link = unsafe {
                // Safety: `link_id` is always derived from a previous in-bounds index or
                // from `find_left_top_link`, so it remains in `0..self.links.len()`.
                self.links.get_unchecked(link_id)
            };
            *bold = *bold || link.is_bold();
            node_id = points.push_node_and_get_other(link, node_id);

            visited.visit(link_id);
        }
    }
}
