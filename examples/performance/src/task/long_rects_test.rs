use i_float::fix_vec::FixVec;
use i_float::fix_float::FixFloat;
use i_shape::fix_path::FixPath;
use i_overlay::fill::shape_type::ShapeType;
use i_overlay::layout::overlay::Overlay;
use chrono::prelude::*;
use i_overlay::bool::fill_rule::FillRule;

// 0.2.0
// 3 GHz 6-Core Intel Core i5, 40 GB 2667 MHz DDR4

// 100 - 0
// 200 - 2
// 300 - 8
// 500 - 40, 39, 38

pub struct LongRectsTest;

impl LongRectsTest {
    pub fn run() {
        let n: usize = 500;

        let subj_paths = Self::long_rects(
            FixVec::ZERO,
            FixVec::new_i64(10, 20 * n as i64),
            FixVec::new_i64(20, 0),
            n,
        );

        let clip_paths = Self::long_rects(
            FixVec::ZERO,
            FixVec::new_i64(20 * n as i64, 10),
            FixVec::new_i64(0, 20),
            n,
        );

        let start = Utc::now();

        let mut overlay = Overlay::new(subj_paths.len() + clip_paths.len());
        overlay.add_paths(subj_paths, ShapeType::SUBJECT);
        overlay.add_paths(clip_paths, ShapeType::CLIP);

        let graph = overlay.build_graph();

        let clip = graph.extract_shapes_min_area(FillRule::Clip, FixFloat::ZERO);

        assert!(!clip.is_empty());

        let subject = graph.extract_shapes_min_area(FillRule::Subject, FixFloat::ZERO);
        assert!(!subject.is_empty());

        let difference = graph.extract_shapes_min_area(FillRule::Difference, FixFloat::ZERO);
        assert!(!difference.is_empty());

        let intersect = graph.extract_shapes_min_area(FillRule::Intersect, FixFloat::ZERO);
        assert!(!intersect.is_empty());

        let union = graph.extract_shapes_min_area(FillRule::Union, FixFloat::ZERO);
        assert!(!union.is_empty());

        let xor = graph.extract_shapes_min_area(FillRule::Xor, FixFloat::ZERO);
        assert!(!xor.is_empty());

        let end = Utc::now();

        let duration = end - start;
        println!("Spend time: {} seconds", duration.num_seconds());
    }

    fn long_rects(start: FixVec, size: FixVec, offset: FixVec, n: usize) -> Vec<FixPath> {
        let mut result = Vec::with_capacity(n);
        let mut p = start;
        for _ in 0..n {
            let path = [
                p,
                p + FixVec::new_i64(0, size.y.value()),
                p + size,
                p + FixVec::new_i64(size.x.value(), 0),
            ].to_vec();

            result.push(path);

            p = p + offset
        }

        result
    }
}