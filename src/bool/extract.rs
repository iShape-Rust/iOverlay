use std::collections::HashMap;
use i_float::fix_float::FixFloat;
use i_shape::fix_path::{FixPath, FixPathExtension};
use i_shape::fix_bnd::FixBnd;
use i_float::fix_vec::FixVec;
use i_shape::fix_shape::FixShape;
use i_shape::triangle::Triangle;
use crate::fill::segment::SegmentFill;
use crate::index::EMPTY_INDEX;
use crate::layout::overlay_graph::OverlayGraph;

use super::overlay_rule::OverlayRule;
use super::filter::Filter;

struct Contour {
    path: FixPath,
    // Array of points in clockwise order
    boundary: FixBnd,
    // Smallest bounding box of the path
    start: FixVec,
    // Leftmost point in the path
    is_cavity: bool,      // True if path is an internal cavity (hole), false if external (hull)
}

impl OverlayGraph {
    pub fn extract_shapes(&self, fill_rule: OverlayRule) -> Vec<FixShape> {
        self.extract_shapes_min_area(fill_rule, FixFloat::new_i64(0))
    }

    pub fn extract_shapes_min_area(&self, fill_rule: OverlayRule, min_area: FixFloat) -> Vec<FixShape> {
        let mut visited = self.links.filter(fill_rule);

        let mut holes = Vec::new();
        let mut shapes = Vec::new();
        let mut shape_bounds = Vec::new();

        for i in 0..self.links.len() {
            if !visited[i] {
                let contour = self.get_contour(fill_rule, min_area, i, &mut visited);

                if !contour.path.is_empty() {
                    if contour.is_cavity {
                        holes.push(contour);
                    } else {
                        shapes.push(FixShape::new_with_contour_and_holes(contour.path, vec![]));
                        shape_bounds.push(contour.boundary);
                    }
                }
            }
        }

        if holes.is_empty() {
            return shapes;
        }

        if shapes.len() == 1 {
            shapes[0].paths.reserve_exact(holes.len());
            for hole in holes {
                shapes[0].add_hole(hole.path)
            }
        } else {
            let mut shape_candidates = Vec::new();

            // find for each hole its shape
            let mut hole_counter: HashMap<usize, usize> = HashMap::with_capacity(holes.len());
            let mut hole_shape = vec![0; holes.len()];

            for (index, hole) in holes.iter().enumerate() {

                shape_candidates.clear();
                for shape_index in 0..shapes.len() {
                    let shape_bnd = &shape_bounds[shape_index];
                    if shape_bnd.is_inside(hole.boundary) {
                        shape_candidates.push(shape_index);
                    }
                }

                assert!(!shape_candidates.is_empty());

                let mut best_shape_index = EMPTY_INDEX;

                if shape_candidates.len() <= 1 {
                    best_shape_index = shape_candidates[0];
                } else {
                    let mut min_dist = i64::MAX;
                    for &shape_index in shape_candidates.iter() {
                        let dist = Self::get_bottom_vertical_distance(shapes[shape_index].contour(), hole.start);
                        if min_dist > dist {
                            min_dist = dist;
                            best_shape_index = shape_index;
                        }
                    }
                }

                hole_shape[index] = best_shape_index;
                *hole_counter.entry(best_shape_index).or_insert(0) += 1;
            }

            for (shape_index, hole_count) in hole_counter.into_iter() {
                let shape = &mut shapes[shape_index];
                shape.paths.reserve_exact(hole_count);
            }

            for (index, hole) in holes.into_iter().enumerate() {
                let shape_index = hole_shape[index];
                let shape = &mut shapes[shape_index];
                shape.add_hole(hole.path);
            }
        }

        shapes
    }

    fn get_contour(&self, fill_rule: OverlayRule, min_area: FixFloat, index: usize, visited: &mut Vec<bool>) -> Contour {
        let mut path = FixPath::new();
        let mut next = index;

        let mut link = self.links[index];

        let mut a = link.a;
        let mut b = link.b;

        let mut left_link = link;

        let mut new_visited = Vec::new();

        // Find a closed tour
        loop {
            new_visited.push(next);
            path.push(a.point);
            let node = self.nodes[b.index];

            if node.count == 2 {
                next = node.other(next);
            } else {
                let is_fill_top = fill_rule.is_fill_top(link.fill);
                let is_cw = Self::is_clockwise(a.point, b.point, is_fill_top);
                next = self.find_nearest_link_to(a, b, next, is_cw, visited);
                if next == EMPTY_INDEX {
                    break;
                }
            }

            link = self.links[next];
            a = b;
            b = link.other(b);

            // Find leftmost and bottom link
            if left_link.a.point.bit_pack() >= link.a.point.bit_pack() {
                let is_same_point = left_link.a.index == link.a.index;
                let is_same_point_and_turn_clockwise = is_same_point && Triangle::is_clockwise(link.b.point, link.a.point, left_link.b.point);

                if !is_same_point || is_same_point_and_turn_clockwise {
                    left_link = link;
                }
            }

            if next == index {
                break;
            }
        }

        let is_cavity = fill_rule.is_fill_bottom(left_link.fill);

        Self::validate(&mut path, min_area);

        for idx in new_visited {
            visited[idx] = true;
        }

        let boundary = if !path.is_empty() {
            FixBnd::from_points(&path)
        } else {
            FixBnd::ZERO
        };

        Contour { path, boundary, start: left_link.a.point, is_cavity }
    }

    fn is_clockwise(a: FixVec, b: FixVec, is_top_inside: bool) -> bool {
        let is_direct = a.bit_pack() < b.bit_pack();

        Self::xnor(is_direct, is_top_inside)
    }

    fn xnor(a: bool, b: bool) -> bool {
        (a && b) || !(a || b)
    }

    fn validate(path: &mut FixPath, min_area: FixFloat) {
        path.remove_degenerates();

        if path.len() < 3 {
            path.clear();
            return;
        }

        let area = path.area();

        if area.abs() < min_area.value() {
            path.clear();
        } else if area < 0 {
            path.reverse();
        }
    }

    // points of holes can not have any common points with hull
    fn get_bottom_vertical_distance(path: &FixPath, p: FixVec) -> i64 {
        let mut p0 = path[path.len() - 1];
        let mut nearest_y = i64::MIN;

        for pi in path {
            // any bottom and non vertical

            if p0.x != pi.x {
                let ab: (FixVec, FixVec) = if p0.x < pi.x {
                    (p0, *pi)
                } else {
                    (*pi, p0)
                };

                if ab.0.x <= p.x && p.x <= ab.1.x {
                    let y = Self::get_vertical_intersection(ab.0, ab.1, p.x);

                    if p.y.value() > y && y > nearest_y {
                        nearest_y = y;
                    }
                }
            }

            p0 = *pi;
        }

        p.y.value() - nearest_y
    }

    fn get_vertical_intersection(p0: FixVec, p1: FixVec, x: FixFloat) -> i64 {
        let y01 = (p0.y - p1.y).value();
        let x01 = (p0.x - p1.x).value();
        let xx0 = (x - p0.x).value();

        (y01 * xx0) / x01 + p0.y.value()
    }
}


impl OverlayRule {
    fn is_fill_top(&self, fill: SegmentFill) -> bool {
        match self {
            OverlayRule::Subject => fill & SegmentFill::SUBJECT_TOP == SegmentFill::SUBJECT_TOP,
            OverlayRule::Clip => fill & SegmentFill::CLIP_TOP == SegmentFill::CLIP_TOP,
            OverlayRule::Intersect => fill & SegmentFill::BOTH_TOP == SegmentFill::BOTH_TOP,
            OverlayRule::Union => fill & SegmentFill::BOTH_BOTTOM == SegmentFill::NONE,
            OverlayRule::Difference => fill & SegmentFill::BOTH_TOP == SegmentFill::SUBJECT_TOP,
            OverlayRule::Xor => {
                let is_subject = fill & SegmentFill::BOTH_TOP == SegmentFill::SUBJECT_TOP;
                let is_clip = fill & SegmentFill::BOTH_TOP == SegmentFill::CLIP_TOP;
                is_subject || is_clip
            }
        }
    }

    fn is_fill_bottom(&self, fill: SegmentFill) -> bool {
        match self {
            OverlayRule::Subject => fill & SegmentFill::SUBJECT_BOTTOM == SegmentFill::SUBJECT_BOTTOM,
            OverlayRule::Clip => fill & SegmentFill::CLIP_BOTTOM == SegmentFill::CLIP_BOTTOM,
            OverlayRule::Intersect => fill & SegmentFill::BOTH_BOTTOM == SegmentFill::BOTH_BOTTOM,
            OverlayRule::Union => fill & SegmentFill::BOTH_TOP == SegmentFill::NONE,
            OverlayRule::Difference => fill & SegmentFill::BOTH_BOTTOM == SegmentFill::SUBJECT_BOTTOM,
            OverlayRule::Xor => {
                let is_subject = fill & SegmentFill::BOTH_BOTTOM == SegmentFill::SUBJECT_BOTTOM;
                let is_clip = fill & SegmentFill::BOTH_BOTTOM == SegmentFill::CLIP_BOTTOM;
                is_subject || is_clip
            }
        }
    }
}