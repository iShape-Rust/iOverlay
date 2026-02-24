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
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::int::shape::{IntContour, IntShapes};
use i_shape::util::reserve::Reserve;

impl OffsetGraph<'_> {
    pub(crate) fn extract_offset(&self, main_direction: ContourDirection, min_area: u64) -> IntShapes {
        let visited = self.links.filter_by_overlay(OverlayRule::Subject);
        self.extract_offset_shapes(visited, main_direction, min_area)
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
        filter: Vec<VisitState>,
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

            let left_top_link = unsafe {
                // SAFETY: `link_index` walks 0..buffer.visited.len(), and buffer.visited.len() <= self.links.len().
                GraphUtil::find_left_top_link(self.links, self.nodes, link_index, visited)
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

            let mut contour = self.get_fill_contour(&start_data, direction, &mut bold, &mut buffer.visited);
            if !bold {
                link_index += 1;
                continue;
            }

            let (is_valid, is_modified) = contour.validate(min_area, true);

            if !is_valid {
                link_index += 1;
                continue;
            }

            output.add_contour(buffer.points.as_slice());
        }
    }

    fn get_fill_contour(
        &self,
        start_data: &StartPathData,
        clockwise: bool,
        bold: &mut bool,
        visited: &mut [VisitState],
    ) -> IntContour {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        let mut contour = IntContour::new();
        contour.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            link_id = GraphUtil::next_link(self.links, self.nodes, link_id, node_id, clockwise, visited);

            let link = unsafe {
                // SAFETY: `link_id` is always derived from a previous in-bounds index or
                // from `find_left_top_link`, so it remains in `0..self.links.len()`.
                self.links.get_unchecked(link_id)
            };
            *bold = *bold || link.is_bold();

            node_id = contour.push_node_and_get_other(link, node_id);

            visited.visit(link_id);
        }

        contour
    }
}

#[cfg(test)]
mod tests {
    use crate::geom::x_segment::XSegment;
    use crate::segm::offset::ShapeCountOffset;
    use crate::segm::segment::Segment;
    use alloc::vec;
    use i_float::int::point::IntPoint;
    use crate::core::overlay::ContourDirection;
    use crate::mesh::overlay::OffsetOverlay;

    #[test]
    fn test_0() {
        let segments = vec![
            Segment::<ShapeCountOffset> {
                // 0
                x_segment: XSegment {
                    a: IntPoint::new(100_884_823, -84_374_363),
                    b: IntPoint::new(103_800_375, -100_646_683),
                },
                count: ShapeCountOffset { subj: 1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 1
                x_segment: XSegment {
                    a: IntPoint::new(-20_055_752, 25_821_939),
                    b: IntPoint::new(103_800_375, -100_646_683),
                },
                count: ShapeCountOffset {
                    subj: -1,
                    bold: false,
                },
            },
            Segment::<ShapeCountOffset> {
                // 2
                x_segment: XSegment {
                    a: IntPoint::new(-20_055_752, 25_821_939),
                    b: IntPoint::new(-19_013_992, 25_612_755),
                },
                count: ShapeCountOffset { subj: 1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 3
                x_segment: XSegment {
                    a: IntPoint::new(-57_843_404, 28_494_136),
                    b: IntPoint::new(-19_013_992, 25_612_755),
                },
                count: ShapeCountOffset {
                    subj: -1,
                    bold: false,
                },
            },
            Segment::<ShapeCountOffset> {
                // 4
                x_segment: XSegment {
                    a: IntPoint::new(-57_843_404, 28_494_136),
                    b: IntPoint::new(-56_476_844, 28_562_552),
                },
                count: ShapeCountOffset { subj: 1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 5
                x_segment: XSegment {
                    a: IntPoint::new(-56_476_844, 28_562_552),
                    b: IntPoint::new(56_113_245, -11_174_714),
                },
                count: ShapeCountOffset { subj: 1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 6
                x_segment: XSegment {
                    a: IntPoint::new(56_113_245, -11_174_714),
                    b: IntPoint::new(108_356_443, -118_534_979),
                },
                count: ShapeCountOffset { subj: 1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 7
                x_segment: XSegment {
                    a: IntPoint::new(108_356_443, -118_534_979),
                    b: IntPoint::new(108_429_851, -119_688_163),
                },
                count: ShapeCountOffset { subj: 1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 8
                x_segment: XSegment {
                    a: IntPoint::new(34_289_766, 4_029_975),
                    b: IntPoint::new(108_429_851, -119_688_163),
                },
                count: ShapeCountOffset {
                    subj: -1,
                    bold: false,
                },
            },
            Segment::<ShapeCountOffset> {
                // 9
                x_segment: XSegment {
                    a: IntPoint::new(34_289_766, 4_029_975),
                    b: IntPoint::new(64_809_862, -14_876_105),
                },
                count: ShapeCountOffset { subj: 1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 10
                x_segment: XSegment {
                    a: IntPoint::new(-175_197_506, -154_404_209),
                    b: IntPoint::new(64_809_862, -14_876_105),
                },
                count: ShapeCountOffset {
                    subj: -1,
                    bold: false,
                },
            },
            Segment::<ShapeCountOffset> {
                // 11
                x_segment: XSegment {
                    a: IntPoint::new(-175_239_714, -153_263_889),
                    b: IntPoint::new(-175_197_506, -154_404_209),
                },
                count: ShapeCountOffset { subj: -1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 12
                x_segment: XSegment {
                    a: IntPoint::new(-175_239_714, -153_263_889),
                    b: IntPoint::new(-158_100_461, -219_055_730),
                },
                count: ShapeCountOffset { subj: 1, bold: false },
            },
            Segment::<ShapeCountOffset> {
                // 13
                x_segment: XSegment {
                    a: IntPoint::new(-166_926_061, -201_796_434),
                    b: IntPoint::new(-158_100_461, -219_055_730),
                },
                count: ShapeCountOffset { subj: -1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 14
                x_segment: XSegment {
                    a: IntPoint::new(-166_926_061, -201_796_434),
                    b: IntPoint::new(-127_176_915, -251_351_325),
                },
                count: ShapeCountOffset { subj: 1, bold: false },
            },
            Segment::<ShapeCountOffset> {
                // 15
                x_segment: XSegment {
                    a: IntPoint::new(-127_893_331, -250_758_333),
                    b: IntPoint::new(-127_176_915, -251_351_325),
                },
                count: ShapeCountOffset { subj: -1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 16
                x_segment: XSegment {
                    a: IntPoint::new(-184_022_779, -146_081_734),
                    b: IntPoint::new(-127_893_331, -250_758_333),
                },
                count: ShapeCountOffset {
                    subj: -1,
                    bold: false,
                },
            },
            Segment::<ShapeCountOffset> {
                // 17
                x_segment: XSegment {
                    a: IntPoint::new(-184_446_683, -142_060_198),
                    b: IntPoint::new(-184_022_779, -146_081_734),
                },
                count: ShapeCountOffset { subj: -1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 18
                x_segment: XSegment {
                    a: IntPoint::new(-184_446_683, -142_060_198),
                    b: IntPoint::new(-183_294_322, -150_692_526),
                },
                count: ShapeCountOffset { subj: 1, bold: false },
            },
            Segment::<ShapeCountOffset> {
                // 19
                x_segment: XSegment {
                    a: IntPoint::new(-187_451_122, -124_999_534),
                    b: IntPoint::new(-183_294_322, -150_692_526),
                },
                count: ShapeCountOffset { subj: -1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 20
                x_segment: XSegment {
                    a: IntPoint::new(-187_451_122, -124_999_534),
                    b: IntPoint::new(-163_529_925, -186_407_681),
                },
                count: ShapeCountOffset { subj: 1, bold: false },
            },
            Segment::<ShapeCountOffset> {
                // 21
                x_segment: XSegment {
                    a: IntPoint::new(-164_397_893, -185_090_145),
                    b: IntPoint::new(-163_529_925, -186_407_681),
                },
                count: ShapeCountOffset { subj: -1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 22
                x_segment: XSegment {
                    a: IntPoint::new(-164_397_893, -185_090_145),
                    b: IntPoint::new(52_567_258, -230_502_654),
                },
                count: ShapeCountOffset { subj: 1, bold: false },
            },
            Segment::<ShapeCountOffset> {
                // 23
                x_segment: XSegment {
                    a: IntPoint::new(31_682_778, -244_054_974),
                    b: IntPoint::new(52_567_258, -230_502_654),
                },
                count: ShapeCountOffset { subj: -1, bold: true },
            },
            Segment::<ShapeCountOffset> {
                // 24
                x_segment: XSegment {
                    a: IntPoint::new(31_682_778, -244_054_974),
                    b: IntPoint::new(100_884_823, -84_374_363),
                },
                count: ShapeCountOffset { subj: 1, bold: false },
            },
        ];

        let mut overlay = OffsetOverlay::new(128);
        overlay.add_segments(&segments);

        let shapes = overlay
            .build_graph_view_with_solver(Default::default())
            .map(|graph| graph.extract_offset(ContourDirection::CounterClockwise, 0))
            .unwrap_or_default();

        assert!(!shapes.is_empty())
    }
}
