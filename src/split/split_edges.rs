use i_float::fix_vec::FixVec;
use i_shape::triangle::Triangle;
use crate::fill::segment::Segment;
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
        let mut remove_indices = Vec::new();

        while need_to_fix {
            need_to_fix = false;

            let mut e_index = list.first();

            while e_index.is_not_nil() {
                let this_ref = list.edge(e_index.index);

                if this_ref.count.is_empty() {
                    e_index = list.remove_and_next(e_index.index);
                    continue;
                }

                let this_range = this_ref.vertical_range();
                let this_stop = this_ref.b.bit_pack();

                scan_list.items_in_range(this_range, this_ref.a.bit_pack(), &mut candidates);

                let mut is_cross = false;
                let mut new_scan_segment: Option<ScanSegment<VersionedIndex>> = None;

                if !candidates.is_empty() {
                    'scan_loop:
                    for item in candidates.iter() {
                        let scan_edge = match list.validate_edge(item.id) {
                            None => {
                                remove_indices.push(item.index);
                                continue;
                            }
                            Some(scan_edge) => {
                                scan_edge
                            }
                        };

                        let cross = match this_ref.cross(&scan_edge) {
                            None => {
                                continue;
                            }
                            Some(cross) => { cross }
                        };

                        let v_index = item.id;

                        is_cross = true;
                        let this_edge = this_ref.clone();

                        match cross.nature {
                            EdgeCrossType::Pure => {
                                // if the two segments intersect at a point that isn't an end point of either segment...

                                let x = cross.point;

                                // divide both segments

                                let this_lt = ShapeEdge::create_and_validate(this_edge.a, x, this_edge.count);
                                let this_rt = ShapeEdge::create_and_validate(x, this_edge.b, this_edge.count);

                                assert!(this_lt.is_less(&this_rt));

                                let scan_lt = ShapeEdge::create_and_validate(scan_edge.a, x, scan_edge.count);
                                let scan_rt = ShapeEdge::create_and_validate(x, scan_edge.b, scan_edge.count);

                                assert!(scan_lt.is_less(&scan_rt));

                                let new_this_left = list.add_and_merge(e_index.index, this_lt);
                                _ = list.add_and_merge(e_index.index, this_rt);

                                let new_scan_left = list.add_and_merge(v_index.index, scan_lt);
                                _ = list.add_and_merge(v_index.index, scan_rt);

                                list.remove(e_index.index);
                                list.remove(v_index.index);

                                // new point must be exactly on the same line
                                let is_bend = this_edge.is_not_same_line(x) || scan_edge.is_not_same_line(x);
                                need_to_fix = need_to_fix || is_bend;

                                e_index = new_this_left;

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan_left,
                                    range: scan_lt.vertical_range(),
                                    stop: scan_lt.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                            EdgeCrossType::EndB => {
                                // scan edge end divide this edge into 2 parts

                                let x = cross.point;

                                // divide this edge

                                let this_lt = ShapeEdge::create_and_validate(this_edge.a, x, this_edge.count);
                                let this_rt = ShapeEdge::create_and_validate(x, this_edge.b, this_edge.count);

                                assert!(this_lt.is_less(&this_rt));

                                _ = list.add_and_merge(e_index.index, this_rt);
                                let new_this_left = list.add_and_merge(e_index.index, this_lt);

                                list.remove(e_index.index);

                                e_index = new_this_left;

                                // new point must be exactly on the same line
                                let is_bend = this_edge.is_not_same_line(x);
                                need_to_fix = need_to_fix || is_bend;

                                break 'scan_loop;
                            }
                            EdgeCrossType::OverlayB => {
                                // split this into 3 segments

                                let this0 = ShapeEdge::new(this_edge.a, scan_edge.a, this_edge.count);
                                let this1 = ShapeEdge::new(scan_edge.a, scan_edge.b, this_edge.count);
                                let this2 = ShapeEdge::new(scan_edge.b, this_edge.b, this_edge.count);

                                assert!(this0.is_less(&this1));
                                assert!(this1.is_less(&this2));

                                _ = list.add_and_merge(e_index.index, this1);
                                _ = list.add_and_merge(e_index.index, this2);
                                let new_this0 = list.add_and_merge(e_index.index, this0);

                                list.remove(e_index.index);

                                // new point must be exactly on the same line
                                let is_bend = this_edge.is_not_same_line(scan_edge.a) || this_edge.is_not_same_line(scan_edge.b);
                                need_to_fix = need_to_fix || is_bend;

                                e_index = new_this0;

                                break 'scan_loop;
                            }
                            EdgeCrossType::EndA => {
                                // this edge end divide scan edge into 2 parts

                                let x = cross.point;

                                // divide scan edge

                                let scan_lt = ShapeEdge::create_and_validate(scan_edge.a, x, scan_edge.count);
                                let scan_rt = ShapeEdge::create_and_validate(x, scan_edge.b, scan_edge.count);

                                assert!(scan_lt.is_less(&scan_rt));

                                let new_scan_left = list.add_and_merge(v_index.index, scan_lt);
                                _ = list.add_and_merge(v_index.index, scan_rt);

                                list.remove(v_index.index);

                                // new point must be exactly on the same line
                                let is_bend = scan_edge.is_not_same_line(x);
                                need_to_fix = need_to_fix || is_bend;

                                // do not update e_index

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan_left,
                                    range: scan_lt.vertical_range(),
                                    stop: scan_lt.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                            EdgeCrossType::OverlayA => {
                                // split scan into 3 segments

                                let scan0 = ShapeEdge::new(scan_edge.a, this_edge.a, scan_edge.count);
                                let scan1 = ShapeEdge::new(this_edge.a, this_edge.b, scan_edge.count);
                                let scan2 = ShapeEdge::new(this_edge.b, scan_edge.b, scan_edge.count);

                                assert!(scan0.is_less(&scan1));
                                assert!(scan1.is_less(&scan2));

                                let new_scan0 = list.add_and_merge(v_index.index, scan0);
                                _ = list.add_and_merge(v_index.index, scan1);
                                _ = list.add_and_merge(v_index.index, scan2);

                                list.remove(v_index.index);

                                let is_bend = scan_edge.is_not_same_line(this_edge.a) || scan_edge.is_not_same_line(this_edge.b);
                                need_to_fix = need_to_fix || is_bend;

                                // do not update e_index

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan0,
                                    range: scan0.vertical_range(),
                                    stop: scan0.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                            EdgeCrossType::Penetrate => {
                                // penetrate each other

                                let x_this = cross.point;
                                let x_scan = cross.second;

                                // divide both segments

                                let this_lt = ShapeEdge::new(this_edge.a, x_this, this_edge.count);
                                let this_rt = ShapeEdge::new(x_this, this_edge.b, this_edge.count);

                                assert!(this_lt.is_less(&this_rt));

                                let scan_lt = ShapeEdge::new(scan_edge.a, x_scan, scan_edge.count);
                                let scan_rt = ShapeEdge::new(x_scan, scan_edge.b, scan_edge.count);

                                assert!(scan_lt.is_less(&scan_rt));

                                let new_scan_left = list.add_and_merge(v_index.index, scan_lt);
                                _ = list.add_and_merge(v_index.index, scan_rt);

                                _ = list.add_and_merge(e_index.index, this_rt);
                                let new_this_left = list.add_and_merge(e_index.index, this_lt);

                                list.remove(e_index.index);
                                list.remove(v_index.index);

                                // new point must be exactly on the same line
                                let is_bend = this_edge.is_not_same_line(x_this) || scan_edge.is_not_same_line(x_scan);
                                need_to_fix = need_to_fix || is_bend;

                                e_index = new_this_left;

                                new_scan_segment = Some(ScanSegment {
                                    id: new_scan_left,
                                    range: scan_lt.vertical_range(),
                                    stop: scan_lt.b.bit_pack(),
                                });

                                break 'scan_loop;
                            }
                        }
                    }

                    candidates.clear();
                    scan_list.remove_indices(&mut remove_indices);
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
    fn is_not_same_line(&self, point: FixVec) -> bool {
        Triangle::is_not_line(self.a, self.b, point)
    }

    fn vertical_range(&self) -> LineRange {
        if self.a.y > self.b.y {
            LineRange { min: self.b.y as i32, max: self.a.y as i32 }
        } else {
            LineRange { min: self.a.y as i32, max: self.b.y as i32 }
        }
    }

    fn create_and_validate(a: FixVec, b: FixVec, count: ShapeCount) -> Self {
        if a.bit_pack() <= b.bit_pack() {
            Self { a, b, count }
        } else {
            Self { a: b, b: a, count: count.invert() }
        }
    }
}
