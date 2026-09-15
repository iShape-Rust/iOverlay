use i_overlay::mesh::float::stroke::offset::StrokeOffset;
use i_overlay::mesh::float::style::{LineJoin, StrokeStyle};
use i_overlay::mesh::math::MathMode;

fn canonical(shapes: Vec<Vec<Vec<[f64; 2]>>>) -> Vec<Vec<Vec<[i64; 2]>>> {
    let mut shapes: Vec<Vec<Vec<[i64; 2]>>> = shapes
        .into_iter()
        .map(|shape| {
            let mut shape: Vec<Vec<[i64; 2]>> = shape
                .into_iter()
                .map(|path| {
                    let mut path: Vec<_> = path
                        .into_iter()
                        .map(|p| [(p[0] * 1e6).round() as i64, (p[1] * 1e6).round() as i64])
                        .collect();
                    let start = path.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
                    path.rotate_left(start);
                    path
                })
                .collect();
            shape[1..].sort();
            shape
        })
        .collect();
    shapes.sort();
    shapes
}

#[test]
fn reversing_path_preserves_bevel_and_miter_strokes() {
    let mut seed = 0x5493_baa7_9215_7831_u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((seed >> 32) % 21) as f64 - 10.0
    };
    for case in 0..600 {
        let path: Vec<_> = (0..3 + case % 7).map(|_| [next(), next()]).collect();
        let mut reversed = path.clone();
        reversed.reverse();
        for closed in [false, true] {
            for math in [MathMode::Integer, MathMode::Float] {
                for join in [LineJoin::Bevel, LineJoin::Miter(0.2)] {
                    let style = StrokeStyle::new(1.25).math(math).line_join(join.clone());
                    let forward = path.stroke_fixed_scale(style.clone(), closed, 1000.0).unwrap();
                    let backward = reversed.stroke_fixed_scale(style, closed, 1000.0).unwrap();
                    assert_eq!(
                        canonical(forward),
                        canonical(backward),
                        "case={case}, math={math:?}, closed={closed}, join={join:?}, path={path:?}"
                    );
                }
            }
        }
    }
}
