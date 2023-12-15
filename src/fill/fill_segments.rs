use std::cmp::Ordering;
use i_float::fix_vec::FixVec;
use i_shape::triangle::Triangle;
use crate::bool::fill_rule::FillRule;
use crate::fill::fill_scan_list::FillScanList;
use crate::split::shape_count::ShapeCount;
use crate::fill::segment::{Segment, SegmentFill};
use crate::space::line_range::LineRange;
use crate::space::line_space::LineSegment;

struct Handler {
    i: usize,
    y: i32,
}

struct SegEnd {
    i: usize,
    p: FixVec,
}

pub(crate) trait FillSegments {
    fn fill(&mut self, fill_rule: FillRule);
}

impl FillSegments for Vec<Segment> {
    fn fill(&mut self, fill_rule: FillRule) {
        let mut scan_list = FillScanList::new(self);

        let mut counts = vec![ShapeCount { subj: 0, clip: 0 }; self.len()];
        let mut x_buf = Vec::new();
        let mut e_buf = Vec::new();
        let mut r_buf = Vec::new();

        let n = self.len();
        let mut i = 0;

        while i < n {
            let x = self[i].a.x.value();
            x_buf.clear();

            // find all new segments with same a.x

            while i < n && self[i].a.x.value() == x {
                x_buf.push(Handler { i, y: self[i].a.y.value() as i32 });
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
                    e_buf.push(SegEnd { i: handler.i, p: self[handler.i].b });
                    j += 1
                }

                if e_buf.len() > 1 {
                    // sort by angle in counter clock-wise direction
                    e_buf.sort_by(|a, b| a.order(b, FixVec::new_i64(x, y as i64)));
                }

                // find nearest scan segment for y
                let mut iterator = scan_list.iterator_to_bottom(y);
                let mut best_y = i32::MAX;
                let mut best_index = usize::MAX;

                while best_index == usize::MAX && iterator.min != i32::MIN {
                    let candidates = scan_list.all_in_range(iterator);
                    r_buf.clear();

                    for candidate in candidates {
                        let seg_index = candidate.id;

                        if self[seg_index].b.x.value() <= x {
                            r_buf.push(candidate.index)
                        } else {
                            let cy = self[seg_index].vertical_intersection(x) as i32;

                            if cy <= y {
                                if best_index == usize::MAX {
                                    if cy == y {
                                        if Triangle::is_clockwise(FixVec::new_i64(x, cy as i64), self[seg_index].b, self[seg_index].a) {
                                            best_index = seg_index;
                                            best_y = cy;
                                        }
                                    } else {
                                        best_index = seg_index;
                                        best_y = cy;
                                    }
                                } else {
                                    if best_y == cy {
                                        if self[best_index].under(self[seg_index], FixVec::new_i64(x, cy as i64)) {
                                            best_index = seg_index;
                                            best_y = cy;
                                        }
                                    } else if best_y < cy {
                                        best_index = seg_index;
                                        best_y = cy;
                                    }
                                }
                            }
                        }
                    }

                    if !r_buf.is_empty() {
                        scan_list.remove(&mut r_buf);
                    }

                    if best_index == usize::MAX {
                        iterator = scan_list.next(iterator);
                    }
                }

                let mut sum_count: ShapeCount;
                if best_index != usize::MAX {
                    sum_count = counts[best_index]
                } else {
                    // this is the most bottom segment group
                    sum_count = ShapeCount::new(0, 0);
                }

                for se in e_buf.iter() {
                    if self[se.i].is_vertical() {
                        _ = self[se.i].add_and_fill(sum_count, fill_rule);
                    } else {
                        sum_count = self[se.i].add_and_fill(sum_count, fill_rule);
                        counts[se.i] = sum_count;
                        scan_list.insert(LineSegment { id: se.i, range: self[se.i].vertical_range() });
                    }
                }
            }
        }
    }
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn vertical_range(&self) -> LineRange {
        if self.a.y > self.b.y {
            LineRange { min: self.b.y.value() as i32, max: self.a.y.value() as i32 }
        } else {
            LineRange { min: self.a.y.value() as i32, max: self.b.y.value() as i32 }
        }
    }

    fn vertical_intersection(&self, x: i64) -> i64 {
        let y01 = (self.a.y - self.b.y).value();
        let x01 = (self.a.x - self.b.x).value();
        let xx0 = x - self.a.x.value();

        (y01 * xx0) / x01 + self.a.y.value()
    }

    fn under(&self, other: Segment, cross: FixVec) -> bool {
        if self.a == other.a {
            Triangle::is_clockwise(self.a, other.b, self.b)
        } else if self.b == other.b {
            Triangle::is_clockwise(self.b, self.a, other.a)
        } else {
            // probably this case impossible
            Triangle::is_clockwise(cross, other.b, self.b)
        }
    }

    fn add_and_fill(&mut self, sum_count: ShapeCount, fill_rule: FillRule) -> ShapeCount {
        let subj_top: SegmentFill;
        let subj_bottom: SegmentFill;
        let clip_top: SegmentFill;
        let clip_bottom: SegmentFill;

        let new_count = sum_count.add(self.count);

        match fill_rule {
            FillRule::EvenOdd => {
                let s_top = 1 & new_count.subj;
                let s_bottom = 1 & sum_count.subj;

                let c_top = 1 & new_count.clip;
                let c_bottom = 1 & sum_count.clip;

                subj_top = if s_top == 1 { SegmentFill::SUBJECT_TOP } else { SegmentFill::NONE };
                subj_bottom = if s_bottom == 1 { SegmentFill::SUBJECT_BOTTOM } else { SegmentFill::NONE };
                clip_top = if c_top == 1 { SegmentFill::CLIP_TOP } else { SegmentFill::NONE };
                clip_bottom = if c_bottom == 1 { SegmentFill::CLIP_BOTTOM } else { SegmentFill::NONE };
            }
            FillRule::NonZero => {
                if self.count.subj == 0 {
                    subj_top = if sum_count.subj != 0 { SegmentFill::SUBJECT_TOP } else { SegmentFill::NONE };
                    subj_bottom = if sum_count.subj != 0 { SegmentFill::SUBJECT_BOTTOM } else { SegmentFill::NONE };
                } else {
                    subj_top = if new_count.subj != 0 { SegmentFill::SUBJECT_TOP } else { SegmentFill::NONE };
                    subj_bottom = if sum_count.subj != 0 { SegmentFill::SUBJECT_BOTTOM } else { SegmentFill::NONE };
                }
                if self.count.clip == 0 {
                    clip_top = if sum_count.clip != 0 { SegmentFill::CLIP_TOP } else { SegmentFill::NONE };
                    clip_bottom = if sum_count.clip != 0 { SegmentFill::CLIP_BOTTOM } else { SegmentFill::NONE };
                } else {
                    clip_top = if new_count.clip != 0 { SegmentFill::CLIP_TOP } else { SegmentFill::NONE };
                    clip_bottom = if sum_count.clip != 0 { SegmentFill::CLIP_BOTTOM } else { SegmentFill::NONE };
                }
            }
        }

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
        if Triangle::is_clockwise(center, other.p, self.p) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}