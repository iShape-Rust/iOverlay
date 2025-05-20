use alloc::vec::Vec;
use i_tree::ExpiredVal;
use crate::geom::line_range::LineRange;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::Segment;
use crate::segm::winding::WindingCount;
use crate::split::snap_radius::SnapRadius;
use crate::split::solver::SplitSolver;
use i_tree::seg::exp::{SegExpCollection, SegRange};
use i_tree::seg::tree::SegExpTree;
use crate::core::solver::Solver;

#[derive(Debug, Clone, Copy)]
struct IdSegment {
    id: usize,
    x_segment: XSegment,
}

impl ExpiredVal<i32> for IdSegment {
    #[inline]
    fn expiration(&self) -> i32 {
        self.x_segment.b.x
    }
}

impl SplitSolver {
    pub(super) fn tree_split<C: WindingCount>(
        &mut self,
        snap_radius: SnapRadius,
        segments: &mut Vec<Segment<C>>,
        solver: &Solver
    ) -> bool {
        let range: SegRange<i32> = segments.ver_range().into();
        let mut tree: SegExpTree<i32, i32, IdSegment> = if let Some(tree) = SegExpTree::new(range) {
            tree
        } else {
            return self.list_split(snap_radius, segments, solver);
        };

        let mut need_to_fix = true;
        let mut any_intersection = false;

        let mut snap_radius = snap_radius;

        while need_to_fix && segments.len() > 2 {
            need_to_fix = false;
            self.marks.clear();

            let radius = snap_radius.radius();

            for (i, si) in segments.iter().enumerate() {
                let time = si.x_segment.a.x;
                let si_range = si.x_segment.y_range().into();
                for sj in tree.iter_by_range(si_range, time) {
                    let (this_index, scan_index, this, scan) = if si.x_segment < sj.x_segment {
                        (i, sj.id, &si.x_segment, &sj.x_segment)
                    } else {
                        (sj.id, i, &sj.x_segment, &si.x_segment)
                    };

                    let is_round = SplitSolver::cross(
                        this_index,
                        scan_index,
                        this,
                        scan,
                        &mut self.marks,
                        radius,
                    );

                    need_to_fix = is_round || need_to_fix;
                }

                tree.insert_by_range(si_range, si.id_segment(i));
            }

            if self.marks.is_empty() {
                return any_intersection;
            }

            any_intersection = true;
            tree.clear();

            self.apply(segments, need_to_fix, solver);

            snap_radius.increment();
        }

        any_intersection
    }
}

impl From<LineRange> for SegRange<i32> {
    #[inline]
    fn from(value: LineRange) -> Self {
        Self {
            min: value.min,
            max: value.max,
        }
    }
}

trait VerticalRange {
    fn ver_range(&self) -> LineRange;
}

impl<C: Send> VerticalRange for Vec<Segment<C>> {
    fn ver_range(&self) -> LineRange {
        let mut min_y = self[0].x_segment.a.y;
        let mut max_y = min_y;

        for edge in self.iter() {
            min_y = min_y.min(edge.x_segment.a.y);
            max_y = max_y.max(edge.x_segment.a.y);
            min_y = min_y.min(edge.x_segment.b.y);
            max_y = max_y.max(edge.x_segment.b.y);
        }

        LineRange {
            min: min_y,
            max: max_y,
        }
    }
}

impl<C: Send> Segment<C> {
    #[inline]
    fn id_segment(&self, id: usize) -> IdSegment {
        IdSegment {
            id,
            x_segment: self.x_segment,
        }
    }
}
