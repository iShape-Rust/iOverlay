use i_float::point::Point;
use crate::array::SwapRemoveIndex;
use crate::x_order::XOrder;
use crate::x_segment::XSegment;
use crate::line_range::LineRange;
use crate::split::scan_store::{CrossSegment, ScanSplitStore};
use crate::split::version_segment::{RemoveVersionSegment, VersionSegment};

#[derive(Debug, Clone)]
struct IntervalNode {
    range: LineRange,
    list: Vec<VersionSegment>,
}

impl IntervalNode {
    fn new(range: LineRange) -> Self {
        Self { range, list: Vec::with_capacity(4) }
    }
}

pub struct ScanSplitTree {
    power: usize,
    nodes: Vec<IntervalNode>,
}

impl ScanSplitTree {
    pub(super) fn new(range: LineRange, count: usize) -> Self {
        let max_power_range = range.log2();
        let max_power_count = (count as i32).log2() >> 1;
        let power = 10.min(max_power_count.min(max_power_range));
        let nodes = Self::create_nodes(range, power);
        Self { power, nodes }
    }

    fn create_nodes(range: LineRange, power: usize) -> Vec<IntervalNode> {
        let n = 1 << power;

        // to make round more precise we use upscale/downscale
        let scale = 4;
        let len = (range.max - range.min) as usize;
        let step = (((len << scale) as f64) / (n as f64)) as i64;

        let capacity = (n << 1) - 1;
        let mut nodes = vec![IntervalNode::new(LineRange { min: 0, max: 0 }); capacity];

        let mut i = 0;
        let mut a0 = range.min;
        let mut s = (range.min as i64) << scale;
        while i < capacity - 1 {
            s += step;
            let a = (s >> scale) as i32;
            nodes[i] = IntervalNode::new(LineRange { min: a0, max: a });
            i += 2;
            a0 = a;
        }
        nodes[i] = IntervalNode::new(LineRange { min: a0, max: range.max });

        for j in 2..power + 1 {
            let t = 1 << j;
            let r = t >> 2;
            let mut i = (t >> 1) - 1;
            while i < capacity {
                let lt = i - r;
                let rt = i + r;
                let lt_min = nodes[lt].range.min;
                let rt_max = nodes[rt].range.max;
                nodes[i] = IntervalNode::new(LineRange { min: lt_min, max: rt_max });
                i += t
            }
        }

        // middle
        nodes[(1 << power) - 1] = IntervalNode::new(range);

        nodes
    }

    fn remove(&mut self, segment: &VersionSegment, scan_pos: Point) {
        // same logic as for insert but now we remove

        let mut s = 1 << self.power;
        let mut i = s - 1;
        let range = segment.x_segment.y_range();

        let mut early_out = false;

        while s > 1 {
            let middle = self.nodes[i].range.middle();
            s >>= 1;
            if range.max <= middle {
                i -= s;
            } else if range.min >= middle {
                i += s;
            } else {
                early_out = true;
                break;
            }
        }

        // at this moment segment is in the middle of node[i]
        if !early_out || self.nodes[i].range == range {
            self.nodes[i].list.remove_segment(segment, scan_pos);
            return;
        }

        let i_lt = i - s;
        let i_rt = i + s;

        let sm = s;

        if range.min == self.nodes[i_lt].range.min {
            self.nodes[i_lt].list.remove_segment(segment, scan_pos);
        } else {
            early_out = false;
            let e = range.min;
            i = i_lt;

            while s > 1 {
                let middle = self.nodes[i].range.middle();

                s >>= 1;

                let lt = i - s;
                let rt = i + s;

                i = lt;

                if e <= middle {
                    self.nodes[rt].list.remove_segment(segment, scan_pos);
                    if e == middle {
                        early_out = true;
                        break;
                    }
                    i = lt;
                } else {
                    i = rt;
                }
            }

            if !early_out {
                self.nodes[i].list.remove_segment(segment, scan_pos);
            }
        }

        if range.max == self.nodes[i_rt].range.max {
            self.nodes[i_rt].list.remove_segment(segment, scan_pos);
        } else {
            early_out = false;
            let e = range.max;
            s = sm;
            i = i_rt;

            while s > 1 {
                let middle = self.nodes[i].range.middle();

                s >>= 1;
                let lt = i - s;
                let rt = i + s;

                if e >= middle {
                    self.nodes[lt].list.remove_segment(segment, scan_pos);
                    if e == middle {
                        early_out = true;
                        break;
                    }
                    i = rt;
                } else {
                    i = lt;
                }
            }

            if !early_out {
                self.nodes[i].list.remove_segment(segment, scan_pos);
            }
        }
    }

    fn cross(&mut self, index: usize, this: XSegment, scan_pos: Point) -> Option<CrossSegment> {
        let mut j = 0;

        let list = &mut self.nodes[index].list;

        while j < list.len() {
            let scan = &list[j];
            if scan.x_segment.b.order_by_line_compare(scan_pos) {
                list.swap_remove_index(j);
                continue;
            }

            // order is important! this * scan
            if let Some(cross) = this.cross(&scan.x_segment) {
                let index = scan.index;
                let segment = scan.clone();
                self.remove(&segment, scan_pos);
                return Some(CrossSegment { index, cross });
            }
            j += 1;
        }

        None
    }

    fn find_node(&self, index: usize, value: i32, scale: usize) -> usize {
        let mut s = scale;
        let mut i = index;
        while s > 1 {
            let middle = self.nodes[i].range.middle();

            if value == middle {
                return i;
            }

            s >>= 1;

            if value < middle {
                i -= s;
            } else {
                i += s;
            }
        }

        i
    }
}

impl ScanSplitStore for ScanSplitTree {
    fn intersect(&mut self, this: XSegment) -> Option<CrossSegment> {
        let mut s = 1 << self.power;
        let mut i = s - 1;
        let range = this.y_range();
        let scan_pos= this.a;

        let mut early_out = false;

        while s > 0 {
            let cross = self.cross(i, this, scan_pos);
            if !cross.is_none() {
                return cross;
            }
            s >>= 1;

            let middle = self.nodes[i].range.middle();
            if range.max <= middle {
                i -= s;
            } else if range.min >= middle {
                i += s;
            } else {
                early_out = true;
                break;
            }
        }

        if !early_out {
            // no need more search
            return None;
        }

        let i_lt = self.find_node(i - s, range.min, s);
        let i_rt = self.find_node(i + s, range.max, s);

        i = i_lt;

        while i <= i_rt {
            let cross = self.cross(i, this, scan_pos);
            if !cross.is_none() {
                return cross;
            }
            i += 1;
        }

        return None;
    }

    fn insert(&mut self, segment: VersionSegment) {
        let mut s = 1 << self.power;
        let mut i = s - 1;
        let range = segment.x_segment.y_range();

        let mut early_out = false;

        while s > 1 {
            let middle = self.nodes[i].range.middle();
            s >>= 1;
            if range.max <= middle {
                i -= s;
            } else if range.min >= middle {
                i += s;
            } else {
                early_out = true;
                break;
            }
        }
        // at this moment segment is in the middle of node[i]
        if !early_out || self.nodes[i].range == range {
            self.nodes[i].list.push(segment);
            return;
        }

        let i_lt = i - s;
        let i_rt = i + s;

        let sm = s;

        if range.min == self.nodes[i_lt].range.min {
            self.nodes[i_lt].list.push(segment.clone());
        } else {
            early_out = false;
            let e = range.min;
            i = i_lt;

            while s > 1 {
                let middle = self.nodes[i].range.middle();

                s >>= 1;

                let lt = i - s;
                let rt = i + s;

                if e <= middle {
                    self.nodes[rt].list.push(segment.clone());
                    if e == middle {
                        // no more append is possible
                        early_out = true;
                        break;
                    }
                    i = lt;
                } else {
                    i = rt;
                }
            }

            // add to leaf anyway
            if !early_out {
                // we down to a leaf, add it anyway
                self.nodes[i].list.push(segment.clone());
            }
        }

        if range.max == self.nodes[i_rt].range.max {
            self.nodes[i_rt].list.push(segment);
        } else {
            early_out = false;
            let e = range.max;
            let mut s = sm;
            i = i_rt;

            while s > 1 {
                let middle = self.nodes[i].range.middle();

                s >>= 1;
                let lt = i - s;
                let rt = i + s;

                if e >= middle {
                    self.nodes[lt].list.push(segment.clone());
                    if e == middle {
                        // no more append is possible
                        early_out = true;
                        break;
                    }
                    i = rt;
                } else {
                    i = lt;
                }
            }

            if !early_out {
                // we down to a leaf, add it anyway
                self.nodes[i].list.push(segment);
            }
        }
    }

    fn clear(&mut self) {
        for n in self.nodes.iter_mut() {
            n.list.clear()
        }
    }
}

trait Log2Extension {
    fn log2(&self) -> usize;
}

impl Log2Extension for i32 {
    fn log2(&self) -> usize {
        debug_assert!(self >= &0);
        let n = self.leading_zeros();
        (i32::BITS - n) as usize
    }
}

impl LineRange {
    fn middle(&self) -> i32 {
        (self.max + self.min) >> 1
    }
    fn log2(&self) -> usize {
        (self.max - self.min).log2()
    }
}

#[cfg(test)]
impl ScanSplitTree {
    fn with_power(range: LineRange, power: usize) -> Self {
        let nodes = Self::create_nodes(range, power);
        Self { power, nodes }
    }
    fn count(&self) -> usize {
        let mut s = 0;
        for node in self.nodes.iter() {
            s += node.list.len();
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use i_float::point::Point;
    use rand::Rng;
    use crate::x_segment::XSegment;
    use crate::line_range::LineRange;
    use crate::split::scan_list::ScanSplitList;
    use crate::split::scan_tree::ScanSplitTree;
    use crate::split::shape_edge_cross::EdgeCrossType;
    use crate::split::scan_store::ScanSplitStore;
    use crate::split::version_index::{DualIndex, VersionedIndex};
    use crate::split::version_segment::VersionSegment;

    #[test]
    fn test_0() {
        let nodes = ScanSplitTree::create_nodes(LineRange { min: 0, max: 128 }, 4);
        assert_eq!(31, nodes.len());
    }

    #[test]
    fn test_1() {
        let nodes = ScanSplitTree::create_nodes(LineRange { min: 0, max: 128 }, 5);
        assert_eq!(63, nodes.len());
    }

    #[test]
    fn test_02() {
        let mut tree = ScanSplitTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: Point::new(0, 1), b: Point::new(0, 127) };
        tree.insert(VersionSegment { index: VersionedIndex::EMPTY, x_segment });


        assert_eq!(true, !tree.nodes[0].list.is_empty());
        assert_eq!(true, tree.nodes[1].list.is_empty());
        assert_eq!(true, !tree.nodes[2].list.is_empty());

        assert_eq!(true, tree.nodes[3].list.is_empty());

        assert_eq!(true, tree.nodes[4].list.is_empty());
        assert_eq!(true, !tree.nodes[5].list.is_empty());
        assert_eq!(true, tree.nodes[6].list.is_empty());

        assert_eq!(true, tree.nodes[7].list.is_empty());

        assert_eq!(true, tree.nodes[8].list.is_empty());
        assert_eq!(true, !tree.nodes[9].list.is_empty());
        assert_eq!(true, tree.nodes[10].list.is_empty());

        assert_eq!(true, tree.nodes[11].list.is_empty());

        assert_eq!(true, !tree.nodes[12].list.is_empty());
        assert_eq!(true, tree.nodes[13].list.is_empty());
        assert_eq!(true, !tree.nodes[14].list.is_empty());
    }

    #[test]
    fn test_03() {
        let mut tree = ScanSplitTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: Point::new(0, 16), b: Point::new(0, 112) };
        tree.insert(VersionSegment { index: VersionedIndex::EMPTY, x_segment });


        assert_eq!(true, tree.nodes[0].list.is_empty());
        assert_eq!(true, tree.nodes[1].list.is_empty());
        assert_eq!(true, !tree.nodes[2].list.is_empty());

        assert_eq!(true, tree.nodes[3].list.is_empty());

        assert_eq!(true, tree.nodes[4].list.is_empty());
        assert_eq!(true, !tree.nodes[5].list.is_empty());
        assert_eq!(true, tree.nodes[6].list.is_empty());

        assert_eq!(true, tree.nodes[7].list.is_empty());

        assert_eq!(true, tree.nodes[8].list.is_empty());
        assert_eq!(true, !tree.nodes[9].list.is_empty());
        assert_eq!(true, tree.nodes[10].list.is_empty());

        assert_eq!(true, tree.nodes[11].list.is_empty());

        assert_eq!(true, !tree.nodes[12].list.is_empty());
        assert_eq!(true, tree.nodes[13].list.is_empty());
        assert_eq!(true, tree.nodes[14].list.is_empty());
    }

    #[test]
    fn test_04() {
        let mut tree = ScanSplitTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: Point::new(0, 17), b: Point::new(0, 111) };
        tree.insert(VersionSegment { index: VersionedIndex::EMPTY, x_segment });


        assert_eq!(true, tree.nodes[0].list.is_empty());
        assert_eq!(true, tree.nodes[1].list.is_empty());
        assert_eq!(true, !tree.nodes[2].list.is_empty());

        assert_eq!(true, tree.nodes[3].list.is_empty());

        assert_eq!(true, tree.nodes[4].list.is_empty());
        assert_eq!(true, !tree.nodes[5].list.is_empty());
        assert_eq!(true, tree.nodes[6].list.is_empty());

        assert_eq!(true, tree.nodes[7].list.is_empty());

        assert_eq!(true, tree.nodes[8].list.is_empty());
        assert_eq!(true, !tree.nodes[9].list.is_empty());
        assert_eq!(true, tree.nodes[10].list.is_empty());

        assert_eq!(true, tree.nodes[11].list.is_empty());

        assert_eq!(true, !tree.nodes[12].list.is_empty());
        assert_eq!(true, tree.nodes[13].list.is_empty());
        assert_eq!(true, tree.nodes[14].list.is_empty());
    }

    #[test]
    fn test_05() {
        let mut tree = ScanSplitTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: Point::new(0, 32), b: Point::new(0, 96) };
        tree.insert(VersionSegment { index: VersionedIndex::EMPTY, x_segment });


        assert_eq!(true, tree.nodes[0].list.is_empty());
        assert_eq!(true, tree.nodes[1].list.is_empty());
        assert_eq!(true, tree.nodes[2].list.is_empty());

        assert_eq!(true, tree.nodes[3].list.is_empty());

        assert_eq!(true, tree.nodes[4].list.is_empty());
        assert_eq!(true, !tree.nodes[5].list.is_empty());
        assert_eq!(true, tree.nodes[6].list.is_empty());

        assert_eq!(true, tree.nodes[7].list.is_empty());

        assert_eq!(true, tree.nodes[8].list.is_empty());
        assert_eq!(true, !tree.nodes[9].list.is_empty());
        assert_eq!(true, tree.nodes[10].list.is_empty());

        assert_eq!(true, tree.nodes[11].list.is_empty());

        assert_eq!(true, tree.nodes[12].list.is_empty());
        assert_eq!(true, tree.nodes[13].list.is_empty());
        assert_eq!(true, tree.nodes[14].list.is_empty());
    }

    #[test]
    fn test_06() {
        let mut tree = ScanSplitTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: Point::new(0, 33), b: Point::new(0, 95) };
        tree.insert(VersionSegment { index: VersionedIndex::EMPTY, x_segment });


        assert_eq!(true, tree.nodes[0].list.is_empty());
        assert_eq!(true, tree.nodes[1].list.is_empty());
        assert_eq!(true, tree.nodes[2].list.is_empty());

        assert_eq!(true, tree.nodes[3].list.is_empty());

        assert_eq!(true, !tree.nodes[4].list.is_empty());
        assert_eq!(true, tree.nodes[5].list.is_empty());
        assert_eq!(true, !tree.nodes[6].list.is_empty());

        assert_eq!(true, tree.nodes[7].list.is_empty());

        assert_eq!(true, !tree.nodes[8].list.is_empty());
        assert_eq!(true, tree.nodes[9].list.is_empty());
        assert_eq!(true, !tree.nodes[10].list.is_empty());

        assert_eq!(true, tree.nodes[11].list.is_empty());

        assert_eq!(true, tree.nodes[12].list.is_empty());
        assert_eq!(true, tree.nodes[13].list.is_empty());
        assert_eq!(true, tree.nodes[14].list.is_empty());
    }

    #[test]
    fn test_07() {
        let mut tree = ScanSplitTree::with_power(LineRange { min: -8, max: 9 }, 3);
        let version = VersionedIndex { version: 0, index: DualIndex::EMPTY };

        let a0 = Point::new(0, -6);
        let b0 = Point::new(8, 0);
        let a1 = Point::new(0, 3);
        let b1 = Point::new(8, 8);

        let vs = VersionSegment { index: version.clone(), x_segment: XSegment { a: a0, b: b0 } };
        let xs = XSegment { a: a1, b: b1 };

        tree.insert(vs);
        let r1 = tree.intersect(xs);

        assert_eq!(true, r1.is_none());
        assert_eq!(true, tree.count() > 0)
    }

    #[test]
    fn test_08() {
        let test_set = vec![
            XSegment { a: Point::new(-5, 0), b: Point::new(-5, 7) },
            XSegment { a: Point::new(-5, 1), b: Point::new(-4, 1) },
            XSegment { a: Point::new(0, 4), b: Point::new(4, 4) },
            XSegment { a: Point::new(5, -8), b: Point::new(7, -6) },
        ];

        let result = intersect_test(test_set);

        assert_eq!(1, result.len());
    }

    #[test]
    fn test_09() {
        let test_set = vec![
            XSegment { a: Point::new(-5, -6), b: Point::new(-5, 0) },
            XSegment { a: Point::new(0, -7), b: Point::new(7, -7) },
            XSegment { a: Point::new(3, -7), b: Point::new(3, -2) },
            XSegment { a: Point::new(6, -7), b: Point::new(12, -7) },
        ];

        let result = intersect_test(test_set);

        assert_eq!(1, result.len());
    }

    #[test]
    fn test_10() {
        let test_set = vec![
            XSegment { a: Point::new(-8, -1), b: Point::new(-3, 4) },
            XSegment { a: Point::new(-6, 3), b: Point::new(-1, 8) },
            XSegment { a: Point::new(-5, 4), b: Point::new(-1, 4) },
            XSegment { a: Point::new(-2, -1), b: Point::new(-2, 0) },
        ];

        let result = intersect_test(test_set);

        assert_eq!(1, result.len());
    }

    fn intersect_test(test_set: Vec<XSegment>) -> Vec<Point> {
        let mut result = Vec::new();
        let range = range(&test_set);
        let mut tree = ScanSplitTree::new(range, test_set.len());

        let mut i = 0;
        for s in test_set.iter() {
            if let Some(res) = &tree.intersect(s.clone()) {
                result.push(res.cross.point.clone());
                if res.cross.nature == EdgeCrossType::Penetrate {
                    result.push(res.cross.second);
                }
            } else {
                let index = VersionedIndex { version: i, index: DualIndex::EMPTY };
                let v = VersionSegment { index, x_segment: s.clone() };
                tree.insert(v);
                i += 1;
            }
        }

        result
    }

    fn range(list: &Vec<XSegment>) -> LineRange {
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for s in list.iter() {
            min = min.min(s.a.y);
            min = min.min(s.b.y);

            max = max.max(s.a.y);
            max = max.max(s.b.y);
        }

        LineRange { min, max }
    }

    #[test]
    fn test_random_intersect_0() {
        let range: std::ops::Range<i32> = -1000..1000;
        let mut list = ScanSplitList::new(1);
        let mut tree = ScanSplitTree::with_power(LineRange { min: range.start, max: range.end }, 5);
        let index = VersionedIndex { version: 0, index: DualIndex::EMPTY };
        let mut rng = rand::thread_rng();

        for _ in 0..100_000 {
            let a0 = Point::new(0, rng.gen_range(range.clone()));
            let b0 = Point::new(8, rng.gen_range(range.clone()));
            let a1 = Point::new(0, rng.gen_range(range.clone()));
            let b1 = Point::new(8, rng.gen_range(range.clone()));

            let vs = VersionSegment { index, x_segment: XSegment { a: a0, b: b0 } };
            let xs = XSegment { a: a1, b: b1 };
            list.insert(vs.clone());
            tree.insert(vs);

            let r0 = list.intersect(xs);
            let r1 = tree.intersect(xs);

            if r0.is_none() != r1.is_none() {
                print!("a0: {a0}, b0: {b0}, a1: {a1}, b1: {b1}");
                break;
            }

            if !r1.is_none() {
                print!("a0: {a0}, b0: {b0}, a1: {a1}, b1: {b1}");
                break;
            }

            if r1.is_none() {
                assert_eq!(true, tree.count() > 0);
            } else {
                assert_eq!(0, tree.count());
            }

            assert_eq!(r0.is_none(), r1.is_none());

            list.clear();
            tree.clear();

            assert_eq!(0, tree.count());
        }
    }
}