use crate::build::builder::{FillStrategy, GraphBuilder};
use crate::core::graph::{OverlayGraph, OverlayNode};
use crate::core::solver::Solver;
use crate::mesh::boolean::ShapeCountString;
use crate::segm::segment::{Segment, SegmentFill};

impl GraphBuilder<ShapeCountString, OverlayNode> {
    #[inline]
    pub(crate) fn build_offset(&mut self,
                               solver: &Solver,
                               segments: &[Segment<ShapeCountString>],
    ) -> OverlayGraph {
        
        self.build_boolean_fills(fill_rule, solver, segments);
        self.build_links_all(segments);
        self.boolean_graph(solver)
    }
}

/*

    #[inline]
    pub(crate) fn offset_graph_with_solver(mut segments: Vec<Segment<OffsetCountBoolean>>, split_solver: &mut SplitSolver, solver: Solver) -> Option<OverlayGraph> {
        split_solver.split_segments(&mut segments, &solver);
        if segments.is_empty() {
            return None;
        }

        
        let is_list = solver.is_list_fill(&segments);
        let fills = GraphBuilder::fill::<SubjectOffsetStrategy, OffsetCountBoolean>(is_list, &segments);
        let links = OverlayLinkBuilder::build_all_links(&segments, &fills);
        OverlayGraph::new(solver, links)
    }
 */

struct SubjectOffsetStrategy;
const BOLD_BIT: usize = 2;

impl FillStrategy<ShapeCountString> for SubjectOffsetStrategy {

    #[inline(always)]
    fn add_and_fill(this: ShapeCountString, bot: ShapeCountString) -> (ShapeCountString, SegmentFill) {
        let top_subj = bot.subj + this.subj;
        let bot_subj = bot.subj;

        let subj_top = (top_subj > 0) as SegmentFill;
        let subj_bot = (bot_subj > 0) as SegmentFill;

        let bold = this.bold as SegmentFill;

        let fill = subj_top | (subj_bot << 1) | (bold << BOLD_BIT);
        let top = ShapeCountString { subj: top_subj, bold: false }; // bold not need

        (top, fill)
    }
}