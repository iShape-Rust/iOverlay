use crate::build::builder::{FillStrategy, GraphBuilder};
use crate::core::graph::OverlayNode;
use crate::core::link::OverlayLink;
use crate::core::solver::Solver;
use crate::mesh::graph::OffsetGraph;
use crate::segm::offset::ShapeCountOffset;
use crate::segm::segment::{Segment, SegmentFill};

impl GraphBuilder<ShapeCountOffset, OverlayNode> {
    #[inline]
    pub(crate) fn build_offset(&mut self,
                               solver: &Solver,
                               segments: &[Segment<ShapeCountOffset>],
    ) -> OffsetGraph {
        self.build_fills_with_strategy::<SubjectOffsetStrategy>(solver, segments);
        self.build_links_all(segments);
        self.offset_graph(solver)
    }

    #[inline]
    fn offset_graph(&mut self, solver: &Solver) -> OffsetGraph {
        self.build_nodes_and_connect_links(solver);
        OffsetGraph {
            nodes: &self.nodes,
            links: &self.links,
        }
    }
}

struct SubjectOffsetStrategy;
const BOLD_BIT: usize = 2;

impl FillStrategy<ShapeCountOffset> for SubjectOffsetStrategy {

    #[inline(always)]
    fn add_and_fill(this: ShapeCountOffset, bot: ShapeCountOffset) -> (ShapeCountOffset, SegmentFill) {
        let top_subj = bot.subj + this.subj;
        let bot_subj = bot.subj;

        let subj_top = (top_subj > 0) as SegmentFill;
        let subj_bot = (bot_subj > 0) as SegmentFill;

        let bold = this.bold as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (bold << BOLD_BIT);
        let top = ShapeCountOffset { subj: top_subj, bold: false }; // bold not need

        (top, fill)
    }
}


impl OverlayLink {
    #[inline(always)]
    pub(crate) fn is_bold(&self) -> bool {
        self.fill & (1 << BOLD_BIT) != 0
    }
}