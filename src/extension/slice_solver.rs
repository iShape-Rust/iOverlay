use i_shape::int::shape::IntShapes;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::Overlay;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::overlay_rule::FillTopStrategy;
use crate::segm::segment::{CLIP_BOTH, NONE, Segment, SegmentFill, SUBJ_BOTH, SUBJ_BOTTOM};

impl Overlay {
    pub(super) fn slice(self, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let solver = Default::default();
        let (mut segments, mut fills) = self.into_segments(fill_rule, solver);
        clean_if_needed(&mut segments, &mut fills);

        let graph = OverlayGraph::new(solver, (segments, fills));
        let mut visited = graph.filter_slice();
        graph.extract_shapes_min_area_visited::<SliceStrategy>(min_area, &mut visited)
    }
}

impl OverlayGraph {
    fn filter_slice(&self) -> Vec<u8> {
        self.links.iter().map(|link| {
            let fill = link.fill;
            let subj = fill & SUBJ_BOTH;
            let one_side_subj = subj != 0 && subj != SUBJ_BOTH;

            if one_side_subj {
                1
            } else if fill & CLIP_BOTH != 0 && subj == SUBJ_BOTH {
                // slice edge, we must visit it twice
                2
            } else {
                0
            }
        }).collect()
    }
}

struct SliceStrategy;

impl FillTopStrategy for SliceStrategy {
    #[inline(always)]
    fn is_fill_top(fill: SegmentFill) -> bool {
        fill & SUBJ_BOTTOM == 0
    }
}

trait Fill {
    fn is_empty(&self) -> bool;
}

impl Fill for SegmentFill {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        *self == NONE || *self == SUBJ_BOTH || *self & SUBJ_BOTH == 0
    }
}

#[inline(always)]
fn clean_if_needed(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>) {
    if let Some(first_empty_index) = fills.iter().position(|fill| fill.is_empty()) {
        clean(segments, fills, first_empty_index);
    }
}

fn clean(segments: &mut Vec<Segment>, fills: &mut Vec<SegmentFill>, after: usize) {
    let mut j = after;

    for i in (after + 1)..fills.len() {
        if !fills[i].is_empty() {
            fills[j] = fills[i];
            segments[j] = segments[i];
            j += 1;
        }
    }

    fills.truncate(j);
    segments.truncate(j);
}