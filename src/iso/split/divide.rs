use i_float::int::point::IntPoint;
use crate::geom::x_segment::XSegment;
use crate::iso::core::data::IsoData;
use crate::iso::segment::DgSegment;
use crate::iso::split::column::{SplitPoint, SplitResult};
use crate::iso::split::diagonal::Diagonal;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;

impl IsoData {

    pub(super) fn divide(&mut self, split: SplitResult) -> Vec<Segment<ShapeCountBoolean>> {
        let capacity = split.count() + split.vr_points.len() + split.hz_points.len()
            + split.dg_pos_points.len() + split.dg_neg_points.len() + self.vr_segments.len()
            + self.hz_segments.len() + self.dg_pos_segments.len() + self.dg_neg_segments.len();
        let mut segments = Vec::with_capacity(capacity);

        self.divide_vr(split.vr_points, &mut segments);

        segments
    }

    fn divide_vr(&self, mut vr_points: Vec<SplitPoint>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        vr_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));

        let mut i = 0;
        while i < vr_points.len() {
            let index = vr_points[i].index;
            let vr = &self.vr_segments[index];
            let mut y = vr.yy.min;
            while i < vr_points.len() && vr_points[i].index == index {
                let sp = &vr_points[i];
                if y != sp.xy {
                    segments.push(Segment {
                        x_segment: XSegment {
                            a: IntPoint::new(vr.x, y),
                            b: IntPoint::new(vr.x, sp.xy),
                        },
                        count: vr.count
                    });
                    y = sp.xy;
                }
            }
            if y != vr.yy.max {
                segments.push(Segment {
                    x_segment: XSegment {
                        a: IntPoint::new(vr.x, y),
                        b: IntPoint::new(vr.x, vr.yy.max),
                    },
                    count: vr.count
                });
            }
            i += 1
        }


    }

    fn divide_hz(&self, mut hz_points: Vec<SplitPoint>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        hz_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));

        let mut i = 0;
        while i < hz_points.len() {
            let index = hz_points[i].index;
            let hz = &self.hz_segments[index];
            let mut x = hz.xx.min;
            while i < hz_points.len() && hz_points[i].index == index {
                let sp = &hz_points[i];
                if x != sp.xy {
                    segments.push(Segment {
                        x_segment: XSegment {
                            a: IntPoint::new(x, hz.y),
                            b: IntPoint::new(sp.xy, hz.y),
                        },
                        count: hz.count
                    });
                    x = sp.xy;
                }
                i += 1
            }
            if x != hz.xx.max {
                segments.push(Segment {
                    x_segment: XSegment {
                        a: IntPoint::new(x, hz.y),
                        b: IntPoint::new(hz.xx.max, hz.y),
                    },
                    count: hz.count
                });
            }
        }
    }

    fn divide_dg<F:GetXY>(&self, mut dg_pos_points: Vec<SplitPoint>, segments: &mut Vec<Segment<ShapeCountBoolean>>) {
        dg_pos_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));

        let mut i = 0;
        while i < dg_pos_points.len() {
            let index = dg_pos_points[i].index;
            let dg = &self.dg_pos_segments[index];
            let mut x = dg.xx.min;
            let mut y = dg.y0;
            while i < dg_pos_points.len() && dg_pos_points[i].index == index {
                let sp = &dg_pos_points[i];
                if x != sp.xy {
                    let yi = F::get_y(dg, x);
                    segments.push(Segment {
                        x_segment: XSegment {
                            a: IntPoint::new(x, y),
                            b: IntPoint::new(sp.xy, yi),
                        },
                        count: dg.count
                    });
                    y = yi;
                }
                i += 1
            }
            if x != dg.xx.max {
                segments.push(Segment {
                    x_segment: XSegment {
                        a: IntPoint::new(x, y),
                        b: IntPoint::new(dg.xx.max, F::get_y(dg, dg.xx.max)),
                    },
                    count: dg.count
                });
            }
        }
    }
}

trait GetXY {
    fn get_x(segm: &DgSegment, y: i32) -> i32;
    fn get_y(segm: &DgSegment, x: i32) -> i32;
}

struct PosFrag;

impl GetXY for PosFrag {
    #[inline(always)]
    fn get_x(segm: &DgSegment, y: i32) -> i32 {
        y.wrapping_sub(segm.pos_b())
    }

    #[inline(always)]
    fn get_y(segm: &DgSegment, x: i32) -> i32 {
        x.wrapping_add(segm.pos_b())
    }
}

struct NegFrag;
impl GetXY for NegFrag {
    #[inline(always)]
    fn get_x(segm: &DgSegment, y: i32) -> i32 {
        segm.neg_b().wrapping_sub(y)
    }

    #[inline(always)]
    fn get_y(segm: &DgSegment, x: i32) -> i32 {
        segm.neg_b().wrapping_sub(x)
    }
}