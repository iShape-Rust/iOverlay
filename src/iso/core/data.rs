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

    pub(crate) fn add_contours(&mut self, shape_type: ShapeType, contours: &[IntContour]) {
        let (direct, invert) = ShapeCountBoolean::with_shape_type(shape_type);

        for contour in contours {
            if contour.len() < 3 {
                continue;
            }

            let mut p0 = contour.last().unwrap().clone();

            for &p1 in contour {
                if p0 == p1 {
                    p0 = p1;
                    continue;
                }
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
                p0 = p1;
            }
        }
    }
}
