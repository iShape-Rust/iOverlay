use i_float::int::angle::Angle;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::unit_vector::UnitIntVector;
use i_float::int::vector::IntVector;
use i_overlay::mesh::int::arc::{ArcBuilder, ArcDirection, ArcOptions};
use std::f64::consts::TAU;

fn unit<I: IntNumber>(x: i32, y: i32) -> UnitIntVector<I> {
    let coordinate = |v: i32| {
        let magnitude = I::Wide::from_u32(v.unsigned_abs());
        if v < 0 { -magnitude } else { magnitude }
    };
    IntVector::<I>::new(coordinate(x), coordinate(y))
        .fast_normalize()
        .unwrap()
}

fn angle<I: IntNumber>(v: UnitIntVector<I>) -> f64 {
    v.y().to_f64().atan2(v.x().to_f64())
}

fn check_arc<I: IntNumber>(
    builder: &mut ArcBuilder<I>,
    from: UnitIntVector<I>,
    to: UnitIntVector<I>,
    direction: ArcDirection,
) {
    let sign = if direction == ArcDirection::Counterclockwise {
        1.0
    } else {
        -1.0
    };
    let expected = (sign * (angle(to) - angle(from))).rem_euclid(TAU);
    let scale = UnitIntVector::<I>::DENOMINATOR.to_f64();
    let max_angle = builder.options().max_step.bits() as f64 * TAU / 4294967296.0;
    let result = builder.build(from, to, direction);
    assert!(result.len() < 1536);
    if expected == 0.0 {
        assert!(result.is_empty());
        return;
    }

    let mut previous = from;
    let mut total = 0.0;
    for current in result.iter().copied().chain(std::iter::once(to)) {
        let delta = (sign * (angle(current) - angle(previous))).rem_euclid(TAU);
        assert!(delta > 0.0, "duplicate direction");
        assert!(
            delta <= max_angle + 1e-12,
            "step {} exceeds {} for {}-bit vectors",
            delta.to_degrees(),
            max_angle.to_degrees(),
            I::BITS,
        );
        let x = current.x().to_f64() / scale;
        let y = current.y().to_f64() / scale;
        assert!((if I::BITS == 16 { 0.46 } else { 0.98 }..=1.000000000000001).contains(&(x * x + y * y)));
        total += delta;
        previous = current;
    }
    assert!((total - expected).abs() < 1e-10, "wrong directed arc");
}

fn check_type<I: IntNumber>() {
    // Axes, diagonals, uneven lengths, almost equal/opposite directions,
    // and both sides of quadrant boundaries exercise minor and major arcs.
    let pairs = [
        (1, 0),
        (10000, 1),
        (3, 4),
        (1, 1),
        (1, 10000),
        (0, 1),
        (-1, 10000),
        (-4, 3),
        (-1, 1),
        (-1, 0),
        (-10000, -1),
        (-3, -4),
        (-1, -1),
        (-1, -10000),
        (0, -1),
        (1, -10000),
        (4, -3),
        (1, -1),
        (10000, -1),
    ];
    let directions: Vec<_> = pairs.into_iter().map(|(x, y)| unit::<I>(x, y)).collect();
    for precision in [4, 5, 6, 32] {
        for max_step in [
            ArcOptions::MAX_STEP,
            ArcOptions::MIN_STEP,
            Angle::from_bits(1 << 26),
        ] {
            let mut builder = ArcBuilder::<I>::new(ArcOptions {
                max_step,
                rotation_precision: precision,
            });
            for &from in &directions {
                for &to in &directions {
                    for direction in [ArcDirection::Clockwise, ArcDirection::Counterclockwise] {
                        check_arc(&mut builder, from, to, direction);
                    }
                }
            }
        }
    }
}

#[test]
fn arcs_i16() {
    check_type::<i16>();
}

#[test]
fn arcs_i32() {
    check_type::<i32>();
}

#[test]
fn arcs_i64() {
    check_type::<i64>();
}

#[test]
fn settings_are_clamped_and_default_to_precision_five() {
    let defaults = ArcBuilder::<i32>::default().options();
    assert_eq!(defaults.max_step, ArcOptions::MAX_STEP);
    assert_eq!(defaults.rotation_precision, 5);
    let low = ArcBuilder::<i32>::new(ArcOptions {
        max_step: Angle::from_bits(0),
        rotation_precision: 0,
    })
    .options();
    assert_eq!(low.max_step, ArcOptions::MIN_STEP);
    assert_eq!(low.rotation_precision, 4);
    let high = ArcBuilder::<i32>::new(ArcOptions {
        max_step: Angle::from_bits(u32::MAX),
        rotation_precision: u32::MAX,
    })
    .options();
    assert_eq!(high.max_step, ArcOptions::MAX_STEP);
    assert_eq!(high.rotation_precision, 32);
}

#[test]
fn opposite_directions_select_the_requested_semicircle() {
    let mut builder = ArcBuilder::<i32>::default();
    let from = unit(1, 0);
    let to = unit(-1, 0);
    let upper = builder.build(from, to, ArcDirection::Counterclockwise);
    assert!(!upper.is_empty());
    assert!(upper.iter().all(|v| v.y() > 0));
    let lower = builder.build(from, to, ArcDirection::Clockwise);
    assert!(!lower.is_empty());
    assert!(lower.iter().all(|v| v.y() < 0));
}

#[test]
fn empty_and_short_arcs_clear_the_previous_result() {
    let mut builder = ArcBuilder::<i32>::default();
    let from = unit(1, 0);
    let turn = ArcDirection::Counterclockwise;
    assert!(!builder.build(from, unit(-1, 0), turn).is_empty());
    assert!(builder.build(from, from, turn).is_empty());
    assert!(builder.build(from, unit(10, 1), turn).is_empty());
    // Collinear inputs may have unequal approximate lengths.
    assert!(builder.build(unit(3, 4), unit(3000, 4000), turn).is_empty());
}

#[test]
fn gaps_around_integer_multiples_stay_within_limit() {
    for precision in [4, 5, 6] {
        for max_step in [
            ArcOptions::MIN_STEP,
            Angle::from_bits(1 << 26),
            ArcOptions::MAX_STEP,
        ] {
            let mut builder = ArcBuilder::<i32>::new(ArcOptions {
                max_step,
                rotation_precision: precision,
            });
            let step = max_step.bits() as f64 * TAU / 4294967296.0;
            for start in [0.0_f64, 0.137, 1.91, 4.21] {
                for multiple in [1.0, 2.0, 3.0, 7.0, 31.0, 1000.0] {
                    for delta in [-1e-7, 0.0, 1e-7] {
                        let sweep = multiple * step + delta;
                        if sweep >= TAU {
                            continue;
                        }
                        let make = |a: f64| {
                            unit::<i32>((a.cos() * 1e8).round() as i32, (a.sin() * 1e8).round() as i32)
                        };
                        for direction in [ArcDirection::Clockwise, ArcDirection::Counterclockwise] {
                            let sign = if direction == ArcDirection::Clockwise {
                                -1.0
                            } else {
                                1.0
                            };
                            check_arc(&mut builder, make(start), make(start + sign * sweep), direction);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn long_arcs_accumulate_less_than_one_coordinate_unit_of_rotation_error() {
    fn check<I: IntNumber>() {
        let scale = UnitIntVector::<I>::DENOMINATOR.to_f64();
        for precision in [4, 5, 6] {
            let mut builder = ArcBuilder::<I>::new(ArcOptions {
                max_step: ArcOptions::MIN_STEP,
                rotation_precision: precision,
            });
            for direction in [ArcDirection::Clockwise, ArcDirection::Counterclockwise] {
                let from = unit::<I>(3, 4);
                let end = unit::<I>(3001, 4000);
                let to = if direction == ArcDirection::Clockwise {
                    unit::<I>(3000, 4001)
                } else {
                    end
                };
                let points = builder.build(from, to, direction);
                assert!(points.len() > 1000);
                // Independently measure the first applied rotation, then compare
                // every output with its ideal multiple. Includes contraction,
                // coefficient error and application rounding, not initial normalization.
                let step = angle(points[0]) - angle(from);
                let length = (from.x().to_f64().hypot(from.y().to_f64())) / scale;
                for (i, v) in points.iter().enumerate() {
                    let expected_angle = angle(from) + (i + 1) as f64 * step;
                    let dx = v.x().to_f64() / scale - length * expected_angle.cos();
                    let dy = v.y().to_f64() / scale - length * expected_angle.sin();
                    for radius in [1024.0, 4096.0, 65536.0] {
                        assert!(
                            dx.hypot(dy) * radius < 1.0,
                            "accumulated error exceeds one unit for {} bits",
                            I::BITS
                        );
                    }
                }
            }
        }
    }
    check::<i32>();
    check::<i64>();
}
