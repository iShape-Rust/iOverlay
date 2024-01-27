use std::cmp::Ordering;
use i_float::fix_vec::FixVec;
use i_float::point::Point;
use i_shape::triangle::Triangle;
use crate::bool::fill_rule::FillRule;
use crate::split::shape_count::ShapeCount;
use crate::fill::segment::{Segment, CLIP_BOTTOM, CLIP_TOP, NONE, SUBJECT_BOTTOM, SUBJECT_TOP};
use crate::geom::x_scan_list::XScanList;
use crate::geom::x_segment::XSegment;
use crate::space::line_range::LineRange;
use crate::space::scan_space::ScanSegment;

struct Handler {
    i: usize,
    y: i32,
}

struct SegEnd {
    i: usize,
    p: Point,
}

pub(crate) trait FillSegments {
    fn fill(&mut self, fill_rule: FillRule, range: LineRange);
}

impl FillSegments for Vec<Segment> {
    fn fill(&mut self, fill_rule: FillRule, range: LineRange) {
        let mut scan_list = XScanList::new(range, self.len());
        let mut counts = vec![ShapeCount { subj: 0, clip: 0 }; self.len()];
        let mut x_buf = Vec::new();
        let mut e_buf = Vec::new();
        let mut candidates = Vec::new();

        let n = self.len();
        let mut i = 0;

        while i < n {
            let x = self[i].seg.a.x;
            x_buf.clear();

            // find all new segments with same a.x

            while i < n && self[i].seg.a.x == x {
                x_buf.push(Handler { i, y: self[i].seg.a.y });
                i += 1
            }

            if x_buf.len() > 1 {
                // sort all by a.y
                x_buf.sort_by(|a, b| a.order_asc(b));
            }

            // find nearest segment from scan list for all new segments

            let mut j = 0;
            while j < x_buf.len() {
                let y = x_buf[j].y;

                e_buf.clear();

                // group new segments by same y (all segments in eBuf must have same a)
                while j < x_buf.len() && x_buf[j].y == y {
                    let handler = &x_buf[j];
                    e_buf.push(SegEnd { i: handler.i, p: self[handler.i].seg.b });
                    j += 1
                }

                let p = Point::new(x, y);

                if e_buf.len() > 1 {
                    // sort by angle in counter clock-wise direction
                    let center = FixVec::new_point(p);
                    e_buf.sort_by(|a, b| a.order(b, center));
                }

                let mut iterator = scan_list.iterator_to_bottom(y);
                let mut best_segment: Option<XSegment> = None;
                let mut best_index = usize::MAX;

                while iterator.min != i32::MIN {
                    scan_list.space.ids_in_range(iterator, x, &mut candidates);
                    if !candidates.is_empty() {
                        for &seg_index in candidates.iter() {
                            let segment = self[seg_index].seg;
                            if segment.is_under_point(p) {
                                if let Some(best_seg) = best_segment {
                                    if best_seg.is_under_segment(segment) {
                                        best_segment = Some(segment);
                                        best_index = seg_index;
                                    }
                                } else {
                                    best_segment = Some(segment.clone());
                                    best_index = seg_index;
                                }
                            }
                        }
                        candidates.clear();
                    }

                    if let Some(best_seg) = best_segment {
                        if best_seg.is_above_point(Point::new(x, iterator.min)) {
                            break;
                        }
                    }

                    iterator = scan_list.next(iterator);
                }

                let mut sum_count: ShapeCount;
                if best_index != usize::MAX {
                    sum_count = counts[best_index]
                } else {
                    // this is the most bottom segment group
                    sum_count = ShapeCount::new(0, 0);
                }

                for se in e_buf.iter() {
                    if self[se.i].seg.is_vertical() {
                        _ = self[se.i].add_and_fill(sum_count, fill_rule);
                    } else {
                        sum_count = self[se.i].add_and_fill(sum_count, fill_rule);
                        counts[se.i] = sum_count;
                        let seg = &self[se.i].seg;
                        scan_list.space.insert(ScanSegment { id: se.i, range: seg.y_range(), stop: seg.b.x });
                    }
                }
            }
        }
    }
}

impl Segment {
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

        let subj_top = if is_subj_top { SUBJECT_TOP } else { NONE };
        let subj_bottom = if is_subj_bottom { SUBJECT_BOTTOM } else { NONE };
        let clip_top = if is_clip_top { CLIP_TOP } else { NONE };
        let clip_bottom = if is_clip_bottom { CLIP_BOTTOM } else { NONE };

        self.fill = subj_top | subj_bottom | clip_top | clip_bottom;

        new_count
    }
}

impl Handler {
    pub(super) fn order_asc(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl SegEnd {
    pub(super) fn order(&self, other: &Self, center: FixVec) -> Ordering {
        if Triangle::is_clockwise(center, FixVec::new_point(other.p), FixVec::new_point(self.p)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}