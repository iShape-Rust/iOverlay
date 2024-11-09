use std::cmp::Ordering;
use i_float::int::point::IntPoint;
use i_float::triangle::Triangle;
use i_tree::node::{Color, EMPTY_REF};
use i_tree::tree::Tree;
use crate::fill::count_segment::CountSegment;
use crate::fill::solver::{FillSolver, FillStrategy};
use crate::geom::end::End;
use crate::geom::x_segment::XSegment;
use crate::segm::segment::{Segment, SegmentFill, NONE};
use crate::segm::shape_count::ShapeCount;
use crate::util::log::Int;

pub(super) struct ScanFillTree<C> {
    tree: Tree<CountSegment<C>>,
}

impl<C: ShapeCount> ScanFillTree<C> {
    #[inline]
    pub(super) fn new(count: usize) -> Self {
        let capacity = count.log2_sqrt();
        let count = C::new(0, 0);
        let x_segment = XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO };
        Self { tree: Tree::new(CountSegment { count, x_segment }, capacity) }
    }

    pub(super) fn insert(&mut self, segment: CountSegment<C>) {
        let stop = segment.x_segment.a.x;
        let mut index = self.tree.root;
        let mut p_index = EMPTY_REF;
        let mut is_left = false;

        while index != EMPTY_REF {
            let node = self.tree.node(index);
            p_index = index;
            if node.value.x_segment.b.x <= stop {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                    p_index = EMPTY_REF;
                }
            } else {
                is_left = segment < node.value;
                if is_left {
                    index = node.left;
                } else {
                    index = node.right;
                }
            }
        }

        let new_index = self.tree.store.get_free_index();
        let new_node = self.tree.mut_node(new_index);
        new_node.left = EMPTY_REF;
        new_node.right = EMPTY_REF;
        new_node.color = Color::Red;
        new_node.value = segment;
        new_node.parent = p_index;

        if p_index == EMPTY_REF {
            self.tree.root = new_index;
        } else {
            if is_left {
                self.tree.mut_node(p_index).left = new_index;
            } else {
                self.tree.mut_node(p_index).right = new_index;
            }

            if self.tree.node(p_index).color == Color::Red {
                self.tree.fix_red_black_properties_after_insert(new_index, p_index)
            }
        }
    }

    fn find_under_and_nearest(&mut self, p: IntPoint) -> C {
        let mut index = self.tree.root;
        let mut result = C::new(0, 0);
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.x_segment.b.x <= p.x {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                }
            } else if node.value.x_segment.is_under_point(p) {
                result = node.value.count;
                index = node.right;
            } else {
                index = node.left;
            }
        }

        result
    }
}


impl FillSolver {
    pub(super) fn tree_fill<F: FillStrategy<C>, C: ShapeCount>(segments: &[Segment<C>]) -> Vec<SegmentFill> {
        // Mark. self is sorted by x_segment.a
        let mut scan_list = ScanFillTree::new(segments.len());
        let mut buf = Vec::with_capacity(4);

        let n = segments.len();
        let mut result = vec![NONE; n];
        let mut i = 0;

        while i < n {
            let p = segments[i].x_segment.a;

            buf.push(End { index: i, point: segments[i].x_segment.b });
            i += 1;

            while i < n && segments[i].x_segment.a == p {
                buf.push(End { index: i, point: segments[i].x_segment.b });
                i += 1;
            }

            buf.sort_by(|s0, s1|
            if Triangle::is_clockwise_point(p, s1.point, s0.point) {
                Ordering::Less
            } else {
                Ordering::Greater
            });

            let mut sum_count = scan_list.find_under_and_nearest(p);
            let mut fill: SegmentFill;

            for se in buf.iter() {
                let sid = unsafe { segments.get_unchecked(se.index) };
                (sum_count, fill) = F::add_and_fill(sid.count, sum_count);
                *unsafe { result.get_unchecked_mut(se.index) } = fill;
                if sid.x_segment.is_not_vertical() {
                    scan_list.insert(CountSegment { count: sum_count, x_segment: sid.x_segment });
                }
            }

            buf.clear();
        }

        result
    }
}