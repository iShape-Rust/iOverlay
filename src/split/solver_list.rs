use i_float::point::IntPoint;
use i_tree::node::EMPTY_REF;
use crate::split::cross_solver::{CrossResult, ScanCrossSolver};
use crate::split::shape_edge::ShapeEdge;
use crate::split::store_index::StoreIndex;
use crate::split::store_list::StoreList;
use crate::x_segment::XSegment;

pub(super) struct SplitSolverList {
    pub(super) store: StoreList,
}

impl SplitSolverList {
    pub(super) fn new(store: StoreList) -> Self {
        Self { store }
    }

    pub(super) fn split(&mut self, tree_list_threshold: usize) -> bool {
        let mut need_to_fix = true;

        while need_to_fix {
            need_to_fix = false;

            let mut this = self.store.first(0);

            let mut scan_counter = 0;

            'this_loop:
            while this.node != EMPTY_REF {

                // is it a lot of work and better to use tree solver
                if scan_counter >= tree_list_threshold {
                    return false;
                }

                let this_edge = self.store.edge(this);

                if this_edge.count.is_empty() {
                    this = self.store.remove_and_next(this);
                    continue;
                }

                let mut other = self.store.next(this);

                scan_counter = 0;

                while other.node != EMPTY_REF {
                    scan_counter += 1;

                    let other_edge = self.store.edge(other);
                    if this_edge.x_segment.b <= other_edge.x_segment.a {
                        break;
                    }

                    if ScanCrossSolver::test_y(&this_edge.x_segment, &other_edge.x_segment) {
                        other = self.store.next(other);
                        continue;
                    }

                    // order is important! this x scan
                    if let Some(cross) = ScanCrossSolver::cross(&this_edge.x_segment, &other_edge.x_segment) {
                        match cross {
                            CrossResult::PureExact(point) => {
                                this = self.pure_exact(
                                    point,
                                    this,
                                    other,
                                );
                            }
                            CrossResult::PureRound(point) => {
                                this = self.pure_round(
                                    point,
                                    this,
                                    other,
                                );
                                need_to_fix = true;
                            }
                            CrossResult::OtherEndExact(point) => {
                                this = self.divide_e0_exact(
                                    point,
                                    this_edge.clone(),
                                    this,
                                );
                            }
                            CrossResult::OtherEndRound(point) => {
                                this = self.divide_e0_round(
                                    point,
                                    this_edge.clone(),
                                    this,
                                );
                                need_to_fix = true;
                            }
                            CrossResult::TargetEndExact(point) => {
                                self.divide_e1_exact(
                                    point,
                                    other,
                                );
                            }
                            CrossResult::TargetEndRound(point) => {
                                self.divide_e1_round(
                                    point,
                                    other,
                                );
                                need_to_fix = true;
                            }
                            CrossResult::EndOverlap => {
                                debug_assert!(this_edge.x_segment.a == other_edge.x_segment.a);
                                debug_assert!(this_edge.x_segment.b < other_edge.x_segment.b);

                                this = self.divide_e1_overlap(
                                    this_edge.clone(),
                                    other,
                                );
                            }
                            CrossResult::Overlap => {
                                // segments are collinear
                                // 2 situation are possible
                                // other if fully inside this
                                // or
                                // partly overlap each other

                                if this_edge.x_segment.b < other_edge.x_segment.b {
                                    // partly overlap
                                    this = self.divide_both_partly_overlap(
                                        this_edge.clone(),
                                        other,
                                    )
                                } else {
                                    debug_assert!(other_edge.x_segment.b < this_edge.x_segment.b);
                                    // other inside this
                                    this = self.divide_e0_by_three(
                                        this_edge.clone(),
                                        this,
                                        other_edge.clone(),
                                        other,
                                    )
                                }
                            }
                        }

                        continue 'this_loop;
                    } // cross

                    other = self.store.next(other);
                } // iterate other

                this = self.store.next(this);
            } // iterate this
        } // while need_to_fix

        true
    }

    #[inline]
    fn pure_exact(&mut self, p: IntPoint, i0: StoreIndex, i1: StoreIndex) -> StoreIndex {
        // classic middle intersection, no ends, overlaps etc

        let e1 = self.store.get_and_remove(i1);
        let e0 = self.store.get_and_remove(i0);

        debug_assert!(e0.x_segment < e1.x_segment);

        let e0lt = ShapeEdge { x_segment: XSegment { a: e0.x_segment.a, b: p }, count: e0.count };
        let e0rt = ShapeEdge { x_segment: XSegment { a: p, b: e0.x_segment.b }, count: e0.count };

        debug_assert!(e0lt.x_segment < e0rt.x_segment);

        let e1lt = ShapeEdge { x_segment: XSegment { a: e1.x_segment.a, b: p }, count: e1.count };
        let e1rt = ShapeEdge { x_segment: XSegment { a: p, b: e1.x_segment.b }, count: e1.count };

        debug_assert!(e1lt.x_segment < e1rt.x_segment);

        _ = self.store.add_and_merge(e1lt);
        _ = self.store.add_and_merge(e1rt);

        _ = self.store.add_and_merge(e0rt);
        let next = self.store.add_and_merge(e0lt);

        debug_assert!(e0lt.x_segment.a.x <= p.x);

        next
    }

    #[inline]
    fn pure_round(&mut self, p: IntPoint, i0: StoreIndex, i1: StoreIndex) -> StoreIndex {
        // classic middle intersection, no ends, overlaps etc

        let e1 = self.store.get_and_remove(i1);
        let e0 = self.store.get_and_remove(i0);

        debug_assert!(e0.x_segment < e1.x_segment);

        let e0lt = ShapeEdge::create_and_validate(e0.x_segment.a, p, e0.count);
        let e0rt = ShapeEdge::create_and_validate(p, e0.x_segment.b, e0.count);

        debug_assert!(e0lt.x_segment < e0rt.x_segment);

        let e1lt = ShapeEdge::create_and_validate(e1.x_segment.a, p, e1.count);
        let e1rt = ShapeEdge::create_and_validate(p, e1.x_segment.b, e1.count);

        debug_assert!(e1lt.x_segment < e1rt.x_segment);

        _ = self.store.add_and_merge(e1lt);
        _ = self.store.add_and_merge(e1rt);

        _ = self.store.add_and_merge(e0rt);
        let next = self.store.add_and_merge(e0lt);

        debug_assert!(e0lt.x_segment.a.x <= p.x);

        next
    }

    fn divide_e0_exact(&mut self, p: IntPoint, e0: ShapeEdge, i0: StoreIndex) -> StoreIndex {
        let e0lt = ShapeEdge { x_segment: XSegment { a: e0.x_segment.a, b: p }, count: e0.count };
        let e0rt = ShapeEdge { x_segment: XSegment { a: p, b: e0.x_segment.b }, count: e0.count };

        debug_assert!(e0lt.x_segment < e0rt.x_segment);

        self.store.remove_index(i0);
        _ = self.store.add_and_merge(e0rt);
        let next = self.store.add_and_merge(e0lt);

        next
    }

    fn divide_e0_round(&mut self, p: IntPoint, e0: ShapeEdge, i0: StoreIndex) -> StoreIndex {
        let e0lt = ShapeEdge::create_and_validate(e0.x_segment.a, p, e0.count);
        let e0rt = ShapeEdge::create_and_validate(p, e0.x_segment.b, e0.count);

        debug_assert!(e0lt.x_segment < e0rt.x_segment);

        self.store.remove_index(i0);
        _ = self.store.add_and_merge(e0rt);
        let next = self.store.add_and_merge(e0lt);

        next
    }

    fn divide_e1_exact(&mut self, p: IntPoint, i1: StoreIndex) {
        // this segment-end divide other segment into 2 parts

        let e1 = self.store.get_and_remove(i1);

        let e1lt = ShapeEdge { x_segment: XSegment { a: e1.x_segment.a, b: p }, count: e1.count };
        let e1rt = ShapeEdge { x_segment: XSegment { a: p, b: e1.x_segment.b }, count: e1.count };

        debug_assert!(e1lt.x_segment < e1rt.x_segment);

        _ = self.store.add_and_merge(e1lt);
        _ = self.store.add_and_merge(e1rt);
    }

    fn divide_e1_round(&mut self, p: IntPoint, i1: StoreIndex) {
        // this segment-end divide scan(other) segment into 2 parts

        let e1 = self.store.get_and_remove(i1);

        let e1lt = ShapeEdge::create_and_validate(e1.x_segment.a, p, e1.count);
        let e1rt = ShapeEdge::create_and_validate(p, e1.x_segment.b, e1.count);

        debug_assert!(e1lt.x_segment < e1rt.x_segment);

        _ = self.store.add_and_merge(e1lt);
        _ = self.store.add_and_merge(e1rt);
    }

    fn divide_e1_overlap(&mut self, e0: ShapeEdge, i1: StoreIndex) -> StoreIndex {
        // segments collinear
        // e0.a == e1.a and e0.b < e1.b

        let e1 = self.store.get_and_remove(i1);

        let e1lt = ShapeEdge { x_segment: e0.x_segment, count: e1.count };
        let e1rt = ShapeEdge { x_segment: XSegment { a: e0.x_segment.b, b: e1.x_segment.b }, count: e1.count };

        _ = self.store.add_and_merge(e1rt);
        let next = self.store.add_and_merge(e1lt);

        next // same as i0
    }

    fn divide_both_partly_overlap(&mut self, e0: ShapeEdge, i1: StoreIndex) -> StoreIndex {
        // segments collinear
        // e0.a < e1.a < e0.b < e1.b

        let e1 = self.store.get_and_remove(i1);
        self.store.remove(&e0);

        let e0lt = ShapeEdge { x_segment: XSegment { a: e0.x_segment.a, b: e1.x_segment.a }, count: e0.count };
        let middle = ShapeEdge { x_segment: XSegment { a: e1.x_segment.a, b: e0.x_segment.b }, count: e1.count.add(e0.count) };
        let e1rt = ShapeEdge { x_segment: XSegment { a: e0.x_segment.b, b: e1.x_segment.b }, count: e1.count };

        _ = self.store.add_and_merge(e1rt);
        _ = self.store.add_and_merge(middle);
        let next = self.store.add_and_merge(e0lt);

        next
    }

    fn divide_e0_by_three(&mut self, e0: ShapeEdge, i0: StoreIndex, e1: ShapeEdge, i1: StoreIndex) -> StoreIndex {
        // segments collinear
        // scan.a < this.a < this.b < scan.b

        let e0lt = ShapeEdge { x_segment: XSegment { a: e0.x_segment.a, b: e1.x_segment.a }, count: e0.count };
        let merge = e0.count.add(e1.count);
        let e0rt = ShapeEdge { x_segment: XSegment { a: e1.x_segment.b, b: e0.x_segment.b }, count: e0.count };

        self.store.update(i1, merge);

        // indices will be not valid!

        self.store.remove_index(i0);

        _ = self.store.add_and_merge(e0rt);
        let next = self.store.add_and_merge(e0lt);

        next
    }
}