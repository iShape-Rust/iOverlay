use super::{ArcDirection, ArcMath, ArcOptions, FloatArc, IntegerArc};
use alloc::vec::Vec;
use core::f64::consts::TAU;
use i_float::float::number::FloatNumber;
use i_float::int::angle::Angle;
use i_float::int::number::{int::IntNumber, wide_int::WideIntNumber};
use i_float::int::{unit_vector::UnitIntVector, vector::IntVector};

fn unit<I: IntNumber>(x: i32, y: i32) -> UnitIntVector<I> {
    let coordinate = |value: i32| {
        let magnitude = I::Wide::from_u32(value.unsigned_abs());
        if value < 0 { -magnitude } else { magnitude }
    };
    IntVector::<I>::new(coordinate(x), coordinate(y))
        .fast_normalize()
        .unwrap()
}

fn angle<I: IntNumber>(ray: UnitIntVector<I>) -> f64 {
    FloatNumber::atan2(ray.y().to_f64(), ray.x().to_f64())
}

// Both implementations must satisfy the same traversal and gap guarantees.
// Their point counts, coordinates, and length error are intentionally not compared.
fn check_contract<I: IntNumber, A: ArcMath<UnitIntVector<I>>>() {
    let rays: Vec<_> = [
        (1, 0),
        (10000, 1),
        (3, 4),
        (1, 1),
        (0, 1),
        (-1, 10000),
        (-4, 3),
        (-1, 0),
        (-10000, -1),
        (-3, -4),
        (0, -1),
        (10000, -1),
    ]
    .into_iter()
    .map(|(x, y)| unit::<I>(x, y))
    .collect();

    for max_step in [0, 1 << 26, u32::MAX] {
        for rotation_precision in [0, 5, u32::MAX] {
            let options = ArcOptions {
                max_step: Angle::from_bits(max_step),
                rotation_precision,
            };
            let max_gap = options.clamped().max_step.bits() as f64 * TAU / 4294967296.0;
            let mut arc = A::new(options);
            for &from in &rays {
                for &to in &rays {
                    for direction in [ArcDirection::Clockwise, ArcDirection::Counterclockwise] {
                        let sign = match direction {
                            ArcDirection::Clockwise => -1.0,
                            ArcDirection::Counterclockwise => 1.0,
                        };
                        let progress = |ray| {
                            let delta = sign * (angle(ray) - angle(from));
                            if delta < 0.0 { delta + TAU } else { delta }
                        };
                        let total = progress(to);
                        let output = arc.build(from, to, direction);
                        if total == 0.0 {
                            assert!(output.is_empty());
                            continue;
                        }
                        let mut previous = 0.0;
                        for &ray in output {
                            let current = progress(ray);
                            assert!(current > previous && current < total);
                            assert!(current - previous <= max_gap + 1e-12);
                            previous = current;
                        }
                        assert!(total - previous <= max_gap + 1e-12);
                    }
                }
            }

            let from = unit::<I>(1, 0);
            let direction = ArcDirection::Counterclockwise;
            assert!(!arc.build(from, unit(-1, 0), direction).is_empty());
            assert!(arc.build(from, from, direction).is_empty());
            assert!(!arc.build(from, unit(-1, 0), direction).is_empty());
            assert!(arc.build(from, unit(10000, 1), direction).is_empty());
        }
    }
}

#[test]
fn integer_contract_i16() {
    check_contract::<i16, IntegerArc<i16>>();
}

#[test]
fn integer_contract_i32() {
    check_contract::<i32, IntegerArc<i32>>();
}

#[test]
fn integer_contract_i64() {
    check_contract::<i64, IntegerArc<i64>>();
}

#[test]
fn float_contract_i16() {
    check_contract::<i16, FloatArc<i16>>();
}

#[test]
fn float_contract_i32() {
    check_contract::<i32, FloatArc<i32>>();
}

#[test]
fn float_contract_i64() {
    check_contract::<i64, FloatArc<i64>>();
}
