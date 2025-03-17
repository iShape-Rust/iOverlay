use i_float::int::point::IntPoint;
use i_tree::node::{Color, EMPTY_REF};
use i_tree::tree::Tree;
use crate::fill::count_segment::CountSegment;
use crate::fill::solver::ScanFillStore;
use crate::geom::x_segment::XSegment;
use crate::segm::winding_count::WindingCount;
use crate::util::log::Int;

pub(super) struct ScanFillTree<C> {
    tree: Tree<CountSegment<C>>,
}

impl<C: WindingCount> ScanFillTree<C> {
    #[inline]
    pub(super) fn new(count: usize) -> Self {
        let capacity = count.log2_sqrt();
        let count = C::new(0, 0);
        let x_segment = XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO };
        Self { tree: Tree::new(CountSegment { count, x_segment }, capacity) }
    }
}

impl<C: WindingCount> ScanFillStore<C> for ScanFillTree<C> {

    #[inline]
    fn insert(&mut self, segment: CountSegment<C>) {
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

    #[inline]
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