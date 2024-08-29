use i_float::fix_vec::FixVec;
use i_float::point::IntPoint;

use crate::core::solver::Solver;
use crate::id_point::IdPoint;
use crate::segm::end::End;
use crate::segm::segment::{Segment, SegmentFill};
use crate::sort::SmartSort;

use super::{overlay_link::OverlayLink, overlay_node::OverlayNode};


/// A representation of geometric shapes organized for efficient boolean operations.
///
/// `OverlayGraph` is a core structure designed to facilitate the execution of boolean operations on shapes, such as union, intersection, and difference. It organizes and preprocesses geometric data, making it optimized for these operations. This struct is the result of compiling shape data into a form where boolean operations can be applied directly, efficiently managing the complex relationships between different geometric entities.
///
/// Use `OverlayGraph` to perform boolean operations on the geometric shapes you've added to an `Overlay`, after it has processed the shapes according to the specified fill and overlay rules.
pub struct OverlayGraph {
    pub(crate) solver: Solver,
    pub(crate) nodes: Vec<OverlayNode>,
    pub(crate) links: Vec<OverlayLink>,
}

impl OverlayGraph {

    pub(super) fn new(solver: Solver, bundle: (Vec<Segment>, Vec<SegmentFill>)) -> Self {
        let segments = bundle.0;
        let fills = bundle.1;

        if segments.is_empty() {
            return Self { solver: Default::default(), nodes: vec![], links: vec![] };
        }

        let n = segments.len();
        let mut links: Vec<OverlayLink> = segments
            .into_iter().enumerate()
            .map(|(index, segment)| {
                let fill = *unsafe { fills.get_unchecked(index) };
                OverlayLink::new(
                    IdPoint { id: 0, point: segment.x_segment.a },
                    IdPoint { id: 0, point: segment.x_segment.b },
                    fill,
                )
            }).collect();

        let mut end_bs: Vec<End> = links.iter().enumerate()
            .map(|(i, link)| End { index: i, point: link.b.point })
            .collect();

        end_bs.smart_sort_by(&solver, |a, b| a.point.cmp(&b.point));

        let mut nodes: Vec<OverlayNode> = Vec::with_capacity(n);

        let mut ai = 0;
        let mut bi = 0;
        let mut a = links[0].a.point;
        let mut b = end_bs[0].point;
        let mut next_a_cnt = links.size(a, ai);
        let mut next_b_cnt = end_bs.size(b, bi);
        while next_a_cnt > 0 || next_b_cnt > 0 {
            let (a_cnt, b_cnt) = if a == b {
                (next_a_cnt, next_b_cnt)
            } else if next_a_cnt > 0 && a < b {
                (next_a_cnt, 0)
            } else {
                (0, next_b_cnt)
            };

            let node_id = nodes.len();
            let mut indices = Vec::with_capacity(a_cnt + b_cnt);

            if a_cnt > 0 {
                next_a_cnt = 0;
                for _ in 0..a_cnt {
                    unsafe { links.get_unchecked_mut(ai) }.a.id = node_id;
                    indices.push(ai);
                    ai += 1;
                }
                if ai < n {
                    a = unsafe { links.get_unchecked(ai) }.a.point;
                    next_a_cnt = links.size(a, ai);
                }
            }

            if b_cnt > 0 {
                next_b_cnt = 0;
                for _ in 0..b_cnt {
                    let e = unsafe { end_bs.get_unchecked(bi) };
                    indices.push(e.index);
                    unsafe { links.get_unchecked_mut(e.index) }.b.id = node_id;
                    bi += 1;
                }

                if bi < n {
                    b = unsafe { end_bs.get_unchecked(bi) }.point;
                    next_b_cnt = end_bs.size(b, bi);
                }
            }

            debug_assert!(indices.len() > 1, "indices: {}", indices.len());
            nodes.push(OverlayNode { indices });
        }

        debug_assert!(nodes.len() <= n);

        Self { solver, nodes, links }
    }

    pub(crate) fn find_nearest_link_to(
        &self,
        target: &IdPoint,
        center: &IdPoint,
        ignore: usize,
        in_clockwise: bool,
        visited: &[bool],
    ) -> usize {
        let node = unsafe { self.nodes.get_unchecked(center.id) };

        let mut iter = node.indices.iter();

        let value = if let Some(result) = iter
            .find(|&&val| {
                let is_visited = unsafe { *visited.get_unchecked(val) };
                val != ignore && !is_visited
            }) {
            *result
        } else {
            unreachable!("No one unvisited index is found");
        };

        let mut min_index = value;

        let mut min_vec = unsafe { self.links.get_unchecked(min_index) }.other(center).point.subtract(center.point);
        let v0 = target.point.subtract(center.point); // base vector

        // compare minVec with the rest of the vectors

        for &j in iter {
            let is_visited = unsafe { *visited.get_unchecked(j) };
            if is_visited || ignore == j {
                continue;
            }

            let vj = unsafe { self.links.get_unchecked(j) }.other(center).point.subtract(center.point);

            if v0.is_closer_in_rotation_to(vj, min_vec) == in_clockwise {
                min_vec = vj;
                min_index = j;
            }
        }

        min_index
    }

    #[inline(always)]
    pub(crate) fn is_clockwise(a: IntPoint, b: IntPoint, is_top_inside: bool) -> bool {
        let is_direct = a < b;
        Self::xnor(is_direct, is_top_inside)
    }

    #[inline(always)]
    fn xnor(a: bool, b: bool) -> bool {
        (a && b) || !(a || b)
    }
}

trait Size {
    fn size(&self, point: IntPoint, index: usize) -> usize;
}

impl Size for Vec<OverlayLink> {
    #[inline]
    fn size(&self, point: IntPoint, index: usize) -> usize {
        let mut i = index + 1;
        while i < self.len() && self[i].a.point == point {
            i += 1;
        }

        i - index
    }
}

impl Size for Vec<End> {
    #[inline]
    fn size(&self, point: IntPoint, index: usize) -> usize {
        let mut i = index + 1;
        while i < self.len() && self[i].point == point {
            i += 1;
        }

        i - index
    }
}

trait CloseInRotation {
    fn is_closer_in_rotation_to(&self, a: FixVec, b: FixVec) -> bool;
}

impl CloseInRotation for FixVec {
    // v, a, b vectors are multi-directional
    fn is_closer_in_rotation_to(&self, a: FixVec, b: FixVec) -> bool {
        let cross_a = self.cross_product(a);
        let cross_b = self.cross_product(b);

        if cross_a == 0 || cross_b == 0 {
            // vectors are collinear
            return if cross_a == 0 {
                // a is opposite to self, so based on cross_b
                cross_b > 0
            } else {
                // b is opposite to self, so based on cross_a
                cross_a < 0
            };
        }

        let same_side = (cross_a > 0 && cross_b > 0) || (cross_a < 0 && cross_b < 0);

        if !same_side {
            return cross_a < 0;
        }

        let cross_ab = a.cross_product(b);

        cross_ab < 0
    }
}