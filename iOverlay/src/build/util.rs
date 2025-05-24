use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_key_sort::bin_key::index::BinLayout;
use crate::build::builder::{GraphBuilder, GraphNode};
use crate::segm::winding::WindingCount;

impl<C: WindingCount, N: GraphNode> GraphBuilder<C, N> {

    pub(crate) fn test_contour_for_loops(&mut self, min: i32, contour: &[IntPoint], buffer: &mut Vec<IntPoint>) -> bool {
        if let Some(layout) = Self::contour_bin_layout(min, contour) {
            self.bin_store.init(layout);
            self.bin_store.reserve_bins_space(contour.iter().map(|p|&p.x));

            let count = self.bin_store.prepare_bins();
            buffer.resize(count, IntPoint::default());

            for p in contour.iter() {
                let index = self.bin_store.layout.index(p.x);
                unsafe {
                    let bin = self.bin_store.bins.get_unchecked_mut(index);
                    let item_index = bin.data;
                    bin.data += 1;
                    *buffer.get_unchecked_mut(item_index) = *p;
                }
            }

            for bin in self.bin_store.bins.iter() {
                let start = bin.offset;
                let end = bin.data;
                if end > start + 1 {
                    for i in start..end - 1 {
                        let a = unsafe { buffer.get_unchecked(i) };
                        for j in i + 1..end {
                            let b = unsafe { buffer.get_unchecked(j) };
                            if a == b {
                                return true;
                            }
                        }
                    }
                }
            }
        } else if contour.len() < 64 {
            let n = contour.len();
            if n > 1 {
                for i in 0..n - 1 {
                    let a = unsafe { contour.get_unchecked(i) };
                    for j in i + 1..n {
                        let b = unsafe { contour.get_unchecked(j) };
                        if a == b {
                            return true;
                        }
                    }
                }
            }
        } else {
            buffer.clear();
            buffer.extend_from_slice(contour);
            buffer.sort_unstable();
            for w in buffer.windows(2) {
                if w[0] == w[1] {
                    return true;
                }
            }
        }
        false
    }

    #[inline]
    fn contour_bin_layout(min: i32, contour: &[IntPoint]) -> Option<BinLayout<i32>> {
        let count = contour.len();

        if !(64..=1_000_000).contains(&count) {
            // direct approach work better for small and large data
            return None
        }

        let mut max = i32::MIN;

        for p in contour.iter() {
            max = max.max(p.x);
        }

        BinLayout::new(min..max, count)
    }
}