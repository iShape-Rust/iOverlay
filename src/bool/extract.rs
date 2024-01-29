use std::cmp::Ordering;
use i_float::bit_pack::BitPackVec;
use i_float::fix_float::{FIX_FRACTION_BITS, FixFloat};
use i_shape::fix_path::{FixPath, FixPathExtension};
use i_float::fix_vec::FixVec;
use i_float::point::Point;
use i_shape::fix_shape::FixShape;
use i_shape::triangle::Triangle;
use crate::bool::floor::{Floor, Floors};
use crate::bool::id_point::IdPoint;
use crate::geom::x_scan_list::XScanList;
use crate::index::EMPTY_INDEX;
use crate::layout::overlay_graph::OverlayGraph;
use crate::space::line_range::LineRange;
use crate::space::scan_space::ScanSegment;

use super::overlay_rule::OverlayRule;
use super::filter::Filter;

impl OverlayGraph {
    pub fn extract_shapes(&self, overlay_rule: OverlayRule) -> Vec<FixShape> {
        self.extract_shapes_min_area(overlay_rule, 0)
    }

    pub fn extract_shapes_min_area(&self, overlay_rule: OverlayRule, min_area: FixFloat) -> Vec<FixShape> {
        let mut visited = self.links.filter(overlay_rule);

        let mut holes = Vec::new();
        let mut shapes = Vec::new();

        let mut j = 0;
        while j < self.nodes.len() {
            let i = self.start(j, &visited);
            if i == EMPTY_INDEX {
                j += 1;
            } else {
                let is_hole = overlay_rule.is_fill_top(self.links[i].fill);
                let path = self.get_path(overlay_rule, is_hole, min_area, i, &mut visited);
                if !path.is_empty() {
                    if is_hole {
                        holes.push(path);
                    } else {
                        shapes.push(FixShape { paths: [path].to_vec() });
                    }
                }
            }
        }

        shapes.join(holes);

        shapes
    }

    fn get_path(&self, overlay_rule: OverlayRule, is_hole: bool, min_area: FixFloat, index: usize, visited: &mut Vec<bool>) -> FixPath {
        let mut path = FixPath::new();
        let mut next = index;

        let mut link = self.links[index];

        let mut a = link.a;
        let mut b = link.b;

        let mut new_visited = Vec::new();

        // Find a closed tour
        loop {
            new_visited.push(next);
            path.push(a.point);
            let node = &self.nodes[b.index];

            if node.indices.len() == 2 {
                next = node.other(next);
            } else {
                let is_fill_top = overlay_rule.is_fill_top(link.fill);
                let is_cw = Self::is_clockwise(a.point, b.point, is_fill_top);
                next = self.find_nearest_link_to(a, b, next, is_cw, visited);
                if next == EMPTY_INDEX {
                    break;
                }
            }

            link = self.links[next];
            a = b;
            b = link.other(b);

            if next == index {
                break;
            }
        }

        Self::validate(&mut path, min_area, is_hole);

        for idx in new_visited {
            visited[idx] = true;
        }

        path
    }

    fn is_clockwise(a: FixVec, b: FixVec, is_top_inside: bool) -> bool {
        let is_direct = a.bit_pack() < b.bit_pack();

        Self::xnor(is_direct, is_top_inside)
    }

    fn xnor(a: bool, b: bool) -> bool {
        (a && b) || !(a || b)
    }

    fn validate(path: &mut FixPath, min_area: FixFloat, is_hole: bool) {
        path.remove_degenerates();

        if path.len() < 3 {
            path.clear();
            return;
        }

        let area = path.area();
        let fix_abs_area = area.abs() >> (FIX_FRACTION_BITS + 1);

        if fix_abs_area < min_area {
            path.clear();
        } else if is_hole && area > 0 || !is_hole && area < 0 {
            // for holes must be negative and for contour must be positive
            path.reverse();
        }
    }

    fn start(&self, node_index: usize, visited: &Vec<bool>) -> usize {
        let node = &self.nodes[node_index];
        let mut j = EMPTY_INDEX;
        for &i in node.indices.iter() {
            if !visited[i] {
                if j == EMPTY_INDEX {
                    j = i;
                } else {
                    let a = self.links[j].a.point;
                    let bj = self.links[j].b.point;
                    let bi = self.links[i].b.point;

                    if Triangle::is_clockwise(a, bi, bj) {
                        j = i;
                    }
                }
            }
        }

        j
    }
}

trait JoinHoles {
    fn join(&mut self, holes: Vec<FixPath>);
    fn scan_join(&mut self, holes: Vec<FixPath>);
}

impl JoinHoles for Vec<FixShape> {
    fn join(&mut self, holes: Vec<FixPath>) {
        if self.is_empty() || holes.is_empty() {
            return;
        }

        if self.len() == 1 {
            self[0].paths.reserve_exact(holes.len());
            let mut hole_paths = holes;
            self[0].paths.append(&mut hole_paths);
        } else {
            self.scan_join(holes);
        }
    }

    fn scan_join(&mut self, holes: Vec<FixPath>) {
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut i_points = Vec::with_capacity(holes.len());
        for i in 0..holes.len() {
            let p = holes[i][0];
            let x = p.x as i32;
            let y = p.y as i32;
            i_points.push(IdPoint::new(i, Point::new(x, y)));
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
        i_points.sort_by(|a, b| a.point.order_by_x(&b.point));

        let x_min = i_points[0].point.x;
        let x_max = i_points[i_points.len() - 1].point.x;

        let mut floors = Vec::new();
        for i in 0..self.len() {
            let mut hole_floors = self[i].contour().floors(i, x_min, x_max, &mut y_min, &mut y_max);
            floors.append(&mut hole_floors);
        }

        floors.sort_by(|a, b| a.seg.a.order_by_x(&b.seg.a));

        let mut scan_list = XScanList::new(LineRange { min: y_min, max: y_max }, floors.len());

        let mut hole_shape = vec![0; holes.len()];
        let mut hole_counter = vec![0; self.len()];

        let mut candidates = Vec::new();

        let mut i = 0;
        let mut j = 0;

        while i < i_points.len() {
            let x = i_points[i].point.x;

            while j < floors.len() && floors[j].seg.a.x <= x {
                let floor = floors[j];
                if floor.seg.b.x > x {
                    scan_list.space.insert(ScanSegment { id: j, range: floor.seg.y_range(), stop: floor.seg.b.x })
                }
                j += 1;
            }

            while i < i_points.len() && i_points[i].point.x == x {
                let p = i_points[i].point;

                // find nearest scan segment for y
                let mut iterator = scan_list.iterator_to_bottom(p.y);
                let mut best_floor: Option<Floor> = None;

                while iterator.min != i32::MIN {
                    scan_list.space.ids_in_range(iterator, x, &mut candidates);
                    if !candidates.is_empty() {
                        for &floor_index in candidates.iter() {
                            let floor = floors[floor_index];
                            if floor.seg.is_under_point(p) {
                                if let Some(best_seg) = best_floor {
                                    if best_seg.seg.is_under_segment(floor.seg) {
                                        best_floor = Some(floor);
                                    }
                                } else {
                                    best_floor = Some(floor);
                                }
                            }
                        }
                        candidates.clear();
                    }

                    if let Some(best_seg) = best_floor {
                        if best_seg.seg.is_above_point(Point::new(x, iterator.min)) {
                            break;
                        }
                    }

                    iterator = scan_list.next(iterator);
                }

                if best_floor.is_none() {
                    print!("None");
                }

                assert!(!best_floor.is_none());
                let shape_index = best_floor.map_or(0, |f| f.id);
                let hole_index = i_points[i].id;

                hole_shape[hole_index] = shape_index;
                hole_counter[shape_index] += 1;

                i += 1
            }
        }

        for shape_index in 0..hole_counter.len() {
            let capacity = hole_counter[shape_index];
            self[shape_index].paths.reserve_exact(capacity);
        }

        let mut hole_index = 0;
        for hole in holes.into_iter() {
            let shape_index = hole_shape[hole_index];
            self[shape_index].paths.push(hole);
            hole_index += 1;
        }
    }
}

trait XOrder {
    fn order_by_x(&self, other: &Self) -> Ordering;
}

impl XOrder for Point {
    fn order_by_x(&self, other: &Self) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
