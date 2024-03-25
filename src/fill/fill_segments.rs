use std::cmp::Ordering;
use i_float::point::Point;
use i_float::triangle::Triangle;
use crate::bool::fill_rule::FillRule;
use crate::fill::count_segment::CountSegment;
use crate::split::shape_count::ShapeCount;
use crate::fill::segment::{Segment, CLIP_BOTTOM, CLIP_TOP, NONE, SUBJ_BOTTOM, SUBJ_TOP};
use crate::fill::store::ScanFillStore;

struct XGroup {
    i: usize,
    x: i32,
}

struct PGroup {
    i: usize,
    p: Point,
}

pub(crate) trait FillSegments<S: ScanFillStore> {
    fn fill(&mut self, scan_store: S, fill_rule: FillRule);
}

impl<S: ScanFillStore> FillSegments<S> for Vec<Segment> {
    fn fill(&mut self, scan_store: S, fill_rule: FillRule) {
        let mut scan_list = scan_store;
        let mut x_buf = Vec::new();
        let mut p_buf = Vec::new();

        let n = self.len();
        let mut i = 0;

        while i < n {
            let x = self[i].seg.a.x;

            x_buf.clear();

            // find all new segments with same a.x
            while i < n && self[i].seg.a.x == x {
                x_buf.push(XGroup { i, x: self[i].seg.a.y });
                i += 1;
            }

            if x_buf.len() > 1 {
                x_buf.sort_by(|a, b| a.order_by_x(b));
            }

            let mut j = 0;
            while j < x_buf.len() {
                let y = x_buf[j].x;

                p_buf.clear();

                // group new segments by same y (all segments in eBuf must have same a)
                while j < x_buf.len() && x_buf[j].x == y {
                    let handler = &x_buf[j];
                    p_buf.push(PGroup { i: handler.i, p: self[handler.i].seg.b });
                    j += 1;
                }

                let p = Point::new(x, y);

                if p_buf.len() > 1 {
                    p_buf.sort_by(|a, b| a.order_by_angle(b, p));
                }

                let mut sum_count = if let Some(count) = scan_list.find_under(p, x) {
                    count
                } else {
                    ShapeCount::new(0, 0)
                };

                for se in p_buf.iter() {
                    if self[se.i].seg.is_vertical() {
                        _ = self[se.i].add_and_fill(sum_count, fill_rule);
                    } else {
                        sum_count = self[se.i].add_and_fill(sum_count, fill_rule);
                        scan_list.insert(CountSegment { count: sum_count, x_segment: self[se.i].seg }, x);
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

        let subj_top = if is_subj_top { SUBJ_TOP } else { NONE };
        let subj_bottom = if is_subj_bottom { SUBJ_BOTTOM } else { NONE };
        let clip_top = if is_clip_top { CLIP_TOP } else { NONE };
        let clip_bottom = if is_clip_bottom { CLIP_BOTTOM } else { NONE };

        self.fill = subj_top | subj_bottom | clip_top | clip_bottom;

        new_count
    }
}


impl PGroup {
    fn order_by_angle(&self, other: &Self, center: Point) -> Ordering {
        if Triangle::is_clockwise_point(center, other.p, self.p) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl XGroup {
    fn order_by_x(&self, other: &Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}