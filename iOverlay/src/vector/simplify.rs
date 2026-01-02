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
        let mut path = self.to_vec();
        let mut index = 0;

        while index < path.len() {
            if path.len() < 3 {
                return None;
            }

            let prev = if index == 0 { path.len() - 1 } else { index - 1 };

            let a = path[prev].a;
            let b = path[index].a;
            let c = path[index].b;

            if is_collinear(a, b, c) {
                path[prev].b = c;
                path.remove(index);

                if path.len() < 3 {
                    return None;
                }

                if index == 0 {
                    index = path.len() - 1;
                } else {
                    index -= 1;
                }
            } else {
                index += 1;
            }
        }

        Some(path)
    }
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

#[inline]
fn is_collinear(a: IntPoint, b: IntPoint, c: IntPoint) -> bool {
    let ab = b.subtract(a);
    let bc = c.subtract(b);
    ab.cross_product(bc) == 0
}
