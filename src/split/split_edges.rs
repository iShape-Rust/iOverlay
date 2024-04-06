use i_float::point::Point;
use i_float::triangle::Triangle;
use crate::fill::segment::Segment;
use crate::x_order::XOrder;
use crate::x_segment::XSegment;
use crate::core::solver::Solver;
use crate::split::shape_edge::ShapeEdge;
use crate::line_range::LineRange;
use crate::split::cross_solver::{CrossResult, ScanCrossSolver};
use crate::split::scan_list::ScanSplitList;
use crate::split::scan_tree::ScanSplitTree;
use crate::split::shape_count::ShapeCount;
use crate::split::split_range_list::SplitRangeList;
use crate::split::scan_store::ScanSplitStore;
use crate::split::version_index::{DualIndex, VersionedIndex};
use crate::split::version_segment::VersionSegment;

pub(crate) trait SplitEdges {
    fn split(&mut self, range: LineRange, solver: Solver) -> Vec<Segment>;
}

impl SplitEdges for Vec<ShapeEdge> {
    fn split(&mut self, range: LineRange, solver: Solver) -> Vec<Segment> {
        let is_small_range = range.max - range.min < 128;
        let is_list: bool;
        #[cfg(debug_assertions)]
        {
            is_list = matches!(solver, Solver::List) || matches!(solver, Solver::Auto) && (self.len() < 1_000 || is_small_range);
        }

        #[cfg(not(debug_assertions))]
        {
            is_list = matches!(solver, Solver::List) || matches!(solver, Solver::Auto) && self.len() < 1_000 || is_small_range;
        }

        let list = SplitRangeList::new(self);

        if is_list {
            let mut solver = SplitSolver { list, scan_store: ScanSplitList::new(self.len()) };
            solver.solve()
        } else {
            let mut solver = SplitSolver { list, scan_store: ScanSplitTree::new(range, self.len()) };
            solver.solve()
        }
    }
}


struct SplitSolver<S> {
    scan_store: S,
    list: SplitRangeList,
}

impl<S: ScanSplitStore> SplitSolver<S> {
    fn solve(&mut self) -> Vec<Segment> {
        let mut need_to_fix = true;

        while need_to_fix {
            need_to_fix = false;

            let mut this = self.list.first();

            while this.is_not_nil() {
                let this_edge = self.list.edge(this.index);

                if this_edge.count.is_empty() {
                    this = self.list.remove_and_next(this.index);
                    continue;
                }

                let cross_result = if let Some(cross) = self.scan_store.intersect_and_remove_other(this_edge.x_segment) {
                    cross
                } else {
                    self.scan_store.insert(VersionSegment { index: this, x_segment: this_edge.x_segment });
                    this = self.list.next(this.index);
                    continue;
                };

                let other = cross_result.index;

                let scan_edge = if let Some(edge) = self.list.validate_edge(other) {
                    edge
                } else {
                    continue;
                };

                let this_edge = this_edge.clone();

                match cross_result.cross {
                    CrossResult::Pure(point) => {
                        this = self.pure(point, &this_edge, this.index, &scan_edge, other.index);
                        need_to_fix = need_to_fix
                            || this_edge.x_segment.is_not_same_line(point)
                            || scan_edge.x_segment.is_not_same_line(point);
                    }
                    CrossResult::OtherEndExact(point) => {
                        this = self.divide_this_exact(point, &this_edge, this.index, &scan_edge, other);
                    }
                    CrossResult::OtherEndRound(point) => {
                        this = self.divide_this_round(point, &this_edge, this.index, &scan_edge, other);
                        need_to_fix = true;
                    }
                    CrossResult::TargetEndExact(point) => {
                        self.divide_scan_exact(
                            point,
                            &this_edge,
                            &scan_edge,
                            other.index,
                        );
                    }
                    CrossResult::TargetEndRound(point) => {
                        self.divide_scan_round(
                            point,
                            &this_edge,
                            &scan_edge,
                            other.index,
                        );
                        need_to_fix = true;
                    }
                    CrossResult::EndOverlap => {
                        // segments are collinear
                        // 2 situation are possible
                        // this.a inside scan(other)
                        // or
                        // scan.b inside this

                        if this_edge.x_segment.b == scan_edge.x_segment.b {
                            // this.a inside scan(other)
                            this = self.divide_scan_overlap(&this_edge, this.index, &scan_edge, other.index);

                            // scan.a < this.a
                            debug_assert!(scan_edge.x_segment.a.order_by_line_compare(this_edge.x_segment.a));
                        } else {
                            // scan.b inside this

                            this = self.divide_this_overlap(&this_edge, this.index, &scan_edge, other.index);

                            // scan.b < this.b
                            debug_assert!(scan_edge.x_segment.b.order_by_line_compare(this_edge.x_segment.b));
                        }
                    }
                    CrossResult::Overlap => {
                        // segments are collinear
                        // 2 situation are possible
                        // this if fully inside scan(other)
                        // or
                        // partly overlap each other

                        if this_edge.x_segment.b.order_by_line_compare(scan_edge.x_segment.b) {
                            // partly overlap
                            this = self.divide_both_partly_overlap(&this_edge, this.index, &scan_edge, other.index)
                        } else {
                            // this inside scan
                            this = self.divide_scan_by_three(&this_edge, this.index, &scan_edge, other.index)
                        }
                    }
                }
            } // while

            self.scan_store.clear();
        } // while

        self.list.segments()
    }

    fn pure(&mut self, p: Point, this_edge: &ShapeEdge, this: DualIndex, scan_edge: &ShapeEdge, other: DualIndex) -> VersionedIndex {
        // classic middle intersection, no ends, overlaps etc

        let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, p, this_edge.count);
        let this_rt = ShapeEdge::create_and_validate(p, this_edge.x_segment.b, this_edge.count);

        debug_assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, p, scan_edge.count);
        let scan_rt = ShapeEdge::create_and_validate(p, scan_edge.x_segment.b, scan_edge.count);

        debug_assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

        let lt_this = self.list.add_and_merge(this, this_lt);
        self.list.add_and_merge(this, this_rt);

        let lt_scan = self.list.add_and_merge(other, scan_lt);
        let rt_scan = self.list.add_and_merge(other, scan_rt);

        self.list.remove(this);
        self.list.remove(other);

        debug_assert!(this_lt.x_segment.a.x <= p.x);

        if ScanCrossSolver::is_valid_scan(&scan_lt.x_segment, &this_lt.x_segment) {
            self.scan_store.insert(VersionSegment { index: lt_scan, x_segment: scan_lt.x_segment });
        }

        if ScanCrossSolver::is_valid_scan(&scan_rt.x_segment, &this_lt.x_segment) {
            self.scan_store.insert(VersionSegment { index: rt_scan, x_segment: scan_rt.x_segment });
        }

        lt_this
    }

    fn divide_this_exact(&mut self, p: Point, this_edge: &ShapeEdge, this: DualIndex, scan_edge: &ShapeEdge, scan_ver: VersionedIndex) -> VersionedIndex {
        let this_lt = ShapeEdge { x_segment: XSegment { a: this_edge.x_segment.a, b: p }, count: this_edge.count };
        let this_rt = ShapeEdge { x_segment: XSegment { a: p, b: this_edge.x_segment.b }, count: this_edge.count };

        debug_assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

        let lt_this = self.list.add_and_merge(this, this_lt);
        _ = self.list.add_and_merge(lt_this.index, this_rt);

        self.list.remove(this);

        if ScanCrossSolver::is_valid_scan(&scan_edge.x_segment, &this_lt.x_segment) {
            self.scan_store.insert(VersionSegment { index: scan_ver, x_segment: scan_edge.x_segment });
        }

        lt_this
    }

    fn divide_this_round(&mut self, p: Point, this_edge: &ShapeEdge, this: DualIndex, scan_edge: &ShapeEdge, scan_ver: VersionedIndex) -> VersionedIndex {
        let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, p, this_edge.count);
        let this_rt = ShapeEdge::create_and_validate(p, this_edge.x_segment.b, this_edge.count);

        debug_assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

        let lt_this = self.list.add_and_merge(this, this_lt);
        _ = self.list.add_and_merge(lt_this.index, this_rt);

        self.list.remove(this);

        if ScanCrossSolver::is_valid_scan(&scan_edge.x_segment, &this_lt.x_segment) {
            self.scan_store.insert(VersionSegment { index: scan_ver, x_segment: scan_edge.x_segment });
        }

        lt_this
    }

    fn divide_scan_exact(&mut self, p: Point, this_edge: &ShapeEdge, scan_edge: &ShapeEdge, other: DualIndex) {
        // this segment-end divide scan(other) segment into 2 parts

        let scan_lt = ShapeEdge { x_segment: XSegment { a: scan_edge.x_segment.a, b: p }, count: scan_edge.count };
        let scan_rt = ShapeEdge { x_segment: XSegment { a: p, b: scan_edge.x_segment.b }, count: scan_edge.count };

        debug_assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

        let new_scan_left = self.list.add_and_merge(other, scan_lt);
        let new_scan_right = self.list.add_and_merge(other, scan_rt);

        self.list.remove(other);

        if this_edge.x_segment.a.x < p.x {
            // this < p
            self.scan_store.insert(VersionSegment { index: new_scan_left, x_segment: scan_lt.x_segment });
        } else if scan_rt.x_segment.is_less(&this_edge.x_segment) {
            // scanRt < this
            self.scan_store.insert(VersionSegment { index: new_scan_right, x_segment: scan_rt.x_segment });
        }
    }

    fn divide_scan_round(&mut self, p: Point, this_edge: &ShapeEdge, scan_edge: &ShapeEdge, other: DualIndex) {
        // this segment-end divide scan(other) segment into 2 parts

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, p, scan_edge.count);
        let scan_rt = ShapeEdge::create_and_validate(p, scan_edge.x_segment.b, scan_edge.count);

        debug_assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

        let new_scan_left = self.list.add_and_merge(other, scan_lt);
        let new_scan_right = self.list.add_and_merge(other, scan_rt);

        self.list.remove(other);

        if this_edge.x_segment.a.x < p.x {
            // this < p
            self.scan_store.insert(VersionSegment { index: new_scan_left, x_segment: scan_lt.x_segment });
        } else if scan_rt.x_segment.is_less(&this_edge.x_segment) {
            // scanRt < this
            self.scan_store.insert(VersionSegment { index: new_scan_right, x_segment: scan_rt.x_segment });
        }
    }

    fn divide_scan_overlap(&mut self, this_edge: &ShapeEdge, this: DualIndex, scan_edge: &ShapeEdge, other: DualIndex) -> VersionedIndex {
        // segments collinear
        // this.b == scan.b and scan.a < this.a < scan.b

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);
        let merge = this_edge.count.add(scan_edge.count);

        self.list.add_and_merge(other, scan_lt);
        self.list.update_count(this, merge);

        self.list.remove(other);

        self.list.next(this)
    }

    fn divide_this_overlap(&mut self, this_edge: &ShapeEdge, this: DualIndex, scan_edge: &ShapeEdge, other: DualIndex) -> VersionedIndex {
        // segments collinear
        // this.a == scan.a and this.a < scan.b < this.b

        let merge = this_edge.count.add(scan_edge.count);
        let this_rt = ShapeEdge::create_and_validate(scan_edge.x_segment.b, this_edge.x_segment.b, this_edge.count);

        let new_version = self.list.update_count(other, merge);
        self.list.add_and_merge(other, this_rt);

        self.list.remove(this);

        let ver_index = VersionedIndex { version: new_version, index: other };
        self.scan_store.insert(VersionSegment { index: ver_index, x_segment: scan_edge.x_segment });

        self.list.next(other)
    }

    fn divide_both_partly_overlap(&mut self, this_edge: &ShapeEdge, this: DualIndex, scan_edge: &ShapeEdge, other: DualIndex) -> VersionedIndex {
        // segments collinear
        // scan.a < this.a < scan.b < this.b

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);
        let middle = ShapeEdge::create_and_validate(this_edge.x_segment.a, scan_edge.x_segment.b, scan_edge.count.add(this_edge.count));
        let this_rt = ShapeEdge::create_and_validate(scan_edge.x_segment.b, this_edge.x_segment.b, this_edge.count);

        let lt = self.list.add_and_merge(other, scan_lt).index;
        let md = self.list.add_and_merge(lt, middle);
        self.list.add_and_merge(md.index, this_rt);

        self.list.remove(this);
        self.list.remove(other);

        let mid_segment = VersionSegment { index: md, x_segment: middle.x_segment };
        self.scan_store.insert(mid_segment);

        self.list.next(md.index)
    }

    fn divide_scan_by_three(&mut self, this_edge: &ShapeEdge, this: DualIndex, scan_edge: &ShapeEdge, other: DualIndex) -> VersionedIndex {
        // segments collinear
        // scan.a < this.a < this.b < scan.b

        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);
        let merge = this_edge.count.add(scan_edge.count);
        let scan_rt = ShapeEdge::create_and_validate(this_edge.x_segment.b, scan_edge.x_segment.b, scan_edge.count);

        let new_version = self.list.update_count(this, merge);

        self.list.add_and_merge(other, scan_lt);
        self.list.add_and_merge(this, scan_rt);

        self.list.remove(other);

        let ver_index = VersionedIndex { version: new_version, index: this };
        let ver_segment = VersionSegment { index: ver_index, x_segment: this_edge.x_segment };
        self.scan_store.insert(ver_segment);

        self.list.next(this)
    }
}


impl ShapeEdge {
    fn create_and_validate(a: Point, b: Point, count: ShapeCount) -> Self {
        if a.order_by_line_compare(b) {
            Self { x_segment: XSegment { a, b }, count }
        } else {
            Self { x_segment: XSegment { a: b, b: a }, count: count.invert() }
        }
    }
}

impl XSegment {
    fn is_not_same_line(&self, point: Point) -> bool {
        Triangle::is_not_line_point(self.a, self.b, point)
    }
}
