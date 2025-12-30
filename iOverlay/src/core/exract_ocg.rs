use alloc::vec;
use alloc::vec::Vec;
use i_shape::int::shape::IntShapes;
use i_shape::util::reserve::Reserve;
use crate::bind::segment::{ContourIndex, IdSegment};
use crate::bind::solver::{JoinHoles, LeftBottomSegment};
use crate::core::divide::ContourDecomposition;
use crate::core::extract::{BooleanExtractionBuffer, GraphContour, GraphUtil, StartPathData, Visit, VisitState};
use crate::core::graph::OverlayGraph;
use crate::core::overlay::ContourDirection;
use crate::core::overlay_rule::OverlayRule;
use crate::geom::v_segment::VSegment;

impl OverlayGraph<'_> {
    pub(crate) fn extract_ocg(
        &self,
        overlay_rule: OverlayRule,
        buffer: &mut BooleanExtractionBuffer,
    ) -> IntShapes {
        let is_main_dir_cw = self.options.output_direction == ContourDirection::Clockwise;

        let mut shapes = Vec::new();
        let mut holes = Vec::new();
        let mut anchors = Vec::new();

        buffer.points.reserve_capacity(buffer.visited.len());

        let mut link_index = 0;
        let mut anchors_already_sorted = true;
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

            self.find_contour(&start_data, traversal_direction, visited_state, buffer);
            let (is_valid, is_modified) = buffer.points.validate(
                self.options.min_output_area,
                self.options.preserve_output_collinear,
            );

            if !is_valid {
                link_index += 1;
                continue;
            }

            if is_hole {
                if let Some(hole_contours) = buffer.points.decompose_contours() {
                    anchors_already_sorted = false;
                    for mut hole in hole_contours.into_iter() {
                        let (is_valid, _) = hole.validate(
                            self.options.min_output_area,
                            self.options.preserve_output_collinear,
                        );

                        if !is_valid {
                            continue;
                        }

                        let most_left = hole.left_bottom_segment();
                        let id_data = ContourIndex::new_hole(holes.len());
                        anchors.push(IdSegment::with_segment(id_data, most_left));
                        holes.push(hole);
                    }
                } else {
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
            } else {
                let contour = buffer.points.as_slice().to_vec();
                shapes.push(vec![contour]);
            }
        }

        if !anchors_already_sorted {
            anchors.sort_by(|s0, s1| s0.v_segment.a.cmp(&s1.v_segment.a));
        }

        shapes.join_sorted_holes(holes, anchors, is_main_dir_cw);

        shapes
    }


}