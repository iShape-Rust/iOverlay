//! Manual comparison; run with release optimizations and an otherwise idle machine.
extern crate std;
use super::offset::build_stroke_overlay_iter;
use crate::core::{fill_rule::FillRule, integer::OverlayInt, overlay_rule::OverlayRule};
use crate::mesh::int::{
    arc::ArcOptions,
    math::{backend::MeshMath, float::FloatMath, integer::IntegerMath},
    style::{IntLineCap, IntLineJoin, IntStrokeStyle},
};
use crate::mesh::math::MathMode;
use alloc::vec::Vec;
use core::hint::black_box;
use i_float::float::number::FloatNumber;
use i_float::int::{angle::Angle, point::IntPoint};
use std::{println, time::Instant};

fn measure(mut run: impl FnMut() -> usize) -> (f64, usize) {
    let count = black_box(run());
    let mut samples = [0.0; 7];
    for sample in &mut samples {
        let start = Instant::now();
        for _ in 0..10 {
            black_box(run());
        }
        *sample = start.elapsed().as_secs_f64() * 100_000.0;
    }
    samples.sort_by(f64::total_cmp);
    (samples[3], count)
}

fn benchmark<I: OverlayInt>() {
    for jagged in [false, true] {
        let paths: Vec<Vec<IntPoint<I>>> = (0..16)
            .map(|row| {
                (0..96)
                    .map(|j| {
                        let x = j as f64 * 2048.0;
                        let y = row as f64 * 32768.0
                            + if jagged {
                                if j % 2 == 0 { 0.0 } else { 4096.0 }
                            } else {
                                FloatNumber::sin(j as f64 * 0.18) * 4096.0
                            };
                        IntPoint::new(I::from_rounded_float(x), I::from_rounded_float(y))
                    })
                    .collect()
            })
            .collect();
        let vectors: Vec<_> = paths
            .iter()
            .flat_map(|p| p.windows(2).map(|pair| pair[1] - pair[0]))
            .collect();
        for math in [MathMode::Integer, MathMode::Float] {
            let (us, _) = measure(|| {
                for &v in &vectors {
                    match math {
                        MathMode::Integer => {
                            black_box(<IntegerMath as MeshMath<I>>::normalize(black_box(v)));
                        }
                        MathMode::Float => {
                            black_box(<FloatMath as MeshMath<I>>::normalize(black_box(v)));
                        }
                    }
                }
                vectors.len()
            });
            println!("i{} jagged={jagged} normalize {math:?}: {us:.2} us", I::BITS);
        }
        for join in [
            IntLineJoin::Bevel,
            IntLineJoin::Miter(Angle::from_radians(0.1)),
            IntLineJoin::Round(ArcOptions::default()),
        ] {
            for math in [MathMode::Integer, MathMode::Float] {
                let style = IntStrokeStyle::new(I::from_rounded_float(512.0))
                    .math(math)
                    .line_join(join)
                    .start_cap(IntLineCap::Round(ArcOptions::default()))
                    .end_cap(IntLineCap::Square);
                for full in [false, true] {
                    let (us, count) = measure(|| {
                        let mut overlay = build_stroke_overlay_iter(
                            black_box(&paths).iter().map(|p| p.iter().copied()),
                            black_box(&style),
                            false,
                            Default::default(),
                        );
                        if full {
                            overlay
                                .overlay(OverlayRule::Subject, FillRule::Positive)
                                .iter()
                                .flatten()
                                .map(Vec::len)
                                .sum()
                        } else {
                            overlay.segments.len()
                        }
                    });
                    println!(
                        "i{} jagged={jagged} join={join:?} full={full} {math:?}: {us:.2} us, count={count}",
                        I::BITS
                    );
                }
            }
        }
    }
}

#[test]
#[ignore = "manual timing: cargo test --release --lib benchmark_math_modes -- --ignored --nocapture"]
fn benchmark_math_modes() {
    benchmark::<i32>();
    benchmark::<i64>();
}
