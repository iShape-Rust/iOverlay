use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes};
use crate::core::extract::{JoinHoles, StartPathData, Validate};
use crate::core::fill_rule::FillRule;
use crate::core::overlay::Overlay;
use crate::core::overlay_graph::OverlayGraph;
use crate::core::solver::Solver;
use crate::extension::hole_point::ExclusionIdPoint;
use crate::segm::segment::{CLIP_BOTH, NONE, Segment, SegmentFill, SUBJ_BOTH, SUBJ_BOTTOM};

impl Overlay {
    pub(super) fn slice(self, fill_rule: FillRule, min_area: i64) -> IntShapes {
        let solver = Default::default();
        let (mut segments, mut fills) = self.into_segments(fill_rule, solver);
        clean_if_needed(&mut segments, &mut fills);

        let mut graph = OverlayGraph::new(solver, (segments, fills));
        graph.remove_leaf_links();
        graph.extract_slice_shapes_min_area(min_area)
    }
}

impl OverlayGraph {
    fn filter_slice(&self) -> Vec<u8> {
        self.links.iter().enumerate().map(|(index, link)| {
            if !self.node(link.a.id).is_contain(index) {
                // this link is a leaf and it is removed
                return 0;
            }
            let fill = link.fill;
            let subj = fill & SUBJ_BOTH;
            let one_side_subj = subj != 0 && subj != SUBJ_BOTH;

            if one_side_subj {
                1
            } else if fill & CLIP_BOTH != 0 && subj == SUBJ_BOTH {
                // ony edges inside subj
                // slice edge, we must visit it twice
                2
            } else {
                0
            }
        }).collect()
    }

    fn extract_slice_shapes_min_area(&self, min_area: i64) -> IntShapes {
        let mut binding = self.filter_slice();
        let visited = binding.as_mut_slice();
        let mut holes = Vec::new();
        let mut shapes = Vec::new();
        let mut hole_points = Vec::new();

        let mut link_index = 0;
        while link_index < visited.len() {
            let &count_to_visit = unsafe { visited.get_unchecked(link_index) };
            if count_to_visit == 0 {
                link_index += 1;
                continue;
            }

            let left_top_link = self.find_left_top_link(link_index, visited);
            let link = self.link(left_top_link);
            let &top_link_visited = unsafe { visited.get_unchecked(left_top_link) };

            if top_link_visited == 1 {
                let is_hole = link.fill & SUBJ_BOTTOM != SUBJ_BOTTOM;
                let start_data = StartPathData::new(is_hole, link, left_top_link);
                let mut path = self.get_path(&start_data, visited);

                if path.validate(min_area) {
                    if is_hole {
                        hole_points.push(ExclusionIdPoint {
                            id: holes.len(),
                            exclusion: usize::MAX,
                            point: start_data.begin,
                        });
                        holes.push(path);
                    } else {
                        shapes.push(vec![path]);
                    }
                }
            } else {
                // it's a hole and body at the same time

                // extract hole
                let hole_start_data = StartPathData::new(true, link, left_top_link);
                hole_points.push(ExclusionIdPoint {
                    id: holes.len(),
                    exclusion: shapes.len(),
                    point: hole_start_data.begin,
                });

                let mut hole_path = self.get_path(&hole_start_data, visited);
                if hole_path.validate(min_area) {
                    holes.push(hole_path);
                }

                // extract body
                let body_start_data = StartPathData::new(false, link, left_top_link);
                let mut body_path = self.get_path(&body_start_data, visited);
                if body_path.validate(min_area) {
                    shapes.push(vec![body_path]);
                }
            }
        }

        shapes.join_holes(&self.solver, holes, hole_points);

        shapes
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

trait JoinExclusionHoles {
    fn join_holes(&mut self, solver: &Solver, holes: Vec<IntPath>, hole_points: Vec<ExclusionIdPoint>);
}

impl JoinExclusionHoles for Vec<IntShape> {
    #[inline]
    fn join_holes(&mut self, solver: &Solver, holes: Vec<IntPath>, hole_points: Vec<ExclusionIdPoint>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].reserve_exact(holes.len());
            let mut hole_paths = holes;
            self[0].append(&mut hole_paths);
        } else {
            self.join_holes_by_points(solver, holes, hole_points);
        }
    }
}