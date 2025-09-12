use crate::build::builder::{GraphBuilder, GraphNode};
use crate::segm::winding::WindingCount;
use alloc::vec::Vec;
use i_float::int::point::IntPoint;
use i_key_sort::sort::two_keys::TwoKeysSort;

impl<C: WindingCount, N: GraphNode> GraphBuilder<C, N> {
    pub(crate) fn test_contour_for_loops(
        &mut self,
        contour: &[IntPoint],
        buffer: &mut Vec<IntPoint>,
    ) -> bool {
        if contour.len() < 64 {
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
            buffer.sort_by_two_keys(false, |p| p.x, |p| p.y);
            for w in buffer.windows(2) {
                if w[0] == w[1] {
                    return true;
                }
            }
        }
        false
    }
}
