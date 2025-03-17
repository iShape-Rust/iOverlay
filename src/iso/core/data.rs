use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use crate::core::overlay::ShapeType;
use crate::geom::line_range::LineRange;
use crate::iso::core::metric::Metric;
use crate::iso::segment::{DgSegment, HzSegment, VrSegment};
use crate::segm::winding_count::{ShapeCountBoolean, WindingCount};
use i_shape::int::shape::IntContour;

#[derive(Clone)]
pub(crate) struct IsoData {
    pub(crate) vr_segments: Vec<VrSegment>,
    pub(crate) hz_segments: Vec<HzSegment>,
    pub(crate) dg_pos_segments: Vec<DgSegment>,
    pub(crate) dg_neg_segments: Vec<DgSegment>,
}

impl IsoData {
    pub(super) fn new(metric: &Metric) -> Self {
        Self {
            vr_segments: Vec::with_capacity(metric.vr_count),
            hz_segments: Vec::with_capacity(metric.hz_count),
            dg_pos_segments: Vec::with_capacity(metric.dg_pos_count),
            dg_neg_segments: Vec::with_capacity(metric.dg_neg_count),
        }
    }

    pub(super) fn add_contours(&mut self, shape_type: ShapeType, contours: &[IntContour]) {
        for contour in contours {
            if contour.len() < 3 {
                continue;
            }
            self.add_contour(contour, shape_type);
        }
    }

    fn add_contour(&mut self, contour: &IntContour, shape_type: ShapeType) {
        let mut iter = contour.iter();

        // our goal add all not degenerate segments
        let mut p0 = if let Some(&p) = iter.next() { p } else { return; };
        let mut p1 = if let Some(&p) = iter.next() { p } else { return; };

        let q0 = p0;
        for &p in &mut iter {
            if Triangle::is_not_line_point(p0, p1, p) {
                p0 = p1;
                p1 = p;
                break;
            }
            p1 = p;
        }

        let q1 = p0;

        let (direct, invert) = ShapeCountBoolean::with_shape_type(shape_type);

        for &p in iter {
            if Triangle::is_line_point(p0, p1, p) {
                p1 = p;
                continue;
            }
            self.add_segment(p0, p1, direct, invert);

            p0 = p1;
            p1 = p;
        }

        let is_q0 = Triangle::is_line_point(p0, p1, q0);
        let is_p1 = Triangle::is_line_point(q0, q1, p1);

        match (is_q0, is_p1) {
            (false, false) => {
                // no one is collinear, most common case
                self.add_segment(p0, p1, direct, invert);
                self.add_segment(p1, q0, direct, invert);
                self.add_segment(q0, q1, direct, invert);
            }
            (true, true) => {
                // all collinear
                if p0 != q1 {
                    self.add_segment(p0, q1, direct, invert);
                }
            }
            (true, false) => {
                // p0, p1, q0 is on same line
                if p0 != q0 {
                    self.add_segment(p0, q0, direct, invert);
                }
                self.add_segment(q0, q1, direct, invert);
            }
            (false, true) => {
                // p1, q0, q1 is on same line
                self.add_segment(p0, p1, direct, invert);
                if p1 != q1 {
                    self.add_segment(p1, q1, direct, invert);
                }
            }
        }
    }

    fn add_segment(&mut self, p0: IntPoint, p1: IntPoint, direct: ShapeCountBoolean, invert: ShapeCountBoolean) {
        if p0.x == p1.x {
            let vr_segm = if p0.y < p1.y {
                VrSegment {
                    x: p0.x,
                    yy: LineRange {
                        min: p0.y,
                        max: p1.y,
                    },
                    count: direct,
                }
            } else {
                VrSegment {
                    x: p0.x,
                    yy: LineRange {
                        min: p1.y,
                        max: p0.y,
                    },
                    count: invert,
                }
            };
            self.vr_segments.push(vr_segm);
        } else if p0.y == p1.y {
            let hz = if p0.x < p1.x {
                HzSegment { y: p0.y, xx: LineRange { min: p0.x, max: p1.x }, count: direct }
            } else {
                HzSegment { y: p0.y, xx: LineRange { min: p1.x, max: p0.x }, count: invert }
            };
            self.hz_segments.push(hz);
        } else {
            let (dg, is_pos) = if p0.x < p1.x {
                (
                    DgSegment { y0: p0.y, xx: LineRange { min: p0.x, max: p1.x }, count: direct },
                    p0.y < p1.y
                )
            } else {
                (
                    DgSegment { y0: p1.y, xx: LineRange { min: p1.x, max: p0.x }, count: invert },
                    p1.y < p0.y
                )
            };

            if is_pos {
                self.dg_pos_segments.push(dg);
            } else {
                self.dg_neg_segments.push(dg);
            }
        }
    }
}
