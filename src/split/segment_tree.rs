use crate::util::SwapRemoveIndex;
use crate::line_range::LineRange;
use crate::split::fragment::Fragment;
use crate::split::line_mark::LineMark;
use crate::split::solver::SplitSolver;

#[derive(Debug, Clone)]
struct IntervalNode {
    range: LineRange,
    fragments: Vec<Fragment>,
}

impl IntervalNode {
    fn new(range: LineRange) -> Self {
        Self { range, fragments: Vec::with_capacity(4) }
    }
}

pub(super) struct SegmentTree {
    power: usize,
    nodes: Vec<IntervalNode>,
}

impl SegmentTree {
    #[inline]
    pub(super) fn new(range: LineRange, power: usize) -> Self {
        let nodes = Self::create_nodes(range, power);
        Self { power, nodes }
    }

    fn create_nodes(range: LineRange, power: usize) -> Vec<IntervalNode> {
        let n = 1 << power;

        // to make round more precise we use upscale/downscale
        let scale = 4;
        let len = range.width() as usize;
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

    pub(super) fn insert(&mut self, fragment: Fragment) {
        let mut s = 1 << self.power;
        let mut i = s - 1;
        let range = fragment.y_range();

        let mut early_out = false;

        while s > 1 {
            let middle = self.nodes[i].range.middle();
            s >>= 1;
            if range.max < middle {
                i -= s;
            } else if range.min > middle {
                i += s;
            } else {
                early_out = true;
                break;
            }
        }
        // at this moment segment is in the middle of node[i]
        if !early_out || self.nodes[i].range == range {
            self.nodes[i].fragments.push(fragment);
            return;
        }

        let i_lt = i - s;
        let i_rt = i + s;

        let sm = s;

        if range.min == self.nodes[i_lt].range.min {
            self.nodes[i_lt].fragments.push(fragment.clone());
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
                    self.nodes[rt].fragments.push(fragment.clone());
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
                self.nodes[i].fragments.push(fragment.clone())
            }
        }

        if range.max == self.nodes[i_rt].range.max {
            self.nodes[i_rt].fragments.push(fragment.clone())
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
                    self.nodes[lt].fragments.push(fragment.clone());
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
                self.nodes[i].fragments.push(fragment);
            }
        }
    }

    pub fn intersect(&mut self, this: &Fragment, marks: &mut Vec<LineMark>) -> bool {
        let mut s = 1 << self.power;
        let mut i = s - 1;
        let range = this.y_range();

        let mut early_out = false;
        let mut any_round = false;

        while s > 0 {
            let is_round = self.cross_node(i, &this, marks);
            any_round = is_round || any_round;
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
            return any_round;
        }

        // find most left index

        let mut j = i - s;
        let mut sj = s;
        while sj > 1 {
            let is_round = self.cross_node(j, &this, marks);
            any_round = is_round || any_round;

            let middle = self.nodes[j].range.middle();

            if range.min == middle {
                break;
            }

            sj >>= 1;

            if range.min < middle {
                j -= sj;
            } else {
                j += sj;
            }
        }

        // find most right index

        let i_lt = j;

        j = i + s;
        sj = s;
        while sj > 1 {
            let is_round = self.cross_node(j, &this, marks);
            any_round = is_round || any_round;

            let middle = self.nodes[j].range.middle();

            if range.max == middle {
                break;
            }

            sj >>= 1;

            if range.max < middle {
                j -= sj;
            } else {
                j += sj;
            }
        }

        let i_rt = j;

        i = i_lt;

        while i <= i_rt {
            let is_round = self.cross_node(i, &this, marks);
            any_round = is_round || any_round;
            i += 1;
        }

        any_round
    }

    #[inline]
    pub(super) fn clear(&mut self) {
        for n in self.nodes.iter_mut() {
            n.fragments.clear()
        }
    }

    fn cross_node(&mut self, index: usize, this: &Fragment, marks: &mut Vec<LineMark>) -> bool {
        let swipe_line = this.rect.min_x;
        let mut any_round = false;

        let mut j = 0;
        while j < self.nodes[index].fragments.len() {
            let scan = &self.nodes[index].fragments[j];

            if scan.rect.max_x < swipe_line {
                // remove item if it outside
                self.nodes[index].fragments.swap_remove_index(j);
                continue;
            }

            j += 1;

            if !scan.rect.is_intersect_border_include(&this.rect) {
                continue;
            }

            // MARK: the intersection, ensuring the right order for deterministic results

            let is_round = if this.x_segment < scan.x_segment {
                SplitSolver::cross(
                    this.index,
                    scan.index,
                    &this.x_segment,
                    &scan.x_segment,
                    marks,
                )
            } else {
                SplitSolver::cross(
                    scan.index,
                    this.index,
                    &scan.x_segment,
                    &this.x_segment,
                    marks,
                )
            };

            any_round = is_round || any_round
        }

        any_round
    }
}

impl LineRange {
    fn middle(&self) -> i32 {
        ((self.max as i64 + self.min as i64) >> 1) as i32
    }
}

#[cfg(test)]
impl SegmentTree {
    fn with_power(range: LineRange, power: usize) -> Self {
        let nodes = Self::create_nodes(range, power);
        Self { power, nodes }
    }
}

#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use crate::x_segment::XSegment;
    use crate::line_range::LineRange;
    use crate::split::fragment::Fragment;
    use crate::split::segment_tree::SegmentTree;

    #[test]
    fn test_0() {
        let nodes = SegmentTree::create_nodes(LineRange { min: 0, max: 128 }, 4);
        assert_eq!(31, nodes.len());
    }

    #[test]
    fn test_1() {
        let nodes = SegmentTree::create_nodes(LineRange { min: 0, max: 128 }, 5);
        assert_eq!(63, nodes.len());
    }

    #[test]
    fn test_02() {
        let mut tree = SegmentTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: IntPoint::new(0, 1), b: IntPoint::new(0, 127) };
        tree.insert(Fragment::with_index_and_segment(0, x_segment));


        assert_eq!(true, !tree.nodes[0].fragments.is_empty());
        assert_eq!(true, tree.nodes[1].fragments.is_empty());
        assert_eq!(true, !tree.nodes[2].fragments.is_empty());

        assert_eq!(true, tree.nodes[3].fragments.is_empty());

        assert_eq!(true, tree.nodes[4].fragments.is_empty());
        assert_eq!(true, !tree.nodes[5].fragments.is_empty());
        assert_eq!(true, tree.nodes[6].fragments.is_empty());

        assert_eq!(true, tree.nodes[7].fragments.is_empty());

        assert_eq!(true, tree.nodes[8].fragments.is_empty());
        assert_eq!(true, !tree.nodes[9].fragments.is_empty());
        assert_eq!(true, tree.nodes[10].fragments.is_empty());

        assert_eq!(true, tree.nodes[11].fragments.is_empty());

        assert_eq!(true, !tree.nodes[12].fragments.is_empty());
        assert_eq!(true, tree.nodes[13].fragments.is_empty());
        assert_eq!(true, !tree.nodes[14].fragments.is_empty());
    }

    #[test]
    fn test_03() {
        let mut tree = SegmentTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: IntPoint::new(0, 16), b: IntPoint::new(0, 112) };
        tree.insert(Fragment::with_index_and_segment(0, x_segment));


        assert_eq!(true, tree.nodes[0].fragments.is_empty());
        assert_eq!(true, tree.nodes[1].fragments.is_empty());
        assert_eq!(true, !tree.nodes[2].fragments.is_empty());

        assert_eq!(true, tree.nodes[3].fragments.is_empty());

        assert_eq!(true, tree.nodes[4].fragments.is_empty());
        assert_eq!(true, !tree.nodes[5].fragments.is_empty());
        assert_eq!(true, tree.nodes[6].fragments.is_empty());

        assert_eq!(true, tree.nodes[7].fragments.is_empty());

        assert_eq!(true, tree.nodes[8].fragments.is_empty());
        assert_eq!(true, !tree.nodes[9].fragments.is_empty());
        assert_eq!(true, tree.nodes[10].fragments.is_empty());

        assert_eq!(true, tree.nodes[11].fragments.is_empty());

        assert_eq!(true, !tree.nodes[12].fragments.is_empty());
        assert_eq!(true, tree.nodes[13].fragments.is_empty());
        assert_eq!(true, tree.nodes[14].fragments.is_empty());
    }

    #[test]
    fn test_04() {
        let mut tree = SegmentTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: IntPoint::new(0, 17), b: IntPoint::new(0, 111) };
        tree.insert(Fragment::with_index_and_segment(0, x_segment));


        assert_eq!(true, tree.nodes[0].fragments.is_empty());
        assert_eq!(true, tree.nodes[1].fragments.is_empty());
        assert_eq!(true, !tree.nodes[2].fragments.is_empty());

        assert_eq!(true, tree.nodes[3].fragments.is_empty());

        assert_eq!(true, tree.nodes[4].fragments.is_empty());
        assert_eq!(true, !tree.nodes[5].fragments.is_empty());
        assert_eq!(true, tree.nodes[6].fragments.is_empty());

        assert_eq!(true, tree.nodes[7].fragments.is_empty());

        assert_eq!(true, tree.nodes[8].fragments.is_empty());
        assert_eq!(true, !tree.nodes[9].fragments.is_empty());
        assert_eq!(true, tree.nodes[10].fragments.is_empty());

        assert_eq!(true, tree.nodes[11].fragments.is_empty());

        assert_eq!(true, !tree.nodes[12].fragments.is_empty());
        assert_eq!(true, tree.nodes[13].fragments.is_empty());
        assert_eq!(true, tree.nodes[14].fragments.is_empty());
    }

    #[test]
    fn test_05() {
        let mut tree = SegmentTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: IntPoint::new(0, 32), b: IntPoint::new(0, 96) };
        tree.insert(Fragment::with_index_and_segment(0, x_segment));


        assert_eq!(true, tree.nodes[0].fragments.is_empty());
        assert_eq!(true, tree.nodes[1].fragments.is_empty());
        assert_eq!(true, tree.nodes[2].fragments.is_empty());

        assert_eq!(true, tree.nodes[3].fragments.is_empty());

        assert_eq!(true, tree.nodes[4].fragments.is_empty());
        assert_eq!(true, !tree.nodes[5].fragments.is_empty());
        assert_eq!(true, tree.nodes[6].fragments.is_empty());

        assert_eq!(true, tree.nodes[7].fragments.is_empty());

        assert_eq!(true, tree.nodes[8].fragments.is_empty());
        assert_eq!(true, !tree.nodes[9].fragments.is_empty());
        assert_eq!(true, tree.nodes[10].fragments.is_empty());

        assert_eq!(true, tree.nodes[11].fragments.is_empty());

        assert_eq!(true, tree.nodes[12].fragments.is_empty());
        assert_eq!(true, tree.nodes[13].fragments.is_empty());
        assert_eq!(true, tree.nodes[14].fragments.is_empty());
    }

    #[test]
    fn test_06() {
        let mut tree = SegmentTree::with_power(LineRange { min: 0, max: 128 }, 3);
        let x_segment = XSegment { a: IntPoint::new(0, 33), b: IntPoint::new(0, 95) };
        tree.insert(Fragment::with_index_and_segment(0, x_segment));


        assert_eq!(true, tree.nodes[0].fragments.is_empty());
        assert_eq!(true, tree.nodes[1].fragments.is_empty());
        assert_eq!(true, tree.nodes[2].fragments.is_empty());

        assert_eq!(true, tree.nodes[3].fragments.is_empty());

        assert_eq!(true, !tree.nodes[4].fragments.is_empty());
        assert_eq!(true, tree.nodes[5].fragments.is_empty());
        assert_eq!(true, !tree.nodes[6].fragments.is_empty());

        assert_eq!(true, tree.nodes[7].fragments.is_empty());

        assert_eq!(true, !tree.nodes[8].fragments.is_empty());
        assert_eq!(true, tree.nodes[9].fragments.is_empty());
        assert_eq!(true, !tree.nodes[10].fragments.is_empty());

        assert_eq!(true, tree.nodes[11].fragments.is_empty());

        assert_eq!(true, tree.nodes[12].fragments.is_empty());
        assert_eq!(true, tree.nodes[13].fragments.is_empty());
        assert_eq!(true, tree.nodes[14].fragments.is_empty());
    }

    #[test]
    fn test_07() {
        let mut tree = SegmentTree::with_power(LineRange { min: -8, max: 9 }, 3);

        let a0 = IntPoint::new(0, -6);
        let b0 = IntPoint::new(8, 0);
        let a1 = IntPoint::new(0, 3);
        let b1 = IntPoint::new(8, 8);

        tree.insert(Fragment::with_index_and_segment(0, XSegment { a: a0, b: b0 }));

        let mut marks = Vec::new();

        tree.intersect(&Fragment::with_index_and_segment(0, XSegment { a: a1, b: b1 }), &mut marks);

        assert_eq!(true, marks.is_empty());
    }

    #[test]
    fn test_08() {
        let test_set = vec![
            XSegment { a: IntPoint::new(-5, 0), b: IntPoint::new(-5, 7) },
            XSegment { a: IntPoint::new(-5, 1), b: IntPoint::new(-4, 1) },
            XSegment { a: IntPoint::new(0, 4), b: IntPoint::new(4, 4) },
            XSegment { a: IntPoint::new(5, -8), b: IntPoint::new(7, -6) },
        ];

        let result = intersect_test(test_set);

        assert_eq!(1, result);
    }

    #[test]
    fn test_09() {
        let test_set = vec![
            XSegment { a: IntPoint::new(-5, -6), b: IntPoint::new(-5, 0) },
            XSegment { a: IntPoint::new(0, -7), b: IntPoint::new(7, -7) },
            XSegment { a: IntPoint::new(3, -7), b: IntPoint::new(3, -2) },
            XSegment { a: IntPoint::new(6, -7), b: IntPoint::new(12, -7) },
        ];

        let result = intersect_test(test_set);

        assert_eq!(2, result);
    }

    #[test]
    fn test_10() {
        let test_set = vec![
            XSegment { a: IntPoint::new(-8, -1), b: IntPoint::new(-3, 4) },
            XSegment { a: IntPoint::new(-6, 3), b: IntPoint::new(-1, 8) },
            XSegment { a: IntPoint::new(-5, 4), b: IntPoint::new(-1, 4) },
            XSegment { a: IntPoint::new(-2, -1), b: IntPoint::new(-2, 0) },
        ];

        let result = intersect_test(test_set);

        assert_eq!(1, result);
    }

    fn intersect_test(test_set: Vec<XSegment>) -> usize {
        let mut result = 0;
        let range = range(&test_set);
        let mut tree = SegmentTree::new(range, test_set.len());

        let mut marks = Vec::new();
        for s in test_set.iter() {
            marks.clear();
            let fragment = Fragment::with_index_and_segment(0, s.clone());
            tree.intersect(&fragment, &mut marks);

            if marks.is_empty() {
                tree.insert(fragment);
            } else {
                result += 1;
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
}