use crate::geom::x_segment::XSegment;

pub(super) struct GridLayout {
    pub(super) min_x: i32,
    pub(super) max_x: i32,
    pub(super) power: u32,
}

impl GridLayout {
    #[inline]
    pub(super) fn index(&self, x: i32) -> usize {
        ((x - self.min_x) >> self.power) as usize
    }

    #[inline]
    pub(super) fn pos(&self, index: usize) -> i32 {
        (index << self.power) as i32 + self.min_x
    }

    pub(super) fn new<I>(iter: I, count: usize) -> Option<Self>
    where
        I: Iterator<Item=XSegment>,
    {
        let mut iter = iter.peekable();
        let first = iter.peek()?;
        let min_x = first.a.x;
        let mut max_x = first.b.x;

        for s in iter {
            max_x = s.b.x.max(max_x);
        }

        let log = count.ilog2();
        let max_power = log >> 1;

        Self::with_min_max(min_x, max_x, max_power)
    }

    fn with_min_max(min_x: i32, max_x: i32, max_power: u32) -> Option<Self> {
        let dx = max_x - min_x;
        if dx < 4 {
            return None;
        }
        let log = dx.ilog2();
        let power = if log > max_power {
            log - max_power
        } else {
            1
        };

        Some(Self { min_x, max_x, power })
    }
}

#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_float::int::rect::IntRect;
    use i_float::triangle::Triangle;
    use rand::Rng;
    use crate::geom::x_segment::XSegment;
    use crate::split::fragment::FragmentBuffer;
    use crate::split::grid_layout::GridLayout;

    #[test]
    fn test_0() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 0 }, b: IntPoint { x: 6, y: 3 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 0, max_y: 1 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 0, max_y: 2 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 1, max_y: 3 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_0_inv() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 3 }, b: IntPoint { x: 6, y: 0 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 2, max_y: 3 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 1, max_y: 3 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 0, max_y: 2 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_1() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 1 }, b: IntPoint { x: 6, y: 4 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 1, max_y: 2 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 1, max_y: 3 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 2, max_y: 4 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_1_inv() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 4 }, b: IntPoint { x: 6, y: 1 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 3, max_y: 4 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 2, max_y: 4 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 1, max_y: 3 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_2() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: -1 }, b: IntPoint { x: 6, y: 2 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: -1, max_y: 0 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: -1, max_y: 1 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 0, max_y: 2 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_2_inv() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 2 }, b: IntPoint { x: 6, y: -1 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 1, max_y: 2 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 0, max_y: 2 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: -1, max_y: 1 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_3() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 0 }, b: IntPoint { x: 6, y: 1 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 0, max_y: 1 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 0, max_y: 1 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 0, max_y: 1 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_3_inv() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 1 }, b: IntPoint { x: 6, y: 0 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 0, max_y: 1 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 0, max_y: 1 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 0, max_y: 1 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }

    #[test]
    fn test_4() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 0, y: 0 }, b: IntPoint { x: 5, y: 3 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 0, max_x: 2, min_y: 0, max_y: 2 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 1, max_y: 3 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 5, min_y: 2, max_y: 3 });

        validate_rect(&segment, &buffer.groups[0][0].rect);
        validate_rect(&segment, &buffer.groups[1][0].rect);
        validate_rect(&segment, &buffer.groups[2][0].rect);
    }


    #[test]
    fn test_5() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 0 }, b: IntPoint { x: 4, y: 5 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 2);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 0, max_y: 2 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 1, max_y: 5 });
    }

    #[test]
    fn test_6() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 0, y: 0 }, b: IntPoint { x: 6, y: 6 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 0, max_x: 2, min_y: 0, max_y: 2 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 2, max_y: 4 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 4, max_y: 6 });
    }

    #[test]
    fn test_7() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 1 }, b: IntPoint { x: 5, y: 5 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 1, max_y: 2 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 2, max_y: 4 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 5, min_y: 4, max_y: 5 });
    }

    #[test]
    fn test_7_inv() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 5 }, b: IntPoint { x: 5, y: 1 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 3);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 2, min_y: 4, max_y: 5 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 2, max_y: 4 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 5, min_y: 1, max_y: 2 });
    }

    #[test]
    fn test_8() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 0, y: 0 }, b: IntPoint { x: 7, y: 0 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 4);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 0, max_x: 2, min_y: 0, max_y: 0 });
        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 0, max_y: 0 });
        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: 4, max_x: 6, min_y: 0, max_y: 0 });
        rect_compare(&buffer.groups[3][0].rect, &IntRect { min_x: 6, max_x: 7, min_y: 0, max_y: 0 });
    }

    #[test]
    fn test_9() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 1 }, b: IntPoint { x: 1, y: 9 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 1);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 1, min_y: 1, max_y: 9 });
    }

    #[test]
    fn test_9_inv() {
        let layout = GridLayout { min_x: 0, max_x: 12, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: 1, y: 9 }, b: IntPoint { x: 1, y: 1 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 7);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 1);

        rect_compare(&buffer.groups[0][0].rect, &IntRect { min_x: 1, max_x: 1, min_y: 1, max_y: 9 });
    }

    #[test]
    fn test_10() {
        let layout = GridLayout {
            min_x: -1000_000,
            max_x: 1000_000,
            power: 10,
        };

        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: -100_000, y: -100_000 }, b: IntPoint { x: 100_000, y: 100_000 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        for fragment in buffer.groups.iter().flatten() {
            let rect = &fragment.rect;
            let p0 = IntPoint::new(rect.min_x, rect.min_y);
            let p1 = IntPoint::new(rect.max_x, rect.min_y);
            let p2 = IntPoint::new(rect.min_x, rect.max_y);
            let p3 = IntPoint::new(rect.max_x, rect.max_y);

            assert!(Triangle::is_cw_or_line_point(p0, segment.a, segment.b));
            assert!(Triangle::is_cw_or_line_point(p1, segment.a, segment.b));
            assert!(Triangle::is_cw_or_line_point(p2, segment.b, segment.a));
            assert!(Triangle::is_cw_or_line_point(p3, segment.b, segment.a));
        }
    }

    #[test]
    fn test_11() {
        let layout = GridLayout { min_x: -10, max_x: 10, power: 1 };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: -6, y: 0 }, b: IntPoint { x: 4, y: 2 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 11);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 5);

        validate_rect(&segment, &buffer.groups[2][0].rect);
        validate_rect(&segment, &buffer.groups[3][0].rect);
        validate_rect(&segment, &buffer.groups[4][0].rect);
        validate_rect(&segment, &buffer.groups[5][0].rect);
        validate_rect(&segment, &buffer.groups[6][0].rect);

        rect_compare(&buffer.groups[2][0].rect, &IntRect { min_x: -6, max_x: -4, min_y: 0, max_y: 1 });
        rect_compare(&buffer.groups[3][0].rect, &IntRect { min_x: -4, max_x: -2, min_y: 0, max_y: 1 });
        rect_compare(&buffer.groups[4][0].rect, &IntRect { min_x: -2, max_x: 0, min_y: 0, max_y: 2 });
        rect_compare(&buffer.groups[5][0].rect, &IntRect { min_x: 0, max_x: 2, min_y: 1, max_y: 2 });
        rect_compare(&buffer.groups[6][0].rect, &IntRect { min_x: 2, max_x: 4, min_y: 1, max_y: 2 });
    }

    #[test]
    fn test_12() {
        let layout = GridLayout {
            min_x: -10,
            max_x: 10,
            power: 1,
        };
        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: -8, y: -10 }, b: IntPoint { x: -8, y: -9 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        assert_eq!(buffer.groups.len(), 11);
        assert_eq!(buffer.groups.iter().fold(0usize, |s, it| s + it.len()), 1);

        rect_compare(&buffer.groups[1][0].rect, &IntRect { min_x: -8, max_x: -8, min_y: -10, max_y: -9 });
    }

    #[test]
    fn test_13() {
        let layout = GridLayout {
            min_x: -100_000,
            max_x: 100_000,
            power: 10,
        };

        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: -83143, y: 65289 }, b: IntPoint { x: 45253, y: -76778 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        for fragment in buffer.groups.iter().flatten() {
            validate_rect(&segment, &fragment.rect);
        }
    }

    #[test]
    fn test_14() {
        let layout = GridLayout {
            min_x: -100_000,
            max_x: 100_000,
            power: 10,
        };

        let mut buffer = FragmentBuffer::new(layout);

        let segment = XSegment { a: IntPoint { x: -78454, y: -40819 }, b: IntPoint { x: 47599, y: -57780 } };
        let segments = vec![segment];
        buffer.init_fragment_buffer(segments.iter().copied());

        buffer.add_segment(0, segment);

        for fragment in buffer.groups.iter().flatten() {
            validate_rect(&segment, &fragment.rect);
        }
    }

    #[test]
    fn test_loop_range_0() {
        let min_x = -10;
        let max_x = 10;
        let min_y = -10;
        let max_y = 10;

        let layout = GridLayout {
            min_x: min_x,
            max_x: max_x,
            power: 1,
        };

        let mut buffer = FragmentBuffer::new(layout);

        let mut a = IntPoint::new(min_x, min_y - 1);

        while let Some(ai) = next_point(a, min_y, max_x, max_y) {
            a = ai;
            let mut b = a;
            while let Some(bi) = next_point(b, min_y, max_x, max_y) {
                b = bi;

                let segment = XSegment { a, b };

                let segments = vec![segment];
                buffer.init_fragment_buffer(segments.iter().copied());

                buffer.add_segment(0, segment);

                for fragment in buffer.groups.iter().flatten() {
                    validate_rect(&segment, &fragment.rect);
                }

                buffer.clear();
            }
        }
    }

    #[test]
    fn test_loop_range_1() {
        let min_x = -20;
        let max_x = 20;
        let min_y = -20;
        let max_y = 20;

        let layout = GridLayout {
            min_x: min_x,
            max_x: max_x,
            power: 2,
        };

        let mut buffer = FragmentBuffer::new(layout);

        let mut a = IntPoint::new(min_x, min_y - 1);

        while let Some(ai) = next_point(a, min_y, max_x, max_y) {
            a = ai;
            let mut b = a;
            while let Some(bi) = next_point(b, min_y, max_x, max_y) {
                b = bi;

                let segment = XSegment { a, b };

                let segments = vec![segment];
                buffer.init_fragment_buffer(segments.iter().copied());

                buffer.add_segment(0, segment);

                for fragment in buffer.groups.iter().flatten() {
                    validate_rect(&segment, &fragment.rect);
                }

                buffer.clear();
            }
        }
    }

    #[test]
    fn test_loop_range_2() {
        let min_x = -20;
        let max_x = 20;
        let min_y = -20;
        let max_y = 20;

        let layout = GridLayout {
            min_x: min_x,
            max_x: max_x,
            power: 3,
        };

        let mut buffer = FragmentBuffer::new(layout);

        let mut a = IntPoint::new(min_x, min_y - 1);

        while let Some(ai) = next_point(a, min_y, max_x, max_y) {
            a = ai;
            let mut b = a;
            while let Some(bi) = next_point(b, min_y, max_x, max_y) {
                b = bi;

                let segment = XSegment { a, b };

                let segments = vec![segment];
                buffer.init_fragment_buffer(segments.iter().copied());

                buffer.add_segment(0, segment);

                for fragment in buffer.groups.iter().flatten() {
                    validate_rect(&segment, &fragment.rect);
                }

                buffer.clear();
            }
        }
    }

    #[test]
    fn test_loop_range_3() {
        let min_x = -20;
        let max_x = 20;
        let min_y = -20;
        let max_y = 20;

        let layout = GridLayout {
            min_x: min_x,
            max_x: max_x,
            power: 4,
        };

        let mut buffer = FragmentBuffer::new(layout);

        let mut a = IntPoint::new(min_x, min_y - 1);

        while let Some(ai) = next_point(a, min_y, max_x, max_y) {
            a = ai;
            let mut b = a;
            while let Some(bi) = next_point(b, min_y, max_x, max_y) {
                b = bi;

                let segment = XSegment { a, b };

                let segments = vec![segment];
                buffer.init_fragment_buffer(segments.iter().copied());

                buffer.add_segment(0, segment);

                for fragment in buffer.groups.iter().flatten() {
                    validate_rect(&segment, &fragment.rect);
                }

                buffer.clear();
            }
        }
    }

    #[test]
    fn test_random_0() {
        let layout = GridLayout {
            min_x: -100,
            max_x: 100,
            power: 4,
        };

        let range = layout.min_x..=layout.max_x;
        let mut buffer = FragmentBuffer::new(layout);

        let mut rng = rand::thread_rng();

        for _ in 0..10_000 {
            let x0 = rng.gen_range(range.clone());
            let y0 = rng.gen_range(range.clone());
            let x1 = rng.gen_range(range.clone());
            let y1 = rng.gen_range(range.clone());

            let a = IntPoint::new(x0, y0);
            let b = IntPoint::new(x1, y1);

            let segment = if a <= b {
                XSegment { a, b }
            } else {
                XSegment { a: b, b: a }
            };

            let segments = vec![segment];
            buffer.init_fragment_buffer(segments.iter().copied());

            buffer.add_segment(0, segment);

            for fragment in buffer.groups.iter().flatten() {
                validate_rect(&segment, &fragment.rect);
            }

            buffer.clear();
        }
    }

    #[test]
    fn test_random_1() {
        let layout = GridLayout {
            min_x: -100_000,
            max_x: 100_000,
            power: 10,
        };

        let range = layout.min_x..=layout.max_x;
        let mut buffer = FragmentBuffer::new(layout);

        let mut rng = rand::thread_rng();

        for _ in 0..10_000 {
            let x0 = rng.gen_range(range.clone());
            let y0 = rng.gen_range(range.clone());
            let x1 = rng.gen_range(range.clone());
            let y1 = rng.gen_range(range.clone());

            let a = IntPoint::new(x0, y0);
            let b = IntPoint::new(x1, y1);

            let segment = if a <= b {
                XSegment { a, b }
            } else {
                XSegment { a: b, b: a }
            };

            let segments = vec![segment];
            buffer.init_fragment_buffer(segments.iter().copied());

            buffer.add_segment(0, segment);

            for fragment in buffer.groups.iter().flatten() {
                validate_rect(&segment, &fragment.rect);
            }

            buffer.clear();
        }
    }

    #[inline]
    fn rect_compare(a: &IntRect, b: &IntRect) {
        assert_eq!(a.min_x, b.min_x);
        assert_eq!(a.max_x, b.max_x);
        assert_eq!(a.min_y, b.min_y);
        assert_eq!(a.max_y, b.max_y);
    }

    #[inline]
    fn validate_rect(segment: &XSegment, rect: &IntRect) {
        let p0 = IntPoint::new(rect.min_x, rect.min_y);
        let p1 = IntPoint::new(rect.max_x, rect.min_y);
        let p2 = IntPoint::new(rect.min_x, rect.max_y);
        let p3 = IntPoint::new(rect.max_x, rect.max_y);

        let a0 = Triangle::area_two_point(p0, segment.a, segment.b);
        let a1 = Triangle::area_two_point(p1, segment.a, segment.b);
        let a2 = Triangle::area_two_point(p2, segment.b, segment.a);
        let a3 = Triangle::area_two_point(p3, segment.b, segment.a);

        assert!(a0 >= 0);
        assert!(a1 >= 0);
        assert!(a2 >= 0);
        assert!(a3 >= 0);
    }

    #[inline]
    fn next_point(point: IntPoint, min_y: i32, max_x: i32, max_y: i32) -> Option<IntPoint> {
        let mut x = point.x;
        let mut y = point.y + 1;
        if y <= max_y {
            return Some(IntPoint::new(x, y));
        }
        y = min_y;
        x += 1;
        if x <= max_x {
            return Some(IntPoint::new(x, y));
        }

        None
    }
}