use std::cmp::Ordering;
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

trait ClockWiseSort {
    fn sort_around(self) -> Vec<usize>;
}

impl ClockWiseSort for Vec<IdPoint> {
    // all vectors start from point(0, 0)
    // we are going to sort it in clock wise direction, where 12 O'clock will the minimum

    fn sort_around(mut self) -> Vec<usize> {
        self.sort_unstable_by(|a, b| {
            if a.point.x == 0 || b.point.x == 0 {
                if a.point.x == 0 && b.point.x == 0 {
                    0.cmp(&a.point.y)
                } else if a.point.x == 0 {
                    if a.point.y > 0 || b.point.x < 0 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    if b.point.y > 0 || a.point.x < 0 {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            } else if a.point.x > 0 && b.point.x > 0 || a.point.x < 0 && b.point.x < 0 {
                let cross = a.point.cross_product(b.point);
                cross.cmp(&0)
            } else {
                0.cmp(&a.point.x)
            }
        });

        self.into_iter().map(|ip| ip.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use rand::Rng;
    use rand::seq::SliceRandom;
    use crate::core::overlay_graph::ClockWiseSort;
    use crate::id_point::IdPoint;

    #[test]
    fn test_0() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: 1, y: -1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_1() {
        let result = vec![
            IdPoint { id: 1, point: IntPoint { x: 1, y: -1 } },
            IdPoint { id: 0, point: IntPoint { x: 1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_3() {
        let result = vec![
            IdPoint { id: 1, point: IntPoint { x: -1, y: 1 } },
            IdPoint { id: 0, point: IntPoint { x: 1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_4() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 0, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: 0, y: -1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_5() {
        let result = vec![
            IdPoint { id: 1, point: IntPoint { x: 0, y: -1 } },
            IdPoint { id: 0, point: IntPoint { x: 0, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_6() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 0, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: 1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_7() {
        let result = vec![
            IdPoint { id: 1, point: IntPoint { x: 1, y: 1 } },
            IdPoint { id: 0, point: IntPoint { x: 0, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_8() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: 0 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: 0 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_9() {
        let result = vec![
            IdPoint { id: 1, point: IntPoint { x: -1, y: 0 } },
            IdPoint { id: 0, point: IntPoint { x: 1, y: 0 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_10() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: 0 } },
            IdPoint { id: 1, point: IntPoint { x: 0, y: -1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_11() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: -1, y: -1 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_12() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: -1 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: -1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_13() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: -1 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_14() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_15() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: 0 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_16() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 1, y: -1 } },
            IdPoint { id: 1, point: IntPoint { x: -1, y: -1 } },
            IdPoint { id: 2, point: IntPoint { x: -1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_17() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 0, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: 1, y: 0 } },
            IdPoint { id: 2, point: IntPoint { x: 0, y: -1 } },
            IdPoint { id: 3, point: IntPoint { x: -1, y: 0 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_18() {
        let result = vec![
            IdPoint { id: 3, point: IntPoint { x: -1, y: 0 } },
            IdPoint { id: 2, point: IntPoint { x: 0, y: -1 } },
            IdPoint { id: 1, point: IntPoint { x: 1, y: 0 } },
            IdPoint { id: 0, point: IntPoint { x: 0, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_110() {
        let result = vec![
            IdPoint { id: 0, point: IntPoint { x: 0, y: 1 } },
            IdPoint { id: 1, point: IntPoint { x: 1, y: 1 } },
            IdPoint { id: 2, point: IntPoint { x: 1, y: 0 } },
            IdPoint { id: 3, point: IntPoint { x: 1, y: -1 } },
            IdPoint { id: 4, point: IntPoint { x: 0, y: -1 } },
            IdPoint { id: 5, point: IntPoint { x: -1, y: -1 } },
            IdPoint { id: 6, point: IntPoint { x: -1, y: 0 } },
            IdPoint { id: 7, point: IntPoint { x: -1, y: 1 } },
        ].sort_around();

        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        let mut indices = Vec::with_capacity(32);
        for step in 2..200 {
            for _ in 0..1000 {
                let mut vecs = Vec::with_capacity(32);
                let mut a = 90 - rng.gen_range(0..step);
                let mut id = 0;
                while a > -270 {
                    let sc = (a as f64).to_radians().sin_cos();
                    let x = (1000_000.0 * sc.1) as i32;
                    let y = (1000_000.0 * sc.0) as i32;

                    vecs.push(IdPoint { id, point: IntPoint { x, y } });
                    indices.push(id);
                    a -= rng.gen_range(1..step);
                    id += 1;
                }

                vecs.shuffle(&mut rng);

                let result = vecs.sort_around();

                assert_eq!(result, indices);

                indices.clear();
            }
        }
    }
}