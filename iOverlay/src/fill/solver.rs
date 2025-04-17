use crate::geom::end::End;
use crate::geom::v_segment::VSegment;
use crate::segm::segment::{NONE, Segment, SegmentFill};
use crate::segm::winding_count::WindingCount;
use crate::util::log::Int;
use i_float::triangle::Triangle;
use i_tree::key::exp::KeyExpCollection;
use i_tree::key::list::KeyExpList;
use i_tree::key::tree::KeyExpTree;

pub(crate) trait FillStrategy<C> {
    fn add_and_fill(this: C, bot: C) -> (C, SegmentFill);
}

pub(crate) struct FillSolver;

impl FillSolver {
    #[inline]
    pub(crate) fn fill<F: FillStrategy<C>, C: WindingCount>(
        is_list: bool,
        segments: &[Segment<C>],
    ) -> Vec<SegmentFill> {
        let count = segments.len();
        if is_list {
            let capacity = count.log2_sqrt().max(4) * 2;
            Self::solve::<F, KeyExpList<VSegment, i32, C>, C>(KeyExpList::new(capacity), segments)
        } else {
            let capacity = count.log2_sqrt().max(8);
            Self::solve::<F, KeyExpTree<VSegment, i32, C>, C>(KeyExpTree::new(capacity), segments)
        }
    }

    #[inline]
    fn solve<F: FillStrategy<C>, S: KeyExpCollection<VSegment, i32, C>, C: WindingCount>(
        mut scan_list: S,
        segments: &[Segment<C>],
    ) -> Vec<SegmentFill> {
        let mut buf = Vec::with_capacity(4);

        let n = segments.len();
        let mut result = vec![NONE; n];
        let mut i = 0;

        while i < n {
            let p = segments[i].x_segment.a;

            buf.push(End {
                index: i,
                point: segments[i].x_segment.b,
            });
            i += 1;

            while i < n && segments[i].x_segment.a == p {
                buf.push(End {
                    index: i,
                    point: segments[i].x_segment.b,
                });
                i += 1;
            }

            if buf.len() > 1 {
                buf.sort_by(|s0, s1| Triangle::clock_order_point(p, s1.point, s0.point));
            }

            let mut sum_count =
                scan_list.first_less_or_equal_by(p.x, C::new(0, 0), |s| s.is_under_point_order(p));
            let mut fill: SegmentFill;

            for se in buf.iter() {
                let sid = unsafe { segments.get_unchecked(se.index) };
                (sum_count, fill) = F::add_and_fill(sid.count, sum_count);
                *unsafe { result.get_unchecked_mut(se.index) } = fill;
                if sid.x_segment.is_not_vertical() {
                    scan_list.insert(sid.x_segment.into(), sum_count, p.x);
                }
            }

            buf.clear();
        }

        result
    }
}
