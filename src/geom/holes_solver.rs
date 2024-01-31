use i_float::point::Point;
use crate::geom::floor::Floor;
use crate::geom::id_point::IdPoint;
use crate::geom::x_scan_list::XScanList;
use crate::space::line_range::LineRange;
use crate::space::scan_space::ScanSegment;

pub(crate) struct HolesSolution {
    pub(crate) hole_shape: Vec<usize>,
    pub(crate) hole_counter: Vec<usize>,
}

pub(crate) struct HolesSolver;

impl HolesSolver {
    pub(crate) fn solve(shape_count: usize, y_range: LineRange, i_points: Vec<IdPoint>, floors: Vec<Floor>) -> HolesSolution {
        let hole_count = i_points.len();

        let mut scan_list = XScanList::new(y_range, floors.len());

        let mut hole_shape = vec![0; hole_count];
        let mut hole_counter = vec![0; shape_count];

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

                assert!(!best_floor.is_none());
                let shape_index = best_floor.map_or(0, |f| f.id);
                let hole_index = i_points[i].id;

                hole_shape[hole_index] = shape_index;
                hole_counter[shape_index] += 1;

                i += 1
            }
        }


        HolesSolution { hole_shape, hole_counter }
    }
}