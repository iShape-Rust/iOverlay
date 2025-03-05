use crate::geom::x_segment::XSegment;
use crate::iso::core::data::IsoData;
use crate::iso::segment::{DgSegment, HzSegment, VrSegment};
use crate::iso::split::column::{SplitPoint, SplitResult};
use crate::iso::split::diagonal::Diagonal;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;
use i_float::int::point::IntPoint;

impl IsoData {
    pub(super) fn divide(self, split: SplitResult) -> Vec<Segment<ShapeCountBoolean>> {
        let capacity = split.vr_points.len()
            + split.hz_points.len()
            + split.dg_pos_points.len()
            + split.dg_neg_points.len()
            + self.vr_segments.len()
            + self.hz_segments.len()
            + self.dg_pos_segments.len()
            + self.dg_neg_segments.len();
        let mut segments = Vec::with_capacity(capacity);

        Self::divide_vr(&self.vr_segments, split.vr_points, &mut segments);
        Self::divide_hz(&self.hz_segments, split.hz_points, &mut segments);
        Self::divide_dg::<PosDiagonal>(&self.dg_pos_segments, split.dg_pos_points, &mut segments);
        Self::divide_dg::<NegDiagonal>(&self.dg_pos_segments, split.dg_neg_points, &mut segments);

        segments
    }

    fn divide_vr(
        vr_segments: &[VrSegment],
        mut vr_points: Vec<SplitPoint>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        vr_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));

        let mut i = 0;
        let mut j = 0;
        while i < vr_points.len() {
            let index = vr_points[i].index;
            let vr = &vr_segments[index];
            let mut y = vr.yy.min;
            while i < vr_points.len() && vr_points[i].index == index {
                let sp = &vr_points[i];
                if y != sp.xy {
                    segments.push(Segment {
                        x_segment: XSegment {
                            a: IntPoint::new(vr.x, y),
                            b: IntPoint::new(vr.x, sp.xy),
                        },
                        count: vr.count,
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
                    count: vr.count,
                });
            }

            if j < index {
                Self::create_vr_segments(&vr_segments[j..index], segments);
                j = index + 1;
            }

            i += 1
        }

        if j < vr_segments.len() {
            Self::create_vr_segments(&vr_segments[j..], segments);
        }
    }

    fn divide_hz(
        hz_segments: &[HzSegment],
        mut hz_points: Vec<SplitPoint>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        hz_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));

        let mut i = 0;
        let mut j = 0;
        while i < hz_points.len() {
            let index = hz_points[i].index;
            let hz = &hz_segments[index];
            let mut x = hz.xx.min;
            while i < hz_points.len() && hz_points[i].index == index {
                let xi = hz_points[i].xy;
                if x != xi {
                    segments.push(Segment {
                        x_segment: XSegment {
                            a: IntPoint::new(x, hz.y),
                            b: IntPoint::new(xi, hz.y),
                        },
                        count: hz.count,
                    });
                    x = xi;
                }
                i += 1
            }
            if x != hz.xx.max {
                segments.push(Segment {
                    x_segment: XSegment {
                        a: IntPoint::new(x, hz.y),
                        b: IntPoint::new(hz.xx.max, hz.y),
                    },
                    count: hz.count,
                });
            }

            if j < index {
                Self::create_hz_segments(&hz_segments[j..index], segments);
                j = index + 1;
            }
        }

        if j < hz_segments.len() {
            Self::create_hz_segments(&hz_segments[j..], segments);
        }
    }

    fn divide_dg<F: GetY>(
        dg_segments: &[DgSegment],
        mut dg_points: Vec<SplitPoint>,
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        dg_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));

        let mut i = 0;
        let mut j = 0;
        while i < dg_points.len() {
            let index = dg_points[i].index;
            let dg = &dg_segments[index];
            let mut x = dg.xx.min;
            let mut y = dg.y0;
            while i < dg_points.len() && dg_points[i].index == index {
                let xi = dg_points[i].xy;
                if x != xi {
                    let yi = F::get_y(dg, xi);
                    segments.push(Segment {
                        x_segment: XSegment {
                            a: IntPoint::new(x, y),
                            b: IntPoint::new(xi, yi),
                        },
                        count: dg.count,
                    });
                    x = xi;
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
                    count: dg.count,
                });
            }

            if j < index {
                Self::create_dg_segments::<F>(&dg_segments[j..index], segments);
                j = index + 1;
            }
        }
    }

    fn create_vr_segments(
        vr_segments: &[VrSegment],
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        for vr in vr_segments.iter() {
            segments.push(Segment {
                x_segment: XSegment {
                    a: IntPoint::new(vr.x, vr.yy.min),
                    b: IntPoint::new(vr.x, vr.yy.max),
                },
                count: vr.count,
            });
        }
    }

    fn create_hz_segments(
        hz_segments: &[HzSegment],
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        for hz in hz_segments.iter() {
            segments.push(Segment {
                x_segment: XSegment {
                    a: IntPoint::new(hz.xx.min, hz.y),
                    b: IntPoint::new(hz.xx.max, hz.y),
                },
                count: hz.count,
            });
        }
    }

    fn create_dg_segments<F: GetY>(
        dg_segments: &[DgSegment],
        segments: &mut Vec<Segment<ShapeCountBoolean>>,
    ) {
        for dg in dg_segments.iter() {
            let y1 = F::get_y(dg, dg.xx.max);
            segments.push(Segment {
                x_segment: XSegment {
                    a: IntPoint::new(dg.xx.min, dg.y0),
                    b: IntPoint::new(dg.xx.max, y1),
                },
                count: dg.count,
            });
        }
    }
}

trait GetY {
    fn get_y(segm: &DgSegment, x: i32) -> i32;
}

struct PosDiagonal;

impl GetY for PosDiagonal {
    #[inline(always)]
    fn get_y(segm: &DgSegment, x: i32) -> i32 {
        x.wrapping_add(segm.pos_b())
    }
}

struct NegDiagonal;
impl GetY for NegDiagonal {
    #[inline(always)]
    fn get_y(segm: &DgSegment, x: i32) -> i32 {
        segm.neg_b().wrapping_sub(x)
    }
}
