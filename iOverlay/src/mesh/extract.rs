use alloc::vec;
use alloc::vec::Vec;
use i_shape::int::shape::{IntContour, IntShapes};
use crate::bind::segment::{ContourIndex, IdSegment};
use crate::bind::solver::{JoinHoles, LeftBottomSegment};
use crate::core::extract::{GraphUtil, StartPathData, GraphContour, Visit};
use crate::core::link::{OverlayLink, OverlayLinkFilter};
use crate::core::overlay::ContourDirection;
use crate::core::overlay_rule::OverlayRule;
use crate::geom::v_segment::VSegment;
use crate::mesh::graph::OffsetGraph;
use crate::segm::segment::SUBJ_TOP;

impl OffsetGraph<'_> {
    pub(crate) fn extract_offset(&self, main_direction: ContourDirection, min_area: u64) -> IntShapes {
        let visited = self.links.filter_by_overlay(OverlayRule::Subject);
        self.extract_offset_shapes(visited, main_direction, min_area)
    }

    fn extract_offset_shapes(
        &self,
        filter: Vec<bool>,
        main_direction: ContourDirection,
        min_area: u64,
    ) -> IntShapes {
        let clockwise = main_direction == ContourDirection::Clockwise;
        let mut buffer = filter;
        let visited = buffer.as_mut_slice();
        let mut shapes = Vec::new();
        let mut holes = Vec::new();
        let mut anchors = Vec::new();

        let mut link_index = 0;
        let mut is_all_anchors_sorted = true;
        while link_index < visited.len() {
            if visited.is_visited(link_index) {
                link_index += 1;
                continue;
            }

            let left_top_link = GraphUtil::find_left_top_link(self.links, self.nodes, link_index, visited);
            let link = self.link(left_top_link);
            let is_hole = link.fill & SUBJ_TOP == SUBJ_TOP;
            let mut bold = link.is_bold();
            let direction = is_hole == clockwise;

            let start_data = StartPathData::new(direction, link, left_top_link);

            let mut contour = self.get_fill_contour(&start_data, direction, &mut bold, visited);
            if !bold {
                link_index += 1;
                continue;
            }

            let (is_valid, is_modified) = contour.validate(min_area, true);

            if !is_valid {
                link_index += 1;
                continue;
            }

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

    fn get_fill_contour(
        &self,
        start_data: &StartPathData,
        clockwise: bool,
        bold: &mut bool,
        visited: &mut [bool],
    ) -> IntContour {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        let mut contour = IntContour::new();
        contour.push(start_data.begin);

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
            *bold = *bold || link.is_bold();

            node_id = contour.push_node_and_get_other(link, node_id);

            visited.visit(link_id);
        }

        contour
    }

    #[inline(always)]
    fn link(&self, index: usize) -> &OverlayLink {
        unsafe { self.links.get_unchecked(index) }
    }
}