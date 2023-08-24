use i_float::fix_vec::FixVec;
use i_shape::index_point::IndexPoint;

use crate::{fill::segment::{Segment, SegmentFill}, index::EMPTY_INDEX};

use super::{overlay_node::OverlayNode, overlay_link::OverlayLink};

pub struct OverlayGraph {
    pub (crate) nodes: Vec<OverlayNode>,
    indices: Vec<usize>,
    pub (crate) links: Vec<OverlayLink>,
}

impl OverlayGraph {
    pub (super) fn new(segments: Vec<Segment>) -> Self {
        let n = segments.len();
        let mut links = vec![OverlayLink::new(IndexPoint::ZERO, IndexPoint::ZERO, SegmentFill::NONE); n];
        
        let mut v_store = std::collections::HashMap::new();
        v_store.reserve(2 * n);
        
        for i in 0..n {
            let s = &segments[i];
            let ai = v_store.place(s.a);
            let bi = v_store.place(s.b);
            
            links[i] = OverlayLink::new(
                IndexPoint::new(ai, s.a),
                IndexPoint::new(bi, s.b),
                s.fill
            );
        }
        
        let m = v_store.len();
        let mut n_count = vec![0; m];
        for i in 0..n {
            let l = &links[i];
            
            n_count[l.a.index] += 1;
            n_count[l.b.index] += 1;
        }

        let mut nl = 0;
        for i in 0..m {
            let nc = n_count[i];
            if nc > 2 {
                nl += nc;
            }
        }

        let mut indices = vec![0; nl];
        let mut nodes = vec![OverlayNode::new(0, 0, 0); m];
        let mut offset = 0;

        for i in 0..m {
            let nc = n_count[i];
            
            if nc != 2 {
                nodes[i] = OverlayNode::new(offset, 0, nc);
                offset += nc;
            } else {
                nodes[i] = OverlayNode::new(EMPTY_INDEX, EMPTY_INDEX, nc);
            }
        }
        
        for i in 0..n {
            let link = links[i];
            
            let mut node_a = nodes[link.a.index];
            node_a.add(i, &mut indices);
            nodes[link.a.index] = node_a;
            
            let mut node_b = nodes[link.b.index];
            node_b.add(i, &mut indices);
            nodes[link.b.index] = node_b;
        }

        Self {
            nodes,
            indices,
            links,
        }
    }

    pub(crate) fn find_nearest_link_to(
        &self,
        target: IndexPoint,
        center: IndexPoint,
        ignore: usize,
        in_clockwise: bool,
        visited: &[bool],
    ) -> usize {
        let node = self .nodes[center.index];

        // Find any not visited vector
        let mut i = node.data0;
        let last = i + node.count;
        let mut min_index = EMPTY_INDEX;

        while i < last {
            let j = self.indices[i];
            if !visited[j] && ignore != j {
                min_index = j;
                break
            }
            i += 1;
        }

        if min_index == EMPTY_INDEX {
            return EMPTY_INDEX
        }

        let mut min_vec = self.links[min_index].other(center).point - center.point;
        let v0 = target.point - center.point; // base vector

        // Compare min_vec with the rest of the vectors
        i += 1;
        while i < last {
            let j = self.indices[i];
            if !visited[j] && ignore != j {
                let vj = self.links[j].other(center).point - center.point;
                
                if v0.is_closer_in_rotation_to(vj, min_vec) == in_clockwise {
                    min_vec = vj;
                    min_index = j;
                }
            }
            i += 1;
        }

        min_index
    }

}

// Implementing a private trait to use the place method
trait PlaceFixVec {
    fn place(&mut self, point: FixVec) -> usize;
}

impl PlaceFixVec for std::collections::HashMap<FixVec, usize> {
    fn place(&mut self, point: FixVec) -> usize {
        match self.get(&point) {
            Some(&i) => i,
            None => {
                let i = self.len();
                self.insert(point, i);
                i
            }
        }
    }

}

trait CloseInRotation {
    fn is_closer_in_rotation_to(& self, a: FixVec, b: FixVec) -> bool;
}

impl CloseInRotation for FixVec {

    // v, a, b vectors are multidirectional
    fn is_closer_in_rotation_to(& self, a: FixVec, b: FixVec) -> bool {
        let cross_a = self.unsafe_cross_product(a);
        let cross_b = self.unsafe_cross_product(b);

        if cross_a == 0 || cross_b == 0 {
            // vectors are collinear
            if cross_a == 0 {
                // a is opposite to self, so based on cross_b
                return cross_b > 0
            } else {
                // b is opposite to self, so based on cross_a
                return cross_a < 0
            }
        }

        let same_side = (cross_a > 0 && cross_b > 0) || (cross_a < 0 && cross_b < 0);

        if !same_side {
            return cross_a < 0
        }

        let cross_ab = a.unsafe_cross_product(b);

        cross_ab < 0
    }

} 
