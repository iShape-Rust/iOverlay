use i_shape::int::path::IntPath;
use i_shape::int::shape::IntShapes;
use crate::bind::segment::{ContourIndex, IdSegment};
use crate::bind::solver::{JoinHoles, LeftBottomSegment};
use crate::core::extract::{StartPathData, Validate, Visit};
use crate::core::filter::MaskFilter;
use crate::core::graph::OverlayGraph;
use crate::core::link::{OverlayLink, OverlayLinkBuilder};
use crate::core::node::OverlayNode;
use crate::core::overlay::{ContourDirection, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::core::solver::Solver;
use crate::fill::solver::{FillSolver, FillStrategy};
use crate::geom::v_segment::VSegment;
use crate::segm::segment::{Segment, SegmentFill, SUBJ_TOP};
use crate::segm::winding_count::WindingCount;
use crate::split::solver::SplitSegments;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OffsetCountBoolean {
    pub(crate) subj: i32,
    pub(crate) bold: bool,
}

struct SubjectOffsetStrategy;
const BOLD_BIT: usize = 2;

impl FillStrategy<OffsetCountBoolean> for SubjectOffsetStrategy {

    #[inline(always)]
    fn add_and_fill(this: OffsetCountBoolean, bot: OffsetCountBoolean) -> (OffsetCountBoolean, SegmentFill) {
        let top_subj = bot.subj + this.subj;
        let bot_subj = bot.subj;

        let subj_top = (top_subj < 0) as SegmentFill;
        let subj_bot = (bot_subj < 0) as SegmentFill;

        let bold = this.bold as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (bold << BOLD_BIT);
        let top = OffsetCountBoolean { subj: top_subj, bold: false }; // bold not need

        (top, fill)
    }
}

impl OverlayLink {

    #[inline(always)]
    fn is_bold(&self) -> bool {
        self.fill & (1 << BOLD_BIT) != 0
    }

}

impl WindingCount for OffsetCountBoolean {
    #[inline(always)]
    fn is_not_empty(&self) -> bool { self.subj != 0 }

    #[inline(always)]
    fn new(subj: i32, _: i32) -> Self {
        Self {subj, bold: true}
    }

    #[inline(always)]
    fn with_shape_type(shape_type: ShapeType) -> (Self, Self) {
        match shape_type {
            ShapeType::Subject => (Self {subj: 1, bold: true}, Self {subj: -1, bold: true}),
            ShapeType::Clip => (Self {subj: 0, bold: true}, Self {subj: 0, bold: true}),
        }
    }

    #[inline(always)]
    fn add(self, count: Self) -> Self {
        let subj = self.subj + count.subj;
        let bold = self.bold || count.bold;
        Self {subj, bold}
    }

    #[inline(always)]
    fn apply(&mut self, count: Self) {
        self.subj += count.subj;
        self.bold = self.bold || count.bold;
    }

    #[inline(always)]
    fn invert(self) -> Self {
        let subj = -self.subj;
        Self {subj, bold: self.bold}
    }
}

impl OverlayGraph {
    #[inline]
    pub(crate) fn offset_graph_with_solver(segments: Vec<Segment<OffsetCountBoolean>>, solver: Solver) -> OverlayGraph {
        if segments.is_empty() { return OverlayGraph::new(solver, vec![]); }
        let segments = segments.split_segments(solver);
        if segments.is_empty() { return OverlayGraph::new(solver, vec![]); }
        let is_list = solver.is_list_fill(&segments);
        let fills = FillSolver::fill::<SubjectOffsetStrategy, OffsetCountBoolean>(is_list, &segments);
        let links = OverlayLinkBuilder::build_all_links(&segments, &fills);
        OverlayGraph::new(solver, links)
    }

    pub(crate) fn extract_offset(&self, main_direction: ContourDirection, min_area: usize) -> IntShapes {
        let visited = self.links.filter_by_rule(OverlayRule::Subject);
        self.extract_offset_shapes(visited, main_direction, min_area)
    }

    fn extract_offset_shapes(
        &self,
        filter: Vec<bool>,
        main_direction: ContourDirection,
        min_area: usize,
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

            let left_top_link = self.find_left_top_link(link_index, visited);
            let link = self.link(left_top_link);
            let is_hole = link.fill & SUBJ_TOP == SUBJ_TOP;
            let mut bold = link.is_bold();
            let direction = is_hole == clockwise;

            let start_data = StartPathData::new(direction, link, left_top_link);

            let mut path = self.get_fill_path(&start_data, direction, &mut bold, visited);
            if !bold {
                link_index += 1;
                continue;
            }

            let (is_valid, is_modified) = path.validate(min_area);

            if !is_valid {
                link_index += 1;
                continue;
            }

            if is_hole {
                let mut v_segment = if clockwise {
                    VSegment {
                        a: path[1],
                        b: path[2],
                    }
                } else {
                    VSegment {
                        a: path[0],
                        b: path[path.len() - 1],
                    }
                };
                if is_modified {
                    let most_left = path.left_bottom_segment();
                    if most_left != v_segment {
                        v_segment = most_left;
                        is_all_anchors_sorted = false;
                    }
                };

                debug_assert_eq!(v_segment, path.left_bottom_segment());
                let id = ContourIndex::new_hole(holes.len());
                anchors.push(IdSegment::with_segment(id, v_segment));
                holes.push(path);
            } else {
                shapes.push(vec![path]);
            }
        }

        if !is_all_anchors_sorted {
            anchors.sort_by(|s0, s1| s0.v_segment.a.cmp(&s1.v_segment.a));
        }

        shapes.join_sorted_holes(&self.solver, holes, anchors, clockwise);

        shapes
    }

    fn get_fill_path(
        &self,
        start_data: &StartPathData,
        clockwise: bool,
        bold: &mut bool,
        visited: &mut [bool],
    ) -> IntPath {
        let mut link_id = start_data.link_id;
        let mut node_id = start_data.node_id;
        let last_node_id = start_data.last_node_id;

        visited.visit(link_id);

        let mut path = IntPath::new();
        path.push(start_data.begin);

        // Find a closed tour
        while node_id != last_node_id {
            let node = self.node(node_id);
            link_id = match node {
                OverlayNode::Bridge(bridge) => {
                    if bridge[0] == link_id {
                        bridge[1]
                    } else {
                        bridge[0]
                    }
                }
                OverlayNode::Cross(indices) => {
                    self.find_nearest_link_to(link_id, node_id, clockwise, indices, visited)
                }
            };

            let link = self.link(link_id);
            node_id = if link.a.id == node_id {
                path.push(link.a.point);
                link.b.id
            } else {
                path.push(link.b.point);
                link.a.id
            };

            *bold = *bold || link.is_bold();

            visited.visit(link_id);
        }

        path
    }
}

