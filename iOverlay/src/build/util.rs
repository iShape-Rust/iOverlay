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
        let n = contour.len();
        if n < 64 {
            for (i, a) in contour[..n.saturating_sub(1)].iter().enumerate() {
                if contour[i + 1..].contains(a) {
                    return true;
                }
            }
            return false;
        }

        buffer.clear();
        buffer.extend_from_slice(contour);
        buffer.sort_by_two_keys(false, |p| p.x, |p| p.y);

        buffer.windows(2).any(|w| w[0] == w[1])
    }
}
