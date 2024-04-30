use i_float::point::IntPoint;
use i_tree::node::EMPTY_REF;
use crate::fill::segment::Segment;
use crate::x_segment::XSegment;
use crate::core::solver::Solver;
use crate::split::shape_edge::ShapeEdge;
use crate::line_range::LineRange;
use crate::split::cross_solver::{CrossResult, ScanCrossSolver};
use crate::split::edge_store::{EdgeStore, StoreIndex};
use crate::split::scan_list::ScanSplitList;
use crate::split::scan_tree::ScanSplitTree;
use crate::split::shape_count::ShapeCount;
use crate::split::scan_store::ScanSplitStore;

pub(crate) trait SplitEdges {
    fn split(&mut self, range: LineRange, solver: Solver) -> Vec<Segment>;
}

impl SplitEdges for Vec<ShapeEdge> {
    fn split(&mut self, range: LineRange, solver: Solver) -> Vec<Segment> {
        let is_small_range = range.width() < 128;
        let is_list: bool;
        #[cfg(debug_assertions)]
        {
            is_list = matches!(solver, Solver::List) || matches!(solver, Solver::Auto) && (self.len() < 1_000 || is_small_range);
        }

        #[cfg(not(debug_assertions))]
        {
            is_list = matches!(solver, Solver::List) || matches!(solver, Solver::Auto) && self.len() < 1_000 || is_small_range;
        }

        let store = EdgeStore::new(&self, 16);

        if is_list {
            let mut solver = SplitSolver { store, scan_store: ScanSplitList::new(self.len()) };
            solver.solve()
        } else {
            let mut solver = SplitSolver { store, scan_store: ScanSplitTree::new(range, self.len()) };
            solver.solve()
        }
    }
}


struct SplitSolver<S> {
    scan_store: S,
    store: EdgeStore,
}

impl<S: ScanSplitStore> SplitSolver<S> {
    fn solve(&mut self) -> Vec<Segment> {
        let mut need_to_fix = true;

        while need_to_fix {
            need_to_fix = false;

            let mut this = self.store.first(0);

            while this.node != EMPTY_REF {
                let this_edge = self.store.edge(this);

                if this_edge.count.is_empty() {
                    this = self.store.remove_and_next(this);
                    continue;
                }

                let scan_result = if let Some(cross) = self.scan_store.intersect_and_remove_other(this_edge.x_segment) {
                    cross
                } else {
                    self.scan_store.insert(this_edge.x_segment);
                    this = self.store.next(this);
                    continue;
                };

                let other = self.store.find(&scan_result.other);

                if other.node == EMPTY_REF {
                    continue;
                }

                match scan_result.cross {
                    CrossResult::PureExact(point) => {
                        this = self.pure_exact(point, &this_edge, other);
                    }
                    CrossResult::PureRound(point) => {
                        this = self.pure_round(point, &this_edge, other);
                        need_to_fix = true
                    }
                    CrossResult::OtherEndExact(point) => {
                        this = self.divide_this_exact(point, &this_edge, this, other);
                    }
                    CrossResult::OtherEndRound(point) => {
                        this = self.divide_this_round(point, &this_edge, this, other);
                        need_to_fix = true;
                    }
                    CrossResult::TargetEndExact(point) => {
                        self.divide_scan_exact(
                            point,
                            &this_edge,
                            other,
                        );
                    }
                    CrossResult::TargetEndRound(point) => {
                        self.divide_scan_round(
                            point,
                            &this_edge,
                            other,
                        );
                        need_to_fix = true;
                    }
                    CrossResult::EndOverlap => {
                        // segments are collinear
                        // 2 situation are possible
                        // this.a inside scan(other)
                        // or
                        // scan.b inside this

                        let scan = &self.store.get(other).x_segment;

                        if this_edge.x_segment.b == scan.b {
                            // this.a inside scan(other)
                            this = self.divide_scan_overlap(&this_edge, other);

                            // scan.a < this.a
                            debug_assert!(scan.a < this_edge.x_segment.a);
                        } else {
                            // scan.b inside this

                            this = self.divide_this_overlap(&this_edge, this, other);

                            // scan.b < this.b
                            debug_assert!(scan.b < this_edge.x_segment.b);
                        }
                    }
                    CrossResult::Overlap => {
                        // segments are collinear
                        // 2 situation are possible
                        // this if fully inside scan(other)
                        // or
                        // partly overlap each other

                        let scan = &self.store.get(other).x_segment;

                        if scan.b < this_edge.x_segment.b {
                            // partly overlap
                            this = self.divide_both_partly_overlap(&this_edge, other)
                        } else {
                            // this inside scan
                            this = self.divide_scan_by_three(&this_edge, this, other)
                        }
                    }
                }
            } // while

            self.scan_store.clear();
        } // while

        self.store.segments()
    }

    fn pure_exact(&mut self, p: IntPoint, this_edge: &ShapeEdge, other: StoreIndex) -> StoreIndex {
        // classic middle intersection, no ends, overlaps etc

        let scan_edge = self.store.get_and_remove(other);
        self.store.remove(this_edge);

        let this_lt = ShapeEdge { x_segment: XSegment { a: this_edge.x_segment.a, b: p }, count: this_edge.count };
        let this_rt = ShapeEdge { x_segment: XSegment { a: p, b: this_edge.x_segment.b }, count: this_edge.count };

        debug_assert!(this_lt.x_segment < this_rt.x_segment);

        let scan_lt = ShapeEdge { x_segment: XSegment { a: scan_edge.x_segment.a, b: p }, count: scan_edge.count };
        let scan_rt = ShapeEdge { x_segment: XSegment { a: p, b: scan_edge.x_segment.b }, count: scan_edge.count };

        debug_assert!(scan_lt.x_segment < scan_rt.x_segment);

        self.store.add_and_merge(scan_lt);
        self.store.add_and_merge(scan_rt);

        self.store.add_and_merge(this_rt);
        let lt_this = self.store.add_and_merge(this_lt);

        debug_assert!(this_lt.x_segment.a.x <= p.x);

        debug_assert!(ScanCrossSolver::is_valid_scan(&scan_lt.x_segment, &this_lt.x_segment));
        self.scan_store.insert(scan_lt.x_segment);

        debug_assert!(!ScanCrossSolver::is_valid_scan(&scan_rt.x_segment, &this_lt.x_segment));

        lt_this
    }

    fn pure_round(&mut self, p: IntPoint, this_edge: &ShapeEdge, other: StoreIndex) -> StoreIndex {
        // classic middle intersection, no ends, overlaps etc

        let scan_edge = self.store.get_and_remove(other);
        self.store.remove(this_edge);

        let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, p, this_edge.count);
        let this_rt = ShapeEdge::create_and_validate(p, this_edge.x_segment.b, this_edge.count);

        debug_assert!(this_lt.x_segment < this_rt.x_segment);

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, p, scan_edge.count);
        let scan_rt = ShapeEdge::create_and_validate(p, scan_edge.x_segment.b, scan_edge.count);

        debug_assert!(scan_lt.x_segment < scan_rt.x_segment);

        self.store.add_and_merge(scan_lt);
        self.store.add_and_merge(scan_rt);

        self.store.add_and_merge(this_rt);
        let lt_this = self.store.add_and_merge(this_lt);

        debug_assert!(this_lt.x_segment.a.x <= p.x);

        if ScanCrossSolver::is_valid_scan(&scan_lt.x_segment, &this_lt.x_segment) {
            self.scan_store.insert(scan_lt.x_segment);
        }

        if ScanCrossSolver::is_valid_scan(&scan_rt.x_segment, &this_lt.x_segment) {
            self.scan_store.insert(scan_rt.x_segment);
        }

        lt_this
    }

    fn divide_this_exact(&mut self, p: IntPoint, this_edge: &ShapeEdge, this: StoreIndex, other: StoreIndex) -> StoreIndex {
        let scan = self.store.get(other).x_segment;
        self.store.remove_index(this);

        let this_lt = ShapeEdge { x_segment: XSegment { a: this_edge.x_segment.a, b: p }, count: this_edge.count };
        let this_rt = ShapeEdge { x_segment: XSegment { a: p, b: this_edge.x_segment.b }, count: this_edge.count };

        debug_assert!(this_lt.x_segment < this_rt.x_segment);

        _ = self.store.add_and_merge(this_rt);
        let lt_this = self.store.add_and_merge(this_lt);

        if ScanCrossSolver::is_valid_scan(&scan, &this_lt.x_segment) {
            self.scan_store.insert(scan);
        }

        lt_this
    }

    fn divide_this_round(&mut self, p: IntPoint, this_edge: &ShapeEdge, this: StoreIndex, other: StoreIndex) -> StoreIndex {
        let scan = self.store.get(other).x_segment;
        self.store.remove_index(this);

        let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, p, this_edge.count);
        let this_rt = ShapeEdge::create_and_validate(p, this_edge.x_segment.b, this_edge.count);

        debug_assert!(this_lt.x_segment < this_rt.x_segment);

        _ = self.store.add_and_merge(this_rt);
        let lt_this = self.store.add_and_merge(this_lt);

        if ScanCrossSolver::is_valid_scan(&scan, &this_lt.x_segment) {
            self.scan_store.insert(scan);
        }

        lt_this
    }

    fn divide_scan_exact(&mut self, p: IntPoint, this_edge: &ShapeEdge, other: StoreIndex) {
        // this segment-end divide scan(other) segment into 2 parts

        let scan_edge = self.store.get_and_remove(other);

        let scan_lt = ShapeEdge { x_segment: XSegment { a: scan_edge.x_segment.a, b: p }, count: scan_edge.count };
        let scan_rt = ShapeEdge { x_segment: XSegment { a: p, b: scan_edge.x_segment.b }, count: scan_edge.count };

        debug_assert!(scan_lt.x_segment < scan_rt.x_segment);

        self.store.add_and_merge(scan_lt);
        self.store.add_and_merge(scan_rt);

        if this_edge.x_segment.a.x < p.x {
            // this < p
            self.scan_store.insert(scan_lt.x_segment);
        } else if scan_rt.x_segment < this_edge.x_segment {
            // scanRt < this
            self.scan_store.insert(scan_rt.x_segment);
        }
    }

    fn divide_scan_round(&mut self, p: IntPoint, this_edge: &ShapeEdge, other: StoreIndex) {
        // this segment-end divide scan(other) segment into 2 parts

        let scan_edge = self.store.get_and_remove(other);

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, p, scan_edge.count);
        let scan_rt = ShapeEdge::create_and_validate(p, scan_edge.x_segment.b, scan_edge.count);

        debug_assert!(scan_lt.x_segment < scan_rt.x_segment);

        self.store.add_and_merge(scan_lt);
        self.store.add_and_merge(scan_rt);

        if this_edge.x_segment.a.x < p.x {
            // this < p
            self.scan_store.insert(scan_lt.x_segment);
        } else if scan_rt.x_segment < this_edge.x_segment {
            // scanRt < this
            self.scan_store.insert(scan_rt.x_segment);
        }
    }

    fn divide_scan_overlap(&mut self, this_edge: &ShapeEdge, other: StoreIndex) -> StoreIndex {
        // segments collinear
        // this.b == scan.b and scan.a < this.a < scan.b

        let scan_edge = self.store.get_and_remove(other);

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);

        _ = self.store.add_and_merge(scan_lt);
        // add scan_edge to this
        let new_this = self.store.add_and_merge(ShapeEdge { x_segment: this_edge.x_segment, count: scan_edge.count });


        if new_this.node == EMPTY_REF {
            self.store.find_equal_or_next(new_this.tree, &this_edge.x_segment)
        } else {
            self.store.next(new_this)
        }
    }

    fn divide_this_overlap(&mut self, this_edge: &ShapeEdge, this: StoreIndex, other: StoreIndex) -> StoreIndex {
        // segments collinear
        // this.a == scan.a and this.a < scan.b < this.b

        let scan_edge = self.store.get(other);

        let merge = this_edge.count.add(scan_edge.count);
        let this_rt = ShapeEdge::create_and_validate(scan_edge.x_segment.b, this_edge.x_segment.b, this_edge.count);

        self.store.update(other, merge);
        self.store.add_and_merge(this_rt);

        self.store.remove_index(this);

        self.scan_store.insert(scan_edge.x_segment);

        let new_other = self.store.find(&scan_edge.x_segment);

        self.store.next(new_other)
    }

    fn divide_both_partly_overlap(&mut self, this_edge: &ShapeEdge, other: StoreIndex) -> StoreIndex {
        // segments collinear
        // scan.a < this.a < scan.b < this.b

        let scan_edge = self.store.get_and_remove(other);
        self.store.remove(this_edge);

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);
        let middle = ShapeEdge::create_and_validate(this_edge.x_segment.a, scan_edge.x_segment.b, scan_edge.count.add(this_edge.count));
        let this_rt = ShapeEdge::create_and_validate(scan_edge.x_segment.b, this_edge.x_segment.b, this_edge.count);

        self.store.add_and_merge(scan_lt);
        self.store.add_and_merge(this_rt);
        let md = self.store.add_and_merge(middle);

        self.scan_store.insert(middle.x_segment);

        self.store.next(md)
    }

    fn divide_scan_by_three(&mut self, this_edge: &ShapeEdge, this: StoreIndex, other: StoreIndex) -> StoreIndex {
        // segments collinear
        // scan.a < this.a < this.b < scan.b

        let scan_edge = self.store.get_and_remove(other);

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);
        let merge = this_edge.count.add(scan_edge.count);
        let scan_rt = ShapeEdge::create_and_validate(this_edge.x_segment.b, scan_edge.x_segment.b, scan_edge.count);

        self.store.update(this, merge);

        self.store.add_and_merge(scan_lt);
        self.store.add_and_merge(scan_rt);

        let new_this = self.store.find(&this_edge.x_segment);

        self.scan_store.insert(this_edge.x_segment);

        self.store.next(new_this)
    }
}


impl ShapeEdge {
    fn create_and_validate(a: IntPoint, b: IntPoint, count: ShapeCount) -> Self {
        if a < b {
            Self { x_segment: XSegment { a, b }, count }
        } else {
            Self { x_segment: XSegment { a: b, b: a }, count: count.invert() }
        }
    }
}