use alloc::vec;
use crate::vector::edge::{VectorEdge, VectorPath, VectorShape};
use alloc::vec::Vec;
use i_float::int::point::IntPoint;

/// Simplifies vector contours by removing collinear points when possible.
pub(super) trait VectorSimplify {
    /// Removes redundant points/edges when they lie on a straight line.
    ///
    /// Returns `true` when the contour was modified, `false` otherwise.
    fn simplify_contour(&mut self) -> bool;
}

pub(super) trait VectorSimpleContour {
    fn is_simple(&self) -> bool;
    fn simplified(&self) -> Option<VectorPath>;
}

pub(super) trait VectorSimpleShape {
    fn is_simple(&self) -> bool;
    fn simplified(&self) -> Option<VectorShape>;
}


impl VectorSimplify for VectorPath {
    #[inline]
    fn simplify_contour(&mut self) -> bool {
        if self.is_simple() {
            return false;
        }
        if let Some(simple) = self.simplified() {
            *self = simple;
        } else {
            self.clear();
        }
        true
    }
}

impl VectorSimplify for VectorShape {
    #[inline]
    fn simplify_contour(&mut self) -> bool {
        let mut any_simplified = false;
        let mut any_empty = false;

        for (index, contour) in self.iter_mut().enumerate() {
            if contour.is_simple() {
                continue;
            }
            any_simplified = true;

            if let Some(simple_contour) = contour.simplified() {
                *contour = simple_contour;
            } else if index == 0 {
                self.clear();
                return true;
            } else {
                contour.clear();
                any_empty = true;
            }
        }

        if any_empty {
            self.retain(|contour| !contour.is_empty());
        }

        any_simplified
    }
}

impl VectorSimplify for Vec<VectorShape> {
    #[inline]
    fn simplify_contour(&mut self) -> bool {
        let mut any_simplified = false;
        let mut any_empty = false;

        for shape in self.iter_mut() {
            if shape.is_simple() {
                continue;
            }
            any_simplified = true;
            if let Some(simple_shape) = shape.simplified() {
                *shape = simple_shape;
            } else {
                shape.clear();
                any_empty = true;
            }
        }

        if any_empty {
            self.retain(|shape| !shape.is_empty());
        }

        any_simplified
    }
}

impl VectorSimpleContour for [VectorEdge] {
    #[inline]
    fn is_simple(&self) -> bool {
        let count = self.len();
        if count < 3 {
            return false;
        }

        let mut prev = direction(&self[count - 1]);
        for edge in self.iter() {
            let curr = direction(edge);
            if curr.cross_product(prev) == 0 {
                return false;
            }
            prev = curr;
        }

        true
    }

    fn simplified(&self) -> Option<VectorPath> {
        if self.len() < 3 {
            return None;
        }

        let mut n = self.len();
        let mut nodes: Vec<Node> = vec![Node { next: 0, index: 0, prev: 0 }; n];
        let mut validated: Vec<bool> = vec![false; n];

        let mut i0 = n - 2;
        let mut i1 = n - 1;
        for i2 in 0..n {
            nodes[i1] = Node { next: i2, index: i1, prev: i0 };
            i0 = i1;
            i1 = i2;
        }

        let mut first: usize = 0;
        let mut node = nodes[first];
        let mut i = 0;
        while i < n {
            if validated[node.index] {
                node = nodes[node.next];
                continue;
            }

            let p0 = self[node.prev].b;
            let p1 = self[node.index].b;
            let p2 = self[node.next].b;

            if p1.subtract(p0).cross_product(p2.subtract(p1)) == 0 {
                n -= 1;
                if n < 3 {
                    return None;
                }

                // remove node
                nodes[node.prev].next = node.next;
                nodes[node.next].prev = node.prev;

                if node.index == first {
                    first = node.next
                }

                node = nodes[node.prev];

                if validated[node.prev] {
                    i -= 1;
                    validated[node.prev] = false
                }

                if validated[node.next] {
                    i -= 1;
                    validated[node.next] = false
                }

                if validated[node.index] {
                    i -= 1;
                    validated[node.index] = false
                }
            } else {
                validated[node.index] = true;
                i += 1;
                node = nodes[node.next];
            }
        }

        let mut buffer = vec![VectorEdge::new(0, IntPoint::ZERO, IntPoint::ZERO); n];
        node = nodes[first];

        let mut e0 = &self[node.index];
        for item in buffer.iter_mut().take(n) {
            node = nodes[node.next];
            let e1 = &self[node.index];
            item.a = e0.b;
            item.b = e1.b;
            item.fill = e1.fill;

            e0 = e1;
        }

        Some(buffer)
    }
}

#[derive(Clone, Copy)]
struct Node {
    next: usize,
    index: usize,
    prev: usize,
}

impl VectorSimpleShape for [VectorPath] {
    #[inline]
    fn is_simple(&self) -> bool {
        self.iter().all(|contour| contour.is_simple())
    }

    fn simplified(&self) -> Option<VectorShape> {
        let mut contours = Vec::with_capacity(self.len());
        for (i, contour) in self.iter().enumerate() {
            if contour.is_simple() {
                contours.push(contour.clone());
            } else if let Some(simple) = contour.simplified() {
                contours.push(simple);
            } else if i == 0 {
                return None;
            }
        }

        Some(contours)
    }
}

#[inline]
fn direction(edge: &VectorEdge) -> IntPoint {
    edge.b - edge.a
}

#[cfg(test)]
mod tests {
    use crate::vector::simplify::{IntPoint, VectorSimplify};
    use alloc::vec;
    use i_float::int_pnt;
    use crate::vector::edge::VectorEdge;

    #[test]
    fn test_0() {
        #[rustfmt::skip]
        let mut contour = vec![
            VectorEdge::new(1, int_pnt!(0, -1), int_pnt!(0, -3)),
            VectorEdge::new(2, int_pnt!(0, -3), int_pnt!(1, -3)),
            VectorEdge::new(3, int_pnt!(1, -3), int_pnt!(3, -3)),
            VectorEdge::new(4, int_pnt!(3, -3), int_pnt!(3,  0)),
            VectorEdge::new(5, int_pnt!(3,  0), int_pnt!(0,  0)),
            VectorEdge::new(6, int_pnt!(0,  0), int_pnt!(0, -1)),
        ];

        let result = contour.simplify_contour();

        debug_assert!(result);
        debug_assert!(contour.len() == 4);
    }

    #[test]
    fn test_duplicate_points() {
        #[rustfmt::skip]
        let mut contour = vec![
            VectorEdge::new(1, int_pnt!(-1, 3), int_pnt!(-1, 1)),
            VectorEdge::new(2, int_pnt!(-1, 3), int_pnt!(-1, 1)),
            VectorEdge::new(3, int_pnt!(-1, 1), int_pnt!(-3, 1)),
            VectorEdge::new(4, int_pnt!(-3, 1), int_pnt!(-3, -2)),
            VectorEdge::new(5, int_pnt!(-3, -2), int_pnt!(3, -2)),
            VectorEdge::new(6, int_pnt!(3, -2), int_pnt!(3, 1)),
            VectorEdge::new(7, int_pnt!(3, -2), int_pnt!(1, 1)),
            VectorEdge::new(8, int_pnt!(1, 1), int_pnt!(1, 3)),
            VectorEdge::new(9, int_pnt!(1, 3), int_pnt!(-1, 3)),
            VectorEdge::new(10, int_pnt!(-1, 3), int_pnt!(-1, -1)),
        ];

        let result = contour.simplify_contour();

        debug_assert!(result);
        debug_assert!(contour.len() == 8);
    }

    #[test]
    fn test_tiny_segments() {
        #[rustfmt::skip]
        let mut contour = vec![
            VectorEdge::new(1, int_pnt!(0, 1), int_pnt!(-1, 0)),
            VectorEdge::new(2, int_pnt!(-1, 0), int_pnt!(0, -1)),
            VectorEdge::new(3, int_pnt!(-1, 0), int_pnt!(0, -1)),
            VectorEdge::new(4, int_pnt!(0, -1), int_pnt!(1, 0)),
            VectorEdge::new(5, int_pnt!(1, 0), int_pnt!(0, 1)),
            VectorEdge::new(6, int_pnt!(1, 0), int_pnt!(0, 1)),
        ];

        let result = contour.simplify_contour();

        debug_assert!(result);
        debug_assert!(contour.len() == 4);
    }

    #[test]
    fn test_collinear_runs() {
        #[rustfmt::skip]
        let mut contour = vec![
            VectorEdge::new(1, int_pnt!(-3, 0), int_pnt!(3, 0)),
            VectorEdge::new(2, int_pnt!(-3, 0), int_pnt!(-1, 0)),
            VectorEdge::new(3, int_pnt!(-2, 0), int_pnt!(2, 0)),
            VectorEdge::new(4, int_pnt!(3, 0), int_pnt!(0, -3)),
            VectorEdge::new(5, int_pnt!(0, -3), int_pnt!(-3, 0)),
        ];

        let result = contour.simplify_contour();

        debug_assert!(result);
        debug_assert!(contour.len() == 3);
    }

    #[test]
    fn test_zero_area_path() {
        #[rustfmt::skip]
        let mut contour = vec![
            VectorEdge::new(1, int_pnt!(-3, 0), int_pnt!(3, 0)),
            VectorEdge::new(2, int_pnt!(-3, 0), int_pnt!(-1, 0)),
            VectorEdge::new(3, int_pnt!(-2, 0), int_pnt!(2, 0)),
        ];

        let result = contour.simplify_contour();

        debug_assert!(result);
        debug_assert!(contour.is_empty());
    }
}
