use i_shape::triangle::Triangle;
use crate::split::shape_count::ShapeCount;
use crate::fill::shape_type::ShapeType;
use crate::fill::segment::{ Segment, SegmentFill };

pub(crate) trait FillSegments {
    fn fill(&mut self);
}

impl FillSegments for Vec<Segment> {

    fn fill(&mut self) {
        let capacity = 2 * (self.len() as f64).sqrt() as usize;
        let mut scan_list: Vec<Segment> = Vec::with_capacity(capacity);
        let n = self.len();
        let mut i = 0;

        while i < n {
            let x = self[i].a.x;
            let i0 = i;

            while i < n {
                let si = self[i];
                if si.a.x == x {
                    if si.b.x != si.a.x {
                        // do not include verticals
                        scan_list.push(si);
                    }
                    i += 1;
                } else {
                    break;
                }
            }

            unsafe {
                for k in i0..i {
                    let segm = self.get_unchecked_mut(k);
                    let mut j = 0;
                    let mut count = ShapeCount::new(0, 0);
                    while j < scan_list.len() {
                        let scan = scan_list.get_unchecked(j);

                        if scan.b.x <= x {
                            scan_list.swap_remove(j);
                        } else {
                            if scan.a.x == segm.a.x && scan.a.y == segm.a.y {
                                // have a common point "a"
                                if Triangle::is_clockwise(scan.a, segm.b, scan.b) {
                                    count = count.increment(scan.shape);
                                }
                            } else if scan.b.x > segm.a.x && Triangle::is_clockwise(scan.a, segm.a, scan.b) {
                                count = count.increment(scan.shape);
                            }

                            j += 1;
                        }
                    }

                    let subj_fill: SegmentFill;
                    let out_subj = count.subj % 2 == 0;
                    if segm.shape.0 & ShapeType::SUBJECT.0 != 0 {
                        subj_fill = if out_subj { SegmentFill::SUBJECT_TOP } else { SegmentFill::SUBJECT_BOTTOM };
                    } else {
                        subj_fill = if out_subj { SegmentFill::NONE } else { SegmentFill::SUBJECT_TOP | SegmentFill::SUBJECT_BOTTOM };
                    }

                    let clip_fill: SegmentFill;
                    let out_clip = count.clip % 2 == 0;
                    if segm.shape & ShapeType::CLIP != ShapeType::NONE {
                        clip_fill = if out_clip { SegmentFill::CLIP_TOP } else { SegmentFill::CLIP_BOTTOM };
                    } else {
                        clip_fill = if out_clip { SegmentFill::NONE } else { SegmentFill::CLIP_TOP | SegmentFill::CLIP_BOTTOM };
                    }

                    segm.fill = subj_fill | clip_fill;
                }
            }
        }
    }
}