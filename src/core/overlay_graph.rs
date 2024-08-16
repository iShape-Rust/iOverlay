use i_float::fix_vec::FixVec;
use i_float::point::IntPoint;
use i_float::triangle::Triangle;

use crate::core::solver::Solver;
use crate::id_point::IdPoint;
use crate::segm::segment::Segment;
use crate::sort::SmartSort;
use crate::util::EMPTY_INDEX;

use super::{overlay_link::OverlayLink, overlay_node::OverlayNode};

struct End {
    seg_index: usize,
    point: IntPoint,
}

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
    #[inline(always)]
    pub fn links(&self) -> &Vec<OverlayLink> {
        // for js version
        &self.links
    }

    pub(super) fn new(solver: Solver, segments: Vec<Segment>) -> Self {
        let n = segments.len();

        if n == 0 {
            return Self { solver: Default::default(), nodes: vec![], links: vec![] };
        }

        let mut end_bs: Vec<End> = Vec::with_capacity(n);
        for (seg_index, segment) in segments.iter().enumerate() {
            end_bs.push(End {
                seg_index,
                point: segment.x_segment.b,
            });
        }

        end_bs.smart_sort_by(&solver, |a, b| a.point.cmp(&b.point));

        let mut nodes: Vec<OverlayNode> = Vec::with_capacity(n);
        let mut links: Vec<OverlayLink> = segments
            .iter()
            .map(|segment| OverlayLink::new(IdPoint::ZERO, IdPoint::ZERO, segment.fill))
            .collect();

        let mut ai = 0;
        let mut bi = 0;
        let mut a = segments[0].x_segment.a;
        let mut b = end_bs[0].point;

        while ai < n || bi < n {
            let mut cnt = 0;
            if a == b {
                cnt += segments.size(a, ai);
                cnt += end_bs.size(b, bi);
            } else if ai < n && a < b {
                cnt += segments.size(a, ai);
            } else {
                cnt += end_bs.size(b, bi);
            }

            let mut indices = Vec::with_capacity(cnt);

            if a == b {
                let ip = IdPoint::new(nodes.len(), a);
                while ai < n {
                    let aa = unsafe { segments.get_unchecked(ai) }.x_segment.a;
                    if aa != a {
                        a = aa;
                        break;
                    }
                    unsafe { links.get_unchecked_mut(ai) }.a = ip;
                    indices.push(ai);

                    ai += 1
                }

                while bi < n {
                    let e = unsafe { end_bs.get_unchecked(bi) };
                    if e.point != b {
                        b = e.point;
                        break;
                    }
                    unsafe { links.get_unchecked_mut(e.seg_index) }.b = ip;
                    indices.push(e.seg_index);

                    bi += 1
                }
            } else if ai < n && a < b {
                let ip = IdPoint::new(nodes.len(), a);
                while ai < n {
                    let aa = unsafe { segments.get_unchecked(ai) }.x_segment.a;
                    if aa != a {
                        a = aa;
                        break;
                    }
                    unsafe { links.get_unchecked_mut(ai) }.a = ip;
                    indices.push(ai);

                    ai += 1
                }
            } else {
                let ip = IdPoint::new(nodes.len(), b);
                while bi < n {
                    let e = unsafe { end_bs.get_unchecked(bi) };
                    if e.point != b {
                        b = e.point;
                        break;
                    }
                    unsafe { links.get_unchecked_mut(e.seg_index) }.b = ip;
                    indices.push(e.seg_index);

                    bi += 1
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
        target: IdPoint,
        center: IdPoint,
        ignore: usize,
        in_clockwise: bool,
        visited: &[bool],
    ) -> usize {
        let node = unsafe {
            self.nodes.get_unchecked(center.id)
        };

        let (index, value) = if let Some(result) = node.indices.iter().enumerate().find(|&(_index, &val)| val != ignore && !visited[val]) {
            (result.0, *result.1)
        } else {
            return EMPTY_INDEX;
        };

        let mut i = index + 1;
        let mut min_index = value;

        let mut min_vec = self.links[min_index].other(center).point.subtract(center.point);
        let v0 = target.point.subtract(center.point); // base vector

        // compare minVec with the rest of the vectors

        while i < node.indices.len() {
            let j = unsafe { *node.indices.get_unchecked(i) };
            let is_not_visited = unsafe { !visited.get_unchecked(j) };
            if is_not_visited && ignore != j {
                let link = unsafe { self.links.get_unchecked(j) };
                let vj = link.other(center).point.subtract(center.point);

                if v0.is_closer_in_rotation_to(vj, min_vec) == in_clockwise {
                    min_vec = vj;
                    min_index = j;
                }
            }
            i += 1
        }

        min_index
    }

    pub(crate) fn find_first_link(&self, node_index: usize, visited: &Vec<bool>) -> usize {
        let node = unsafe { self.nodes.get_unchecked(node_index) };

        let mut j = EMPTY_INDEX;
        for &i in node.indices.iter() {
            let is_not_visited = unsafe { !visited.get_unchecked(i) };
            if is_not_visited {
                if j == EMPTY_INDEX {
                    j = i;
                } else {
                    let (a, bi, bj) = unsafe {
                        let link = self.links.get_unchecked(j);
                        let bi = self.links.get_unchecked(i).b.point;
                        (link.a.point, bi, link.b.point)
                    };
                    if Triangle::is_clockwise_point(a, bi, bj) {
                        j = i;
                    }
                }
            }
        }

        j
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

trait Size {
    fn size(&self, point: IntPoint, index: usize) -> usize;
}

impl Size for Vec<Segment> {
    #[inline]
    fn size(&self, point: IntPoint, index: usize) -> usize {
        let mut i = index;
        while i < self.len() && self[i].x_segment.a == point {
            i += 1;
        }

        i - index
    }
}

impl Size for Vec<End> {
    #[inline]
    fn size(&self, point: IntPoint, index: usize) -> usize {
        let mut i = index;
        while i < self.len() && self[i].point == point {
            i += 1;
        }

        i - index
    }
}