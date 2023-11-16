use i_float::fix_vec::FixVec;
use i_float::fix_float::FixFloat;
use i_shape::fix_path::FixPath;
use i_overlay::fill::shape_type::ShapeType;
use i_overlay::layout::overlay::Overlay;
use chrono::prelude::*;
use i_overlay::bool::fill_rule::FillRule;

pub struct ManySquaresTest;

// 0.2.0
// 3 GHz 6-Core Intel Core i5, 40 GB 2667 MHz DDR4

// 100 - 1
// 200 - 12
// 300 - 42, 40

impl ManySquaresTest {
    pub fn run() {
        let n: usize = 100;

        let subj_paths = Self::many_squares(
            FixVec::ZERO,
            20,
            30,
            n,
        );

        let clip_paths = Self::many_squares(
            FixVec::new_i64(15, 15),
            20,
            30,
            n - 1,
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

    fn many_squares(start: FixVec, a: i64, offset: i64, n: usize) -> Vec<FixPath> {
        let mut result = Vec::with_capacity(n * n);
        let mut y = start.y.value();

        for _ in 0..n {
            let mut x = start.x.value();
            for _ in 0..n {
                let path = [
                    FixVec::new_i64(x, y),
                    FixVec::new_i64(x, y + a),
                    FixVec::new_i64(x + a, y + a),
                    FixVec::new_i64(x + a, y)
                ].to_vec();
                result.push(path);
                x += offset;
            }
            y += offset;
        }

        result
    }
}