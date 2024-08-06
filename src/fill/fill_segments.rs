use std::cmp::Ordering;
use i_float::point::IntPoint;
use i_float::triangle::Triangle;
use crate::core::fill_rule::FillRule;
use crate::fill::count_segment::CountSegment;
use crate::fill::scan_list::ScanFillList;
use crate::fill::scan_tree::ScanFillTree;
use crate::split::shape_count::ShapeCount;
use crate::fill::segment::{Segment, CLIP_BOTTOM, CLIP_TOP, NONE, SUBJ_BOTTOM, SUBJ_TOP};

struct BGroup {
    id: usize,
    b: IntPoint,
}

pub(super) trait ScanFillStore {
    fn insert(&mut self, segment: CountSegment);

    fn find_under_and_nearest(&mut self, p: IntPoint) -> ShapeCount;
}

pub(crate) trait FillSegments {
    fn fill(&mut self, fill_rule: FillRule, is_list: bool);
}

impl FillSegments for Vec<Segment> {
    fn fill(&mut self, fill_rule: FillRule, is_list: bool) {
        if is_list {
            self.solve(ScanFillList::new(self.len()), fill_rule);
        } else {
            self.solve(ScanFillTree::new(self.len()), fill_rule);
        }
    }
}

trait FillSolver<S: ScanFillStore> {
    fn solve(&mut self, scan_store: S, fill_rule: FillRule);
}

impl<S: ScanFillStore> FillSolver<S> for Vec<Segment> {
    fn solve(&mut self, scan_store: S, fill_rule: FillRule) {
        // Mark. self is sorted by seg.a

        let mut scan_list = scan_store;
        let mut buf = Vec::new();

        let n = self.len();
        let mut i = 0;

        while i < n {
            let p = self[i].seg.a;

            buf.clear();

            while i < n && self[i].seg.a == p {
                buf.push(BGroup { id: i, b: self[i].seg.b });
                i += 1;
            }

            buf.sort_unstable_by(|s0, s1|
            if Triangle::is_clockwise_point(p, s1.b, s0.b) {
                Ordering::Less
            } else {
                Ordering::Greater
            });

            let mut sum_count = scan_list.find_under_and_nearest(p);

            for se in buf.iter() {
                let sid = unsafe { self.get_unchecked_mut(se.id) };
                sum_count = sid.add_and_fill(sum_count, fill_rule);
                if !sid.seg.is_vertical() {
                    scan_list.insert(CountSegment { count: sum_count, x_segment: sid.seg });
                }
            }
        }
    }
}

impl Segment {
    #[inline]
    fn add_and_fill(&mut self, sum_count: ShapeCount, fill_rule: FillRule) -> ShapeCount {
        let is_subj_top: bool;
        let is_subj_bottom: bool;
        let is_clip_top: bool;
        let is_clip_bottom: bool;

        let new_count = sum_count.add(self.count);

        match fill_rule {
            FillRule::EvenOdd => {
                is_subj_top = 1 & new_count.subj == 1;
                is_subj_bottom = 1 & sum_count.subj == 1;

                is_clip_top = 1 & new_count.clip == 1;
                is_clip_bottom = 1 & sum_count.clip == 1;
            }
            FillRule::NonZero => {
                is_subj_top = new_count.subj != 0;
                is_subj_bottom = sum_count.subj != 0;

                is_clip_top = new_count.clip != 0;
                is_clip_bottom = sum_count.clip != 0;
            }
        }

        let subj_top = if is_subj_top { SUBJ_TOP } else { NONE };
        let subj_bottom = if is_subj_bottom { SUBJ_BOTTOM } else { NONE };
        let clip_top = if is_clip_top { CLIP_TOP } else { NONE };
        let clip_bottom = if is_clip_bottom { CLIP_BOTTOM } else { NONE };

        self.fill = subj_top | subj_bottom | clip_top | clip_bottom;

        new_count
    }
}