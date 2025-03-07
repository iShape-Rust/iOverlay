use crate::iso::split::fragment::{DgFragment, HzFragment, VrFragment};

pub(super) struct Column {
    pub(super) vr_frags: Vec<VrFragment>,
    pub(super) hz_frags: Vec<HzFragment>,
    pub(super) dg_pos_frags: Vec<DgFragment>,
    pub(super) dg_neg_frags: Vec<DgFragment>,
}

pub(super) struct SplitPoint {
    pub(super) index: usize,
    pub(super) xy: i32,
}

pub(super) struct SplitResult {
    pub(super) vr_points: Vec<SplitPoint>,
    pub(super) hz_points: Vec<SplitPoint>,
    pub(super) dg_pos_points: Vec<SplitPoint>,
    pub(super) dg_neg_points: Vec<SplitPoint>,
}

impl Column {
    pub(super) fn split(&mut self, width: i32) -> SplitResult {
        let mut result = SplitResult {
            vr_points: vec![],
            hz_points: vec![],
            dg_pos_points: vec![],
            dg_neg_points: vec![],
        };

        self.vr_frags.sort_unstable_by(|f0, f1| f0.x.cmp(&f1.x));
        self.hz_frags
            .sort_unstable_by(|f0, f1| f0.xx.min.cmp(&f1.xx.min));
        self.dg_pos_frags
            .sort_unstable_by(|f0, f1| f0.xx.min.cmp(&f1.xx.min));
        self.dg_neg_frags
            .sort_unstable_by(|f0, f1| f0.xx.min.cmp(&f1.xx.min));

        self.split_vr_segments(width, &mut result);
        self.split_hz_segments(width, &mut result);
        self.split_dgs(width, &mut result);

        result
    }

    fn split_vr_segments(&self, width: i32, result: &mut SplitResult) {
        // vr vz vr
        if self.vr_frags.len() > 1 {
            for (i0, vr0) in self.vr_frags[0..self.vr_frags.len() - 1].iter().enumerate() {
                for vr1 in self.vr_frags[i0 + 1..].iter() {
                    if vr0.yy.max <= vr1.yy.min {
                        break;
                    }
                    if vr0.x != vr1.x {
                        continue;
                    }
                    if vr0.yy.max > vr1.yy.min {
                        result.vr_points.push(SplitPoint {
                            index: vr0.index,
                            xy: vr1.yy.min,
                        });
                    }

                    if vr0.yy.max > vr1.yy.max {
                        result.vr_points.push(SplitPoint {
                            index: vr0.index,
                            xy: vr1.yy.max,
                        });
                    }

                    if vr0.yy.max < vr1.yy.max {
                        result.vr_points.push(SplitPoint {
                            index: vr1.index,
                            xy: vr0.yy.max,
                        })
                    }
                }
            }
        }
        // vr vz hz
        let mut index = 0;

        for vr in self.vr_frags.iter() {
            // scroll to the first y-overlap
            while index < self.hz_frags.len() && self.hz_frags[index].y < vr.yy.min {
                index += 1;
            }

            for hz in self.hz_frags[index..].iter() {
                if hz.y > vr.yy.max {
                    break;
                }
                if vr.x < hz.xx.min || hz.xx.max < vr.x {
                    continue;
                }

                if hz.xx.min < vr.x && vr.x < hz.xx.max {
                    result.hz_points.push(SplitPoint {
                        index: hz.index,
                        xy: vr.x,
                    });
                }

                if vr.yy.min < hz.y && hz.y < vr.yy.max {
                    result.vr_points.push(SplitPoint {
                        index: vr.index,
                        xy: hz.y,
                    });
                }
            }
        }

        // vr vz dg_pos
        Self::split_vr_vz_dg::<PosFrag>(
            &self.vr_frags,
            &self.dg_pos_frags,
            width,
            &mut result.vr_points,
            &mut result.dg_pos_points,
        );

        // vr vz dg_neg
        Self::split_vr_vz_dg::<NegFrag>(
            &self.vr_frags,
            &self.dg_neg_frags,
            width,
            &mut result.vr_points,
            &mut result.dg_neg_points,
        );
    }

    fn split_vr_vz_dg<F: GetXY>(
        vr_frags: &[VrFragment],
        dg_frags: &[DgFragment],
        width: i32,
        vr_points: &mut Vec<SplitPoint>,
        dg_points: &mut Vec<SplitPoint>,
    ) {
        let mut index = 0;

        for vr in vr_frags.iter() {
            let min_y = vr.yy.min.saturating_sub(width);
            while index < dg_frags.len() && dg_frags[index].yy.min < min_y {
                index += 1;
            }

            for dg in dg_frags[index..].iter() {
                if dg.yy.min > vr.yy.max {
                    break;
                }

                if vr.x < dg.xx.min || dg.xx.max < vr.x {
                    continue;
                }

                let y = F::get_y(dg, vr.x);

                if y < vr.yy.min || vr.yy.max < y {
                    continue;
                }

                if vr.yy.min < y && y < vr.yy.max {
                    vr_points.push(SplitPoint {
                        index: vr.index,
                        xy: y,
                    });
                }

                if dg.xx.min < vr.x && vr.x < dg.xx.max {
                    dg_points.push(SplitPoint {
                        index: dg.index,
                        xy: vr.x,
                    });
                }
            }
        }
    }

    #[inline]
    fn split_hz_segments(&self, width: i32, result: &mut SplitResult) {
        // hz vz dg_pos
        Self::split_hz_vz_dg::<PosFrag>(
            &self.hz_frags,
            &self.dg_pos_frags,
            width,
            &mut result.hz_points,
            &mut result.dg_pos_points,
        );
        // hz vz dg_neg
        Self::split_hz_vz_dg::<NegFrag>(
            &self.hz_frags,
            &self.dg_neg_frags,
            width,
            &mut result.hz_points,
            &mut result.dg_neg_points,
        );
    }

    fn split_hz_vz_dg<F: GetXY>(
        hz_frags: &[HzFragment],
        dg_frags: &[DgFragment],
        width: i32,
        vr_points: &mut Vec<SplitPoint>,
        dg_points: &mut Vec<SplitPoint>,
    ) {
        let mut index = 0;

        for hz in hz_frags.iter() {
            let min_y = hz.y.saturating_sub(width);
            while index < dg_frags.len() && dg_frags[index].yy.min < min_y {
                index += 1;
            }

            for dg in dg_frags[index..].iter() {
                if dg.yy.min > hz.y {
                    break;
                }

                if dg.yy.max < hz.y {
                    continue;
                }

                let x = F::get_x(dg, hz.y);

                if x < hz.xx.min || hz.xx.max < x {
                    continue;
                }

                if hz.xx.min < x && x < hz.xx.max {
                    vr_points.push(SplitPoint {
                        index: hz.index,
                        xy: x,
                    });
                }

                if dg.xx.min < x && x < dg.xx.max {
                    dg_points.push(SplitPoint {
                        index: dg.index,
                        xy: x,
                    });
                }
            }
        }
    }

    fn split_dgs(&self, width: i32, result: &mut SplitResult) {
        // pos vz neg
        let mut index = 0;

        for dg_pos in self.dg_pos_frags.iter() {
            let start_y = dg_pos.yy.min.saturating_sub(width);

            while index < self.dg_neg_frags.len() && self.dg_neg_frags[index].yy.max < start_y {
                index += 1;
            }

            for dg_neg in self.dg_neg_frags[index..].iter() {
                if dg_neg.yy.min > dg_pos.yy.max {
                    break;
                }
                if dg_neg.yy.max < dg_pos.yy.min {
                    continue;
                }

                let x = (dg_neg.neg_b() - dg_pos.pos_b()) / 2;

                if x < dg_pos.xx.min || dg_pos.xx.max < x || x < dg_neg.xx.min || dg_neg.xx.max < x
                {
                    continue;
                }

                if dg_pos.xx.min < x && x < dg_pos.xx.max {
                    result.dg_pos_points.push(SplitPoint {
                        index: dg_pos.index,
                        xy: x,
                    });
                }

                if dg_neg.xx.min < x && x < dg_neg.xx.max {
                    result.dg_neg_points.push(SplitPoint {
                        index: dg_neg.index,
                        xy: x,
                    });
                }
            }
        }
    }
}

trait GetXY {
    fn get_x(frag: &DgFragment, y: i32) -> i32;
    fn get_y(frag: &DgFragment, x: i32) -> i32;
}

struct PosFrag;

impl GetXY for PosFrag {
    #[inline(always)]
    fn get_x(frag: &DgFragment, y: i32) -> i32 {
        y.wrapping_sub(frag.pos_b())
    }

    #[inline(always)]
    fn get_y(frag: &DgFragment, x: i32) -> i32 {
        x.wrapping_add(frag.pos_b())
    }
}

struct NegFrag;
impl GetXY for NegFrag {
    #[inline(always)]
    fn get_x(frag: &DgFragment, y: i32) -> i32 {
        frag.neg_b().wrapping_sub(y)
    }

    #[inline(always)]
    fn get_y(frag: &DgFragment, x: i32) -> i32 {
        frag.neg_b().wrapping_sub(x)
    }
}
