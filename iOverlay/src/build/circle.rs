// use i_key_sort::bin_key::index::BinLayout;
// use crate::build::builder::{GraphBuilder, GraphNode};
// use crate::core::solver::Solver;
// use crate::geom::end::End;
// use crate::segm::segment::Segment;
// use crate::segm::winding::WindingCount;
// use crate::util::sort::SmartBinSort;
//
// impl<C: WindingCount, N: GraphNode> GraphBuilder<C, N> {
//
//     pub(crate) fn has_loops(&mut self, segments: &[Segment<C>], solver: &Solver) -> bool {
//         if segments.is_empty() {
//             return false;
//         }
//         self.ends_for_seg_b(segments, solver);
//         // segments is sorted by a and ends sorted by point
//         // if a exist b is also exist
//
//         let mut i = 0;
//         let mut j = 0;
//         while i < segments.len() {
//             let mut na = 1;
//             let a = segments[i].x_segment.a;
//             i += 1;
//             while i < segments.len() && a == segments[i].x_segment.a {
//                 i += 1;
//                 na += 1;
//             }
//
//             if na > 2 {
//                 return true;
//             }
//
//             while self.ends[j].point <= a {
//                 let b = self.ends[j].point;
//                 let mut n = if b == a {
//                     na + 1
//                 } else {
//                     1
//                 };
//
//                 j += 1;
//                 while j < self.ends.len() && b == self.ends[j].point {
//                     j += 1;
//                     n += 1;
//                 }
//
//                 if n > 2 {
//                     return true;
//                 }
//             }
//         }
//
//         // finish with ends
//         while j < self.ends.len() {
//             let b = self.ends[j].point;
//             let mut n = 1;
//             j += 1;
//             while j < self.ends.len() && b == self.ends[j].point {
//                 j += 1;
//                 n += 1;
//             }
//
//             if n > 2 {
//                 return true;
//             }
//         }
//
//         false
//     }
//
//     #[inline]
//     fn ends_for_seg_b(&mut self, segments: &[Segment<C>], solver: &Solver) {
//         if let Some(layout) = self.layout(segments) {
//             self.bin_store.init(layout);
//             self.bin_store.reserve_bins_space(segments.iter().map(|s|&s.x_segment.b.x));
//             let count = self.bin_store.prepare_bins();
//             self.ends.resize(count, End::default());
//
//             for (i, s) in segments.iter().enumerate() {
//                 self.bin_store.feed_vec(&mut self.ends, End { index: i, point: s.x_segment.b });
//             }
//
//             for bin in self.bin_store.bins.iter() {
//                 let start = bin.offset;
//                 let end = bin.data;
//                 if start < end {
//                     self.ends[start..end].sort_by(|a, b| a.point.cmp(&b.point));
//                 }
//             }
//         } else {
//             self.ends.clear();
//             let additional = segments.len().saturating_sub(self.ends.capacity());
//             if additional > 0 {
//                 self.ends.reserve(additional);
//             }
//             for (i, s) in segments.iter().enumerate() {
//                 self.ends.push(End { index: i, point: s.x_segment.b });
//             }
//             self.ends.smart_bin_sort_by(solver, |a, b| a.point.cmp(&b.point));
//         }
//     }
//
//     #[inline]
//     fn layout(&self, segments: &[Segment<C>]) -> Option<BinLayout<i32>> {
//         let count = segments.len();
//         if !(64..=1_000_000).contains(&count) {
//             // direct approach work better for small and large data
//             return None
//         }
//
//         let mut min = i32::MAX;
//         let mut max = i32::MIN;
//         for s in segments.iter() {
//             min = min.min(s.x_segment.b.x);
//             max = max.max(s.x_segment.b.x);
//         }
//
//         BinLayout::new(min..max, count)
//     }
// }