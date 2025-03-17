use std::cmp::Ordering;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use crate::fill::count_segment::CountSegment;
use crate::fill::solver_list::ScanFillList;
use crate::fill::solver_tree::ScanFillTree;
use crate::geom::end::End;
use crate::segm::segment::{Segment, SegmentFill, NONE};
use crate::segm::winding_count::WindingCount;

pub(super) trait ScanFillStore<C> {
    fn insert(&mut self, segment: CountSegment<C>);
    fn find_under_and_nearest(&mut self, p: IntPoint) -> C;
}

pub(crate) trait FillStrategy<C> {
    fn add_and_fill(this: C, bot: C) -> (C, SegmentFill);
}

pub(crate) struct FillSolver;

impl FillSolver {

    #[inline]
    pub(crate) fn fill<F: FillStrategy<C>, C: WindingCount>(is_list: bool, segments: &[Segment<C>]) -> Vec<SegmentFill> {
        if is_list {
            Self::solve::<F, ScanFillList<C>, C>(ScanFillList::<C>::new(segments.len()), segments)
        } else {
            Self::solve::<F, ScanFillTree<C>, C>(ScanFillTree::<C>::new(segments.len()), segments)
        }
    }

    #[inline]
    fn solve<F: FillStrategy<C>, S: ScanFillStore<C>, C: WindingCount>(mut scan_store: S, segments: &[Segment<C>]) -> Vec<SegmentFill> {
        let mut buf = Vec::with_capacity(4);

        let n = segments.len();
        let mut result = vec![NONE; n];
        let mut i = 0;

        while i < n {
            let p = segments[i].x_segment.a;

            buf.push(End { index: i, point: segments[i].x_segment.b });
            i += 1;

            while i < n && segments[i].x_segment.a == p {
                buf.push(End { index: i, point: segments[i].x_segment.b });
                i += 1;
            }

            if buf.len() > 1 {
                buf.sort_by(|s0, s1|Self::clockwise_order(p, s1.point, s0.point));
            }

            let mut sum_count = scan_store.find_under_and_nearest(p);
            let mut fill: SegmentFill;

            for se in buf.iter() {
                let sid = unsafe { segments.get_unchecked(se.index) };
                (sum_count, fill) = F::add_and_fill(sid.count, sum_count);
                *unsafe { result.get_unchecked_mut(se.index) } = fill;
                if sid.x_segment.is_not_vertical() {
                    scan_store.insert(CountSegment { count: sum_count, x_segment: sid.x_segment });
                }
            }

            buf.clear();
        }

        result
    }

    #[inline(always)]
    fn clockwise_order(p0: IntPoint, p1: IntPoint, p2: IntPoint) -> Ordering {
        let area = Triangle::area_two_point(p0, p1, p2);
        0.cmp(&area)
    }
}