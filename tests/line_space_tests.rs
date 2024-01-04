#[cfg(test)]
mod tests {
    use rand::Rng;
    use i_overlay::space::line_indexer::LineIndexer;
    use i_overlay::space::line_range::LineRange;
    use i_overlay::space::line_segment::LineSegment;
    use i_overlay::space::line_space::LineSpace;

    #[test]
    fn test_0() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });

        assert_eq!(indexer.unsafe_index(LineRange { min: 0, max: 31 }), 0);
        assert_eq!(indexer.unsafe_index(LineRange { min: 1, max: 31 }), 0);
        assert_eq!(indexer.unsafe_index(LineRange { min: 1, max: 30 }), 0);
        assert_eq!(indexer.unsafe_index(LineRange { min: 0, max: 15 }), 0);
        assert_eq!(indexer.unsafe_index(LineRange { min: 16, max: 31 }), 0);
        assert_eq!(indexer.unsafe_index(LineRange { min: 10, max: 20 }), 0);
        assert_eq!(indexer.unsafe_index(LineRange { min: 0, max: 7 }), 1);
        assert_eq!(indexer.unsafe_index(LineRange { min: 8, max: 15 }), 1);
        assert_eq!(indexer.unsafe_index(LineRange { min: 16, max: 23 }), 2);
        assert_eq!(indexer.unsafe_index(LineRange { min: 24, max: 31 }), 2);
        assert_eq!(indexer.unsafe_index(LineRange { min: 4, max: 11 }), 1);
        assert_eq!(indexer.unsafe_index(LineRange { min: 12, max: 19 }), 3);
        assert_eq!(indexer.unsafe_index(LineRange { min: 20, max: 27 }), 2);
        assert_eq!(indexer.unsafe_index(LineRange { min: 10, max: 11 }), 5);
    }

    #[test]
    fn test_000() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        assert_eq!(indexer.unsafe_index(LineRange { min: 0, max: 31 }), 0);
    }

    #[test]
    fn test_001() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        assert_eq!(indexer.unsafe_index(LineRange { min: 0, max: 1 }), 4);
        assert_eq!(indexer.unsafe_index(LineRange { min: 0, max: 0 }), 4);
    }

    #[test]
    fn test_01() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 0, max: 31 });
        indices.sort();
        assert_eq!(indices, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec());
    }

    #[test]
    fn test_02() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 0, max: 15 });
        indices.sort();
        assert_eq!(indices, [0, 1, 3, 4, 5, 8, 9].to_vec());
    }

    #[test]
    fn test_03() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 0, max: 7 });
        indices.sort();
        assert_eq!(indices, [0, 1, 4, 8].to_vec());
    }

    #[test]
    fn test_04() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 0, max: 8 });
        indices.sort();
        assert_eq!(indices, [0, 1, 3, 4, 5, 8].to_vec());
    }

    #[test]
    fn test_05() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 0, max: 3 });
        indices.sort();
        assert_eq!(indices, [0, 1, 4].to_vec());
    }

    #[test]
    fn test_06() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 0, max: 4 });
        indices.sort();
        assert_eq!(indices, [0, 1, 4, 8].to_vec());
    }

    #[test]
    fn test_07() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 29, max: 31 });
        indices.sort();
        assert_eq!(indices, [0, 2, 7].to_vec());
    }

    #[test]
    fn test_08() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 28, max: 31 });
        indices.sort();
        assert_eq!(indices, [0, 2, 7].to_vec());
    }

    #[test]
    fn test_09() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 27, max: 31 });
        indices.sort();
        assert_eq!(indices, [0, 2, 7, 10].to_vec());
    }

    #[test]
    fn test_10() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 26, max: 31 });
        indices.sort();
        assert_eq!(indices, [0, 2, 7, 10].to_vec());
    }

    #[test]
    fn test_11() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 24, max: 31 });
        indices.sort();
        assert_eq!(indices, [0, 2, 7, 10].to_vec());
    }

    #[test]
    fn test_12() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 23, max: 31 });
        indices.sort();
        assert_eq!(indices, [0, 2, 3, 6, 7, 10].to_vec());
    }

    #[test]
    fn test_13() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 7, max: 28 });
        indices.sort();
        assert_eq!(indices, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec());
    }

    #[test]
    fn test_14() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 3, max: 29 });
        indices.sort();
        assert_eq!(indices, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec());
    }

    #[test]
    fn test_15() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 18, max: 26 });
        indices.sort();
        assert_eq!(indices, [0, 2, 3, 6, 7, 9, 10].to_vec());
    }

    #[test]
    fn test_16() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 22, max: 29 });
        indices.sort();
        assert_eq!(indices, [0, 2, 3, 6, 7, 10].to_vec());
    }

    #[test]
    fn test_17() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        let mut indices = indexer.heap_indices(LineRange { min: 19, max: 23 });
        indices.sort();
        assert_eq!(indices, [0, 2, 3, 6, 9, 10].to_vec());
    }

    #[test]
    fn test_18() {
        let indexer = LineIndexer::new(2, LineRange { min: 0, max: 31 });
        assert_eq!(indexer.index(LineRange { min: -35, max: 17 }), 0);
    }

    #[test]
    fn test_20() {
        let mut scan_list = LineSpace::new(2, LineRange { min: 0, max: 31 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 23, max: 27 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: 27, max: 29 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: 21, max: 25 });
        ids.sort();

        assert_eq!(ids, [0].to_vec());
    }

    #[test]
    fn test_21() {
        let mut scan_list = LineSpace::new(2, LineRange { min: 0, max: 31 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 3, max: 18 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: 3, max: 20 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: 3, max: 18 });
        ids.sort();

        assert_eq!(ids, [0, 1].to_vec());
    }

    #[test]
    fn test_22() {
        let mut scan_list = LineSpace::new(2, LineRange { min: 0, max: 31 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 0, max: 14 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: 21, max: 25 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: 17, max: 20 });
        ids.sort();

        assert_eq!(ids, [].to_vec());
    }

    #[test]
    fn test_23() {
        let mut scan_list = LineSpace::new(2, LineRange { min: 0, max: 31 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 11, max: 15 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: 16, max: 27 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: 5, max: 19 });
        ids.sort();

        assert_eq!(ids, [0, 1].to_vec());
    }

    #[test]
    fn test_24() {
        let mut scan_list = LineSpace::new(2, LineRange { min: 21, max: 26 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 21, max: 26 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: 25, max: 26 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: 21, max: 26 });
        ids.sort();

        assert_eq!(ids, [0, 1].to_vec());
    }

    #[test]
    fn test_25() {
        let mut scan_list = LineSpace::new(2, LineRange { min: 4, max: 19 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 4, max: 18 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: 10, max: 19 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: 13, max: 19 });
        ids.sort();

        assert_eq!(ids, [0, 1].to_vec());
    }

    #[test]
    fn test_26() {
        let mut scan_list = LineSpace::new(2, LineRange { min: 0, max: 151 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 83, max: 151 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: 0, max: 49 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: 49, max: 123 });
        ids.sort();

        assert_eq!(ids, [0, 1].to_vec());
    }

    #[test]
    fn test_27() {
        let mut scan_list = LineSpace::new(2, LineRange { min: -65, max: 86 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: 18, max: 86 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: -65, max: -16 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: -16, max: 58 });
        ids.sort();

        assert_eq!(ids, [0, 1].to_vec());
    }

    #[test]
    fn test_28() {
        let mut scan_list = LineSpace::new(2, LineRange { min: -54, max: 17 });
        scan_list.insert(LineSegment { id: 0, range: LineRange { min: -35, max: 17 } });
        scan_list.insert(LineSegment { id: 1, range: LineRange { min: -54, max: -29 } });

        let mut ids = scan_list.all_ids_in_range(LineRange { min: -39, max: 30 });
        ids.sort();

        assert_eq!(ids, [0, 1].to_vec());
    }

    #[test]
    fn test_single_random() {
        let min = -10;
        let max = 10;

        let segments = random_segments(min, max, 2);

        let real_min = segments.iter().map(|s| s.range.min).min().unwrap_or(0);
        let real_max = segments.iter().map(|s| s.range.max).max().unwrap_or(0);

        for level in 2..20 {
            let mut scan_list = LineSpace::new(level, LineRange { min: real_min, max: real_max });
            for segment in segments.iter() {
                scan_list.insert(segment.clone());
            }

            for _ in 0..10_000 {
                let range = random_range(real_min, real_max);

                let mut ids_a: Vec<_> = segments.iter().filter(|s| s.range.is_overlap(range)).map(|s| s.id).collect();
                let mut ids_b = scan_list.all_ids_in_range(range);
                ids_a.sort();
                ids_b.sort();

                assert_eq!(ids_a, ids_b);
            }
        }
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let min = -rng.gen_range(10..100);
            let max = rng.gen_range(10..100);

            let segments = random_segments(min, max, 2);

            let real_min = segments.iter().map(|s| s.range.min).min().unwrap_or(0);
            let real_max = segments.iter().map(|s| s.range.max).max().unwrap_or(0);

            for level in 2..20 {
                let mut scan_list = LineSpace::new(level, LineRange { min: real_min, max: real_max });
                for segment in segments.iter() {
                    scan_list.insert(segment.clone());
                }

                for _ in 0..100 {
                    let range = random_range(real_min, real_max);

                    let mut ids_a: Vec<_> = segments.iter().filter(|s| s.range.is_overlap(range)).map(|s| s.id).collect();
                    let mut ids_b = scan_list.all_ids_in_range(range);
                    ids_a.sort();
                    ids_b.sort();

                    assert_eq!(ids_a, ids_b);
                }
            }
        }
    }

    fn random_segments(min: i32, max: i32, count: usize) -> Vec<LineSegment<usize>> {
        let mut result = Vec::with_capacity(count);
        for id in 0..count {
            let range = random_range(min, max);
            result.push(LineSegment { id, range });
        }

        result
    }

    fn random_range(min: i32, max: i32) -> LineRange {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(min..max);
        let b = rng.gen_range(min..max);
        if a == b {
            LineRange { min: a, max: b + 1 }
        } else if a < b {
            LineRange { min: a, max: b }
        } else {
            LineRange { min: b, max: a }
        }
    }
}