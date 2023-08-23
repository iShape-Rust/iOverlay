use i_shape::fix_edge::EdgeCrossType;

use crate::split::shape_edge::ShapeEdge;
use crate::split::edge_linked_list::EdgeLinkedList;
use crate::split::scan_list::ScanList;
use crate::index::EMPTY_INDEX;

pub(crate) trait SplitEdges {
    fn split(&mut self);
}

impl SplitEdges for Vec<ShapeEdge> {

    fn split(&mut self) {
        // at this moment array is sorted

        let mut list = EdgeLinkedList::new(self);

        let mut scan_list = ScanList::new(4 + self.len() / 4);

        let mut need_to_fix = true;

        while need_to_fix {
            scan_list.clear();
            need_to_fix = false;

            let mut e_index = list.first();

            'main_loop: 
            while e_index != EMPTY_INDEX {
                let e_node = list.node(e_index);
                let this_edge = e_node.edge;

                if this_edge.count.is_even() {
                    
                    list.remove(e_index);
                    scan_list.remove(e_index);
                    e_index = e_node.next;

                    continue
                }

                let scan_pos = this_edge.a_bit_pack;
                let mut s_index = scan_list.first();

                // Iteration over the scan list
                while s_index != EMPTY_INDEX {

                    let scan_edge = list.edge(s_index);

                    if scan_edge.b_bit_pack <= scan_pos || scan_edge.count.is_even() {
                        s_index = scan_list.remove_and_get_next(s_index);
                        continue;
                    }

                    let cross = this_edge.cross(scan_edge);
                    match cross.nature {
                        EdgeCrossType::NotCross => { 
                            // no intersection, go to the next scan edge
                            s_index = scan_list.next(s_index);
                        },
                        EdgeCrossType::Pure => { 
                            // If the two segments intersect at a point that isn't an end point of either segment...
                            
                            let x = cross.point;

                            // devide both segments
                            
                            let this_lt = ShapeEdge::new(this_edge.a, x, this_edge.count);
                            let this_rt = ShapeEdge::new(x, this_edge.b, this_edge.count);
                            
                            let scan_lt = ShapeEdge::new(scan_edge.a, x, scan_edge.count);
                            let scan_rt = ShapeEdge::new(x, scan_edge.b, scan_edge.count);
                            
                            let new_this_left = list.add_and_merge(e_index, this_lt);
                            list.add_and_merge(e_index, this_rt);

                            let new_scan_left = list.add_and_merge(s_index, scan_lt);
                            list.add_and_merge(s_index, scan_rt);

                            list.remove(e_index);
                            list.remove(s_index);

                            scan_list.add(new_scan_left);
                            scan_list.remove(s_index);
                            scan_list.remove(e_index);
                            scan_list.remove_all_less_or_equal(this_lt, &list);

                            // new point must be exactly on the same line
                            let is_bend = this_edge.is_not_same_line(x) || scan_edge.is_not_same_line(x);
                            need_to_fix = need_to_fix || is_bend;
                            
                            e_index = new_this_left;
                            
                            continue 'main_loop
                        },
                        EdgeCrossType::EndB => {
                            // scan edge end devide this edge into 2 parts
                            
                            let x = cross.point;
                            
                            // devide this edge
                            
                            let this_lt = ShapeEdge::new(this_edge.a, x, this_edge.count);
                            let this_rt = ShapeEdge::new(x, this_edge.b, this_edge.count);
                            
                            list.add_and_merge(e_index, this_rt);
                            let new_this_left = list.add_and_merge(e_index, this_lt);

                            list.remove(e_index);
                            
                            scan_list.remove(e_index);
                            scan_list.remove_all_less_or_equal(this_lt, &list);
                            
                            e_index = new_this_left;
                            
                            // new point must be exactly on the same line
                            let is_bend = this_edge.is_not_same_line(x);
                            need_to_fix = need_to_fix || is_bend;
                            
                            continue 'main_loop
                        },
                        EdgeCrossType::OverlayB => {
                            // split this into 3 segments

                            let this0 = ShapeEdge::new(this_edge.a, scan_edge.a, this_edge.count);
                            let this1 = ShapeEdge::new(scan_edge.a, scan_edge.b, this_edge.count);
                            let this2 = ShapeEdge::new(scan_edge.b, this_edge.b, this_edge.count);
                            
                            list.add_and_merge(e_index, this1);
                            list.add_and_merge(e_index, this2);
                            let new_this0 = list.add_and_merge(e_index, this0);
                            
                            list.remove(e_index);
                            
                            scan_list.remove(e_index);
                            scan_list.remove_all_less_or_equal(this0, &list);
                            
                            // new point must be exactly on the same line
                            let is_bend = this_edge.is_not_same_line(scan_edge.a) || this_edge.is_not_same_line(scan_edge.b);
                            need_to_fix = need_to_fix || is_bend;
                            
                            e_index = new_this0;
                            
                            continue 'main_loop
                        },
                        EdgeCrossType::EndA => {
                            // this edge end devide scan edge into 2 parts
                        
                            let x = cross.point;

                            // devide scan edge
                            
                            let scan_lt = ShapeEdge::new(scan_edge.a, x, scan_edge.count);
                            let scan_rt = ShapeEdge::new(x, scan_edge.b, scan_edge.count);
                            
                            let new_scan_left = list.add_and_merge(s_index, scan_lt);
                            list.add_and_merge(s_index, scan_rt);

                            list.remove(s_index);

                            scan_list.add(new_scan_left);
                            scan_list.remove(s_index);
                            scan_list.remove_all_less_or_equal(this_edge, &list);

                            // new point must be exactly on the same line
                            let is_bend = scan_edge.is_not_same_line(x);
                            need_to_fix = need_to_fix || is_bend;
                            
                            // do not update e_index
                            
                            continue 'main_loop
                        },
                        EdgeCrossType::OverlayA => {
                            // split scan into 3 segments
                            
                            let scan0 = ShapeEdge::new(scan_edge.a, this_edge.a, scan_edge.count);
                            let scan1 = ShapeEdge::new(this_edge.a, this_edge.b, scan_edge.count);
                            let scan2 = ShapeEdge::new(this_edge.b, scan_edge.b, scan_edge.count);
                            
                            let new_scan_0 = list.add_and_merge(s_index, scan0);
                            list.add_and_merge(s_index, scan1);
                            list.add_and_merge(s_index, scan2);

                            list.remove(s_index);
                            
                            scan_list.add(new_scan_0);
                            scan_list.remove(s_index);
                            scan_list.remove_all_less_or_equal(this_edge, &list);

                            let is_bend = scan_edge.is_not_same_line(this_edge.a) || scan_edge.is_not_same_line(this_edge.b);
                            need_to_fix = need_to_fix || is_bend;
                            
                            // do not update e_index
                            
                            continue 'main_loop
                        },
                        EdgeCrossType::Penetrate => {
                            // penetrate each other
                            
                            let x_this = cross.point;
                            let x_scan = cross.second;

                            // devide both segments
                            
                            let this_lt = ShapeEdge::new(this_edge.a, x_this, this_edge.count);
                            let this_rt = ShapeEdge::new(x_this, this_edge.b, this_edge.count);
                            
                            let scan_lt = ShapeEdge::new(scan_edge.a, x_scan, this_edge.count);
                            let scan_rt = ShapeEdge::new(x_scan, scan_edge.b, this_edge.count);
                            
                            let new_scan_left = list.add_and_merge(s_index, scan_lt);
                            list.add_and_merge(s_index, scan_rt);
                            
                            list.add_and_merge(e_index, this_rt);
                            let new_this_left = list.add_and_merge(e_index, this_lt);

                            list.remove(e_index);
                            list.remove(s_index);
                            
                            scan_list.add(new_scan_left);
                            scan_list.remove(s_index);
                            scan_list.remove(e_index);
                            scan_list.remove_all_less_or_equal(this_edge, &list);

                            // new point must be exactly on the same line
                            let is_bend = this_edge.is_not_same_line(x_this) || scan_edge.is_not_same_line(x_scan);
                            need_to_fix = need_to_fix || is_bend;
                            
                            e_index = new_this_left;
                            
                            continue 'main_loop
                        }
                    } // match
                } // for scan_list

                scan_list.add(e_index);

                e_index = e_node.next;

            } // main_loop

        } // need_to_fix

        self.splice(.., list.edges());

    }
}
