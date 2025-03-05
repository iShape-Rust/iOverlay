use crate::geom::line_range::LineRange;
use crate::iso::core::data::IsoData;
use crate::iso::layout::Layout;
use crate::iso::segment::{DgSegment, HzSegment, VrSegment};
use crate::iso::split::column::Column;
use crate::iso::split::diagonal::Diagonal;
use crate::iso::split::fragment::{DgFragment, HzFragment, VrFragment};

pub(crate) struct Table {
    pub(crate) columns: Vec<Column>,
    pub(crate) layout: Layout
}

impl Table {
    pub(super) fn new(layout: Layout, data: &IsoData) -> Self {
        let count = layout.count();

        let mut counter = vec![0; count];
        let vr_fragments = layout.create_vr_fragments(&data.vr_segments, &mut counter);

        counter.iter_mut().for_each(|n| *n = 0);
        let hz_fragments = layout.create_hz_fragments(&data.hz_segments, &mut counter);

        counter.iter_mut().for_each(|n| *n = 0);
        let dg_pos_fragments = layout.create_dg_fragments::<PosSegm>(&data.dg_pos_segments, &mut counter);

        counter.iter_mut().for_each(|n| *n = 0);
        let dg_neg_fragments = layout.create_dg_fragments::<NegSegm>(&data.dg_neg_segments, &mut counter);

        let columns: Vec<_> = vr_fragments
            .into_iter()
            .zip(hz_fragments)
            .zip(dg_pos_fragments)
            .zip(dg_neg_fragments)
            .map(|(((vr, hz), dg_pos), dg_neg)| Column {
                vr_frags: vr,
                hz_frags: hz,
                dg_pos_frags: dg_pos,
                dg_neg_frags: dg_neg,
            })
            .collect();

        Self {
            columns,
            layout
        }
    }
}

impl Layout {

    fn create_vr_fragments(&self, vr_segments: &Vec<VrSegment>, seg_counter: &mut [usize]) -> Vec<Vec<VrFragment>> {
        for s in vr_segments {
            let (lt, rt) = self.index_border(s.x);
            for i in lt..=rt {
                seg_counter[i] += 1;
            }
        }

        let mut fragments: Vec<Vec<VrFragment>> = seg_counter.iter().map(|n|Vec::with_capacity(*n)).collect();
        for (index, s) in vr_segments.iter().enumerate() {
            let (lt, rt) = self.index_border(s.x);
            for i in lt..=rt {
                fragments[i].push(VrFragment {
                    index,
                    x: s.x,
                    yy: s.yy,
                })
            }
        }

        fragments
    }

    fn create_hz_fragments(&self, segments: &Vec<HzSegment>, seg_counter: &mut [usize]) -> Vec<Vec<HzFragment>> {
        for s in segments {
            let i0 = self.right_index(s.xx.min);
            let i1 = self.left_index(s.xx.max);
            for i in i0..=i1 {
                seg_counter[i] += 1;
            }
        }

        let mut fragments: Vec<Vec<HzFragment>> = seg_counter.iter().map(|n|Vec::with_capacity(*n)).collect();

        for (index, s) in segments.iter().enumerate() {
            let i0 = self.right_index(s.xx.min);
            let i1 = self.left_index(s.xx.max);
            for i in i0..=i1 {
                fragments[i].push(HzFragment {
                    index,
                    y: s.y,
                    xx: s.xx,
                });
            }
        }

        fragments
    }

    fn create_dg_fragments<F: YY>(&self, segments: &Vec<DgSegment>, seg_counter: &mut [usize]) -> Vec<Vec<DgFragment>> {
        for s in segments {
            let i0 = self.right_index(s.xx.min);
            let i1 = self.left_index(s.xx.max);
            for i in i0..=i1 {
                seg_counter[i] += 1;
            }
        }

        let mut fragments: Vec<Vec<DgFragment>> = seg_counter.iter().map(|n|Vec::with_capacity(*n)).collect();

        for (index, s) in segments.iter().enumerate() {
            let i0 = self.right_index(s.xx.min);
            let i1 = self.left_index(s.xx.max);

            let mut x0 = s.xx.min;
            for i in i0..i1 {
                let xi = self.position(i + 1);
                let yy = F::yy(s, x0, xi);
                fragments[i].push(DgFragment {
                    index,
                    y0: s.y0,
                    xx: s.xx,
                    yy,
                });
                x0 = xi
            }

            fragments[i1].push(DgFragment {
                index,
                y0: s.y0,
                xx: s.xx,
                yy: F::yy(s, x0, s.xx.max),
            });
        }

        fragments
    }
}

trait YY {
    fn yy(s: &DgSegment, x0: i32, x1: i32) -> LineRange;
}

struct PosSegm;
impl YY for PosSegm {
    #[inline(always)]
    fn yy(s: &DgSegment, x0: i32, x1: i32) -> LineRange {
        let b = s.pos_b();
        let min = b.wrapping_add(x0);
        let max = b.wrapping_add(x1);
        LineRange { min, max }
    }
}

struct NegSegm;
impl YY for NegSegm {
    #[inline(always)]
    fn yy(s: &DgSegment, x0: i32, x1: i32) -> LineRange {
        let b = s.neg_b();
        let min = b.wrapping_sub(x0);
        let max = b.wrapping_sub(x1);
        LineRange { min, max }
    }
}