use i_float::point::Point;
use i_float::triangle::Triangle;
use crate::fill::segment::Segment;
use crate::x_order::XOrder;
use crate::x_segment::XSegment;
use crate::layout::solver::Solver;
use crate::split::shape_edge::ShapeEdge;
use crate::line_range::LineRange;
use crate::split::scan_list::ScanSplitList;
use crate::split::scan_tree::ScanSplitTree;
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge_cross::EdgeCrossType;
use crate::split::split_range_list::SplitRangeList;
use crate::split::scan_store::ScanSplitStore;
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

        if is_list {
            let store = ScanSplitList::new(self.len());
            self.solve(store)
        } else {
            let store = ScanSplitTree::new(range, self.len());
            self.solve(store)
        }
    }
}

trait SplitSolver<S: ScanSplitStore> {
    fn solve(&mut self, scan_store: S) -> Vec<Segment>;
}

impl<S: ScanSplitStore> SplitSolver<S> for Vec<ShapeEdge> {
    fn solve(&mut self, scan_store: S) -> Vec<Segment> {
        let mut scan_store = scan_store;
        let mut list = SplitRangeList::new(self);

        let mut need_to_fix = true;

        while need_to_fix {
            need_to_fix = false;

            let mut e_index = list.first();

            while e_index.is_not_nil() {
                let this_edge = list.edge(e_index.index);

                if this_edge.count.is_empty() {
                    e_index = list.remove_and_next(e_index.index);
                    continue;
                }

                let cross_seg = if let Some(cross) = scan_store.intersect(this_edge.x_segment) {
                    cross
                } else {
                    scan_store.insert(VersionSegment { index: e_index, x_segment: this_edge.x_segment });
                    e_index = list.next(e_index.index);
                    continue;
                };

                let v_index = cross_seg.index;

                let scan_edge = if let Some(edge) = list.validate_edge(v_index) {
                    edge
                } else {
                    e_index = list.next(e_index.index);
                    continue;
                };

                let this_edge = this_edge.clone();

                match cross_seg.cross.nature {
                    EdgeCrossType::Pure => {
                        // if the two segments intersect at a point that isn't an end point of either segment...

                        let x = cross_seg.cross.point;

                        // divide both segments

                        let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, x, this_edge.count);
                        let this_rt = ShapeEdge::create_and_validate(x, this_edge.x_segment.b, this_edge.count);

                        debug_assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

                        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, x, scan_edge.count);
                        let scan_rt = ShapeEdge::create_and_validate(x, scan_edge.x_segment.b, scan_edge.count);

                        debug_assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

                        let new_this_left = list.add_and_merge(e_index.index, this_lt);
                        _ = list.add_and_merge(e_index.index, this_rt);

                        let new_scan_left = list.add_and_merge(v_index.index, scan_lt);
                        _ = list.add_and_merge(v_index.index, scan_rt);

                        list.remove(e_index.index);
                        list.remove(v_index.index);

                        // new point must be exactly on the same line
                        let is_bend = this_edge.x_segment.is_not_same_line(x) || scan_edge.x_segment.is_not_same_line(x);
                        need_to_fix = need_to_fix || is_bend;

                        e_index = new_this_left;
                        scan_store.insert(VersionSegment { index: new_scan_left, x_segment: scan_lt.x_segment });
                    }
                    EdgeCrossType::EndB => {
                        // scan edge end divide this edge into 2 parts

                        let x = cross_seg.cross.point;

                        // divide this edge

                        let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, x, this_edge.count);
                        let this_rt = ShapeEdge::create_and_validate(x, this_edge.x_segment.b, this_edge.count);

                        debug_assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

                        _ = list.add_and_merge(e_index.index, this_rt);
                        let new_this_left = list.add_and_merge(e_index.index, this_lt);

                        list.remove(e_index.index);

                        e_index = new_this_left;

                        // new point must be exactly on the same line
                        let is_bend = this_edge.x_segment.is_not_same_line(x);
                        need_to_fix = need_to_fix || is_bend;
                    }
                    EdgeCrossType::OverlayB => {
                        // split this into 3 segments

                        let this0 = ShapeEdge::new(this_edge.x_segment.a, scan_edge.x_segment.a, this_edge.count);
                        let this1 = ShapeEdge::new(scan_edge.x_segment.a, scan_edge.x_segment.b, this_edge.count);
                        let this2 = ShapeEdge::new(scan_edge.x_segment.b, this_edge.x_segment.b, this_edge.count);

                        debug_assert!(this0.x_segment.is_less(&this1.x_segment));
                        debug_assert!(this1.x_segment.is_less(&this2.x_segment));

                        _ = list.add_and_merge(e_index.index, this1);
                        _ = list.add_and_merge(e_index.index, this2);
                        let new_this0 = list.add_and_merge(e_index.index, this0);

                        list.remove(e_index.index);

                        // new point must be exactly on the same line
                        let is_bend = this_edge.x_segment.is_not_same_line(scan_edge.x_segment.a) || this_edge.x_segment.is_not_same_line(scan_edge.x_segment.b);
                        need_to_fix = need_to_fix || is_bend;

                        e_index = new_this0;
                    }
                    EdgeCrossType::EndA => {
                        // this edge end divide scan edge into 2 parts

                        let x = cross_seg.cross.point;

                        // divide scan edge

                        let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, x, scan_edge.count);
                        let scan_rt = ShapeEdge::create_and_validate(x, scan_edge.x_segment.b, scan_edge.count);

                        debug_assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

                        let new_scan_left = list.add_and_merge(v_index.index, scan_lt);
                        _ = list.add_and_merge(v_index.index, scan_rt);

                        list.remove(v_index.index);

                        // new point must be exactly on the same line
                        let is_bend = scan_edge.x_segment.is_not_same_line(x);
                        need_to_fix = need_to_fix || is_bend;

                        // do not update e_index
                        scan_store.insert(VersionSegment { index: new_scan_left, x_segment: scan_lt.x_segment });
                    }
                    EdgeCrossType::OverlayA => {
                        // split scan into 3 segments

                        let scan0 = ShapeEdge::new(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);
                        let scan1 = ShapeEdge::new(this_edge.x_segment.a, this_edge.x_segment.b, scan_edge.count);
                        let scan2 = ShapeEdge::new(this_edge.x_segment.b, scan_edge.x_segment.b, scan_edge.count);

                        debug_assert!(scan0.x_segment.is_less(&scan1.x_segment));
                        debug_assert!(scan1.x_segment.is_less(&scan2.x_segment));

                        let new_scan0 = list.add_and_merge(v_index.index, scan0);
                        _ = list.add_and_merge(v_index.index, scan1);
                        _ = list.add_and_merge(v_index.index, scan2);

                        list.remove(v_index.index);

                        let is_bend = scan_edge.x_segment.is_not_same_line(this_edge.x_segment.a) || scan_edge.x_segment.is_not_same_line(this_edge.x_segment.b);
                        need_to_fix = need_to_fix || is_bend;

                        // do not update e_index
                        scan_store.insert(VersionSegment { index: new_scan0, x_segment: scan0.x_segment });
                    }
                    EdgeCrossType::Penetrate => {
                        // penetrate each other

                        let x_this = cross_seg.cross.point;
                        let x_scan = cross_seg.cross.second;

                        // divide both segments

                        let this_lt = ShapeEdge::new(this_edge.x_segment.a, x_this, this_edge.count);
                        let this_rt = ShapeEdge::new(x_this, this_edge.x_segment.b, this_edge.count);

                        debug_assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

                        let scan_lt = ShapeEdge::new(scan_edge.x_segment.a, x_scan, scan_edge.count);
                        let scan_rt = ShapeEdge::new(x_scan, scan_edge.x_segment.b, scan_edge.count);

                        debug_assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

                        let new_scan_left = list.add_and_merge(v_index.index, scan_lt);
                        _ = list.add_and_merge(v_index.index, scan_rt);

                        _ = list.add_and_merge(e_index.index, this_rt);
                        let new_this_left = list.add_and_merge(e_index.index, this_lt);

                        list.remove(e_index.index);
                        list.remove(v_index.index);

                        // new point must be exactly on the same line
                        let is_bend = this_edge.x_segment.is_not_same_line(x_this) || scan_edge.x_segment.is_not_same_line(x_scan);
                        need_to_fix = need_to_fix || is_bend;

                        e_index = new_this_left;
                        scan_store.insert(VersionSegment { index: new_scan_left, x_segment: scan_lt.x_segment });
                    }
                }
            } // while

            scan_store.clear();
        } // while

        list.segments()
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
