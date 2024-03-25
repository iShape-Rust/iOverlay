use i_float::bit_pack::BitPackVec;
use i_float::point::Point;
use i_float::triangle::Triangle;
use crate::fill::segment::Segment;
use crate::geom::x_order::XOrder;
use crate::geom::x_segment::XSegment;
use crate::split::shape_edge::ShapeEdge;
use crate::space::line_range::LineRange;
use crate::space::scan_space::{ScanSegment, ScanSpace};
use crate::split::shape_count::ShapeCount;
use crate::split::shape_edge_cross::EdgeCrossType;
use crate::split::split_range_list::SplitRangeList;
use crate::split::version_index::VersionedIndex;

pub(crate) trait SplitEdges {
    fn split(&self, range: LineRange) -> Vec<Segment>;
}

impl SplitEdges for Vec<ShapeEdge> {
    fn split(&self, range: LineRange) -> Vec<Segment> {
        // at this moment array is sorted

        let mut list = SplitRangeList::new(self);

        let mut scan_list = ScanSpace::new(range, self.len());

        let mut need_to_fix = true;

        let mut candidates = Vec::new();
        let mut indices_to_remove = Vec::new();

        while need_to_fix {
            need_to_fix = false;

            let mut e_index = list.first();

            while e_index.is_not_nil() {
                let this_ref = list.edge(e_index.index);

                if this_ref.count.is_empty() {
                    e_index = list.remove_and_next(e_index.index);
                    continue;
                }

                let this_range = this_ref.x_segment.y_range();
                let this_stop = this_ref.x_segment.b.bit_pack();

                scan_list.items_in_range(this_range, this_ref.x_segment.a.bit_pack(), &mut candidates);

                let mut is_cross = false;
                let mut new_scan_segment: Option<ScanSegment<VersionedIndex, u64>> = None;

                if !candidates.is_empty() {
                    'scan_loop:
                    for item in candidates.iter() {
                        let scan_edge = if let Some(edge) = list.validate_edge(item.id) {
                            edge
                        } else {
                            indices_to_remove.push(item.index);
                            continue;
                        };

                        let cross = if let Some(cross) = this_ref.x_segment.cross(&scan_edge.x_segment) {
                            cross
                        } else {
                            continue;
                        };

                        let v_index = item.id;

                        is_cross = true;
                        let this_edge = this_ref.clone();

                        match cross.nature {
                            EdgeCrossType::Pure => {
                                // if the two segments intersect at a point that isn't an end point of either segment...

                                let x = cross.point;

                                // divide both segments

                                let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, x, this_edge.count);
                                let this_rt = ShapeEdge::create_and_validate(x, this_edge.x_segment.b, this_edge.count);

                                assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

                                let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, x, scan_edge.count);
                                let scan_rt = ShapeEdge::create_and_validate(x, scan_edge.x_segment.b, scan_edge.count);

                                assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

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

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan_left,
                                    range: scan_lt.x_segment.y_range(),
                                    stop: scan_lt.x_segment.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                            EdgeCrossType::EndB => {
                                // scan edge end divide this edge into 2 parts

                                let x = cross.point;

                                // divide this edge

                                let this_lt = ShapeEdge::create_and_validate(this_edge.x_segment.a, x, this_edge.count);
                                let this_rt = ShapeEdge::create_and_validate(x, this_edge.x_segment.b, this_edge.count);

                                assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

                                _ = list.add_and_merge(e_index.index, this_rt);
                                let new_this_left = list.add_and_merge(e_index.index, this_lt);

                                list.remove(e_index.index);

                                e_index = new_this_left;

                                // new point must be exactly on the same line
                                let is_bend = this_edge.x_segment.is_not_same_line(x);
                                need_to_fix = need_to_fix || is_bend;

                                break 'scan_loop;
                            }
                            EdgeCrossType::OverlayB => {
                                // split this into 3 segments

                                let this0 = ShapeEdge::new(this_edge.x_segment.a, scan_edge.x_segment.a, this_edge.count);
                                let this1 = ShapeEdge::new(scan_edge.x_segment.a, scan_edge.x_segment.b, this_edge.count);
                                let this2 = ShapeEdge::new(scan_edge.x_segment.b, this_edge.x_segment.b, this_edge.count);

                                assert!(this0.x_segment.is_less(&this1.x_segment));
                                assert!(this1.x_segment.is_less(&this2.x_segment));

                                _ = list.add_and_merge(e_index.index, this1);
                                _ = list.add_and_merge(e_index.index, this2);
                                let new_this0 = list.add_and_merge(e_index.index, this0);

                                list.remove(e_index.index);

                                // new point must be exactly on the same line
                                let is_bend = this_edge.x_segment.is_not_same_line(scan_edge.x_segment.a) || this_edge.x_segment.is_not_same_line(scan_edge.x_segment.b);
                                need_to_fix = need_to_fix || is_bend;

                                e_index = new_this0;

                                break 'scan_loop;
                            }
                            EdgeCrossType::EndA => {
                                // this edge end divide scan edge into 2 parts

                                let x = cross.point;

                                // divide scan edge

                                let scan_lt = ShapeEdge::create_and_validate(scan_edge.x_segment.a, x, scan_edge.count);
                                let scan_rt = ShapeEdge::create_and_validate(x, scan_edge.x_segment.b, scan_edge.count);

                                assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

                                let new_scan_left = list.add_and_merge(v_index.index, scan_lt);
                                _ = list.add_and_merge(v_index.index, scan_rt);

                                list.remove(v_index.index);

                                // new point must be exactly on the same line
                                let is_bend = scan_edge.x_segment.is_not_same_line(x);
                                need_to_fix = need_to_fix || is_bend;

                                // do not update e_index

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan_left,
                                    range: scan_lt.x_segment.y_range(),
                                    stop: scan_lt.x_segment.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                            EdgeCrossType::OverlayA => {
                                // split scan into 3 segments

                                let scan0 = ShapeEdge::new(scan_edge.x_segment.a, this_edge.x_segment.a, scan_edge.count);
                                let scan1 = ShapeEdge::new(this_edge.x_segment.a, this_edge.x_segment.b, scan_edge.count);
                                let scan2 = ShapeEdge::new(this_edge.x_segment.b, scan_edge.x_segment.b, scan_edge.count);

                                assert!(scan0.x_segment.is_less(&scan1.x_segment));
                                assert!(scan1.x_segment.is_less(&scan2.x_segment));

                                let new_scan0 = list.add_and_merge(v_index.index, scan0);
                                _ = list.add_and_merge(v_index.index, scan1);
                                _ = list.add_and_merge(v_index.index, scan2);

                                list.remove(v_index.index);

                                let is_bend = scan_edge.x_segment.is_not_same_line(this_edge.x_segment.a) || scan_edge.x_segment.is_not_same_line(this_edge.x_segment.b);
                                need_to_fix = need_to_fix || is_bend;

                                // do not update e_index

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan0,
                                    range: scan0.x_segment.y_range(),
                                    stop: scan0.x_segment.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                            EdgeCrossType::Penetrate => {
                                // penetrate each other

                                let x_this = cross.point;
                                let x_scan = cross.second;

                                // divide both segments

                                let this_lt = ShapeEdge::new(this_edge.x_segment.a, x_this, this_edge.count);
                                let this_rt = ShapeEdge::new(x_this, this_edge.x_segment.b, this_edge.count);

                                assert!(this_lt.x_segment.is_less(&this_rt.x_segment));

                                let scan_lt = ShapeEdge::new(scan_edge.x_segment.a, x_scan, scan_edge.count);
                                let scan_rt = ShapeEdge::new(x_scan, scan_edge.x_segment.b, scan_edge.count);

                                assert!(scan_lt.x_segment.is_less(&scan_rt.x_segment));

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

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan_left,
                                    range: scan_lt.x_segment.y_range(),
                                    stop: scan_lt.x_segment.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                        }
                    }

                    candidates.clear();
                    scan_list.remove_indices(&mut indices_to_remove);
                }

                if is_cross {
                    if let Some(scan_segment) = new_scan_segment {
                        scan_list.insert(scan_segment);
                    }
                } else {
                    scan_list.insert(ScanSegment {
                        id: e_index,
                        range: this_range,
                        stop: this_stop,
                    });
                    e_index = list.next(e_index.index);
                }
            } // while

            scan_list.clear();
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
