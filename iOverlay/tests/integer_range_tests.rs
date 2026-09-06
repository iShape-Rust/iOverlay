use i_float::int::number::wide_int::WideIntNumber;
use i_float::int::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::integer::OverlayInt;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_shape::int::area::Area;

type Contour = Vec<[i64; 2]>;
type Shapes = Vec<Vec<Contour>>;

fn contour<I: OverlayInt + TryFrom<i64> + Into<i64>>(points: &[[i64; 2]]) -> Vec<IntPoint<I>> {
    points
        .iter()
        .map(|p| {
            IntPoint::new(
                I::try_from(p[0]).ok().expect("coordinate fits engine"),
                I::try_from(p[1]).ok().expect("coordinate fits engine"),
            )
        })
        .collect()
}

// Preserve winding and hole ownership, but ignore the starting vertex and shape order.
fn canonical(mut shapes: Shapes) -> Shapes {
    for shape in &mut shapes {
        for path in shape.iter_mut() {
            let start = path.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
            path.rotate_left(start);
        }
        shape[1..].sort();
    }
    shapes.sort();
    shapes
}

fn check<I: OverlayInt + TryFrom<i64> + Into<i64>>(
    subj: &[Contour],
    clip: &[Contour],
    rule: OverlayRule,
    expected: Shapes,
) {
    let subj: Vec<_> = subj.iter().map(|p| contour::<I>(p)).collect();
    let clip: Vec<_> = clip.iter().map(|p| contour::<I>(p)).collect();
    let expected = canonical(expected);
    for solver in [Solver::LIST, Solver::TREE, Solver::FRAG, Solver::AUTO] {
        let result = Overlay::<I>::with_contours_custom(&subj, &clip, Default::default(), solver)
            .overlay(rule, FillRule::EvenOdd);
        // Also exercise the wide area accumulator at the maximum square size.
        for shape in &result {
            assert!(shape[0].area_two() > I::Wide::ZERO);
            for hole in &shape[1..] {
                assert!(hole.area_two() < I::Wide::ZERO);
            }
        }
        let actual = result
            .iter()
            .map(|s| {
                s.iter()
                    .map(|c| c.iter().map(|p| [p.x.into(), p.y.into()]).collect())
                    .collect()
            })
            .collect();
        assert_eq!(
            canonical(actual),
            expected,
            "{} {:?}",
            core::any::type_name::<I>(),
            solver.strategy
        );
    }
}

fn square(lo: i64, hi: i64) -> Contour {
    vec![[lo, lo], [hi, lo], [hi, hi], [lo, hi]]
}

fn boundaries<I: OverlayInt + TryFrom<i64> + Into<i64>>() {
    let half = 1_i64 << (I::BITS - 2);
    let lo = -half;
    let hi = half - 1;
    let outer = square(lo, hi);
    check::<I>(
        &[outer.clone()],
        &[],
        OverlayRule::Subject,
        vec![vec![outer.clone()]],
    );

    // Keep unit-size features at both inclusive endpoints.
    for a in [lo, hi - 1] {
        let small = square(a, a + 1);
        check::<I>(&[small.clone()], &[], OverlayRule::Subject, vec![vec![small]]);
    }

    let inner = square(-half / 2, half / 2);
    let mut hole = inner.clone();
    hole.reverse();
    check::<I>(
        &[outer.clone()],
        &[inner],
        OverlayRule::Difference,
        vec![vec![outer, hole]],
    );

    // Issue #88's steep edge, stretched to the maximum supported span.
    // Reflect and transpose it to exercise increasing/decreasing fragments and both axes.
    for transpose in [false, true] {
        for reflect in [false, true] {
            let mut triangle = vec![[0, lo], [129, hi], [0, hi]];
            for p in &mut triangle {
                if reflect {
                    p[1] = -1 - p[1];
                }
                if transpose {
                    p.swap(0, 1);
                }
            }
            if reflect != transpose {
                triangle.reverse();
            }
            check::<I>(
                &[triangle.clone()],
                &[],
                OverlayRule::Subject,
                vec![vec![triangle]],
            );
        }
    }

    // An exact diagonal intersection near both limits, with a non-axis-aligned divider.
    let m = hi;
    let bow_tie = vec![[-m, -m], [m, m], [-m, m], [m, -m]];
    check::<I>(
        &[bow_tie],
        &[],
        OverlayRule::Subject,
        vec![
            vec![vec![[-m, -m], [m, -m], [0, 0]]],
            vec![vec![[-m, m], [0, 0], [m, m]]],
        ],
    );

    // An odd span puts the crossing at (-0.5, -0.5). Rounding to (0, 0)
    // exercises squared distances and the subsequent segment repair pass.
    let bow_tie = vec![[lo, lo], [hi, hi], [lo, hi], [hi, lo]];
    check::<I>(
        &[bow_tie],
        &[],
        OverlayRule::Subject,
        vec![
            vec![vec![[lo, lo], [hi, lo], [0, 0]]],
            vec![vec![[lo, hi], [0, 0], [hi, hi]]],
        ],
    );

    // Large collinear overlaps and intersections on fragment boundaries.
    let left = vec![[lo, lo], [0, lo], [0, hi], [lo, hi]];
    let bottom = vec![[lo, lo], [hi, lo], [hi, 0], [lo, 0]];
    check::<I>(
        &[left],
        &[bottom],
        OverlayRule::Intersect,
        vec![vec![square(lo, 0)]],
    );
}

#[test]
fn i16_boundaries() {
    boundaries::<i16>();
}

#[test]
fn i32_boundaries() {
    boundaries::<i32>();
}

#[test]
fn i64_boundaries() {
    boundaries::<i64>();
}

#[test]
fn issue_88_with_a_wider_engine_or_rescaled_coordinates() {
    let triangle = vec![[0, 0], [129, -23169], [0, 9854]];
    check::<i32>(
        &[triangle.clone()],
        &[],
        OverlayRule::Subject,
        vec![vec![triangle.clone()]],
    );
    let scaled: Contour = triangle.iter().map(|p| [p[0] / 2, p[1] / 2]).collect();
    check::<i16>(&[scaled.clone()], &[], OverlayRule::Subject, vec![vec![scaled]]);
}

#[test]
fn one_more_unit_of_span_exceeds_the_arithmetic_budget() {
    // These checked calculations test the limit without relying on debug-only panics
    // or promising any particular behavior for unsupported overlay inputs.
    macro_rules! check_budget {
        ($int:ty, $wide:ty) => {{
            let half: $int = 1 << (<$int>::BITS - 2);
            let span = (half - 1).checked_sub(-half).unwrap();
            assert_eq!(span, <$int>::MAX);
            let span = span as $wide;
            assert!(span.checked_mul(span).unwrap().checked_mul(2).is_some());
            assert!(half.checked_sub(-half).is_none());
            let too_wide = span + 1;
            assert!(too_wide.checked_mul(too_wide).unwrap().checked_mul(2).is_none());
        }};
    }
    check_budget!(i16, i32);
    check_budget!(i32, i64);
    check_budget!(i64, i128);
}

#[test]
fn conservative_float_scale_keeps_rounded_i16_span_in_range() {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;

    let half_extent = 1.99999;
    let rect = FloatRect::new(-half_extent, half_extent, -half_extent, half_extent);
    let min = [-half_extent, -half_extent];
    let max = [half_extent, half_extent];

    // Truncating log2(1.99999) to 0 selects scale 8192.
    // 1.99999 * 8192 = 16383.91808 rounds to 16384: the span exceeds i16::MAX.
    let old_scale = FloatPointAdapter::<[f64; 2], i16>::with_scale(rect, 8192.0);
    let old_min = old_scale.float_to_int(&min);
    let old_max = old_scale.float_to_int(&max);
    assert_eq!((old_min.x, old_max.x), (-16384, 16384));
    assert_eq!(i32::from(old_max.x) - i32::from(old_min.x), 32768);
    assert_eq!(old_max.x.checked_sub(old_min.x), None);

    let conservative = FloatPointAdapter::<[f64; 2], i16>::with_coordinate_bits(rect, i16::BITS - 3);
    let safe_min = conservative.float_to_int(&min);
    let safe_max = conservative.float_to_int(&max);
    assert_eq!((safe_min.x, safe_max.x), (-8192, 8192));
    assert_eq!(safe_max.x.checked_sub(safe_min.x), Some(16384));
}

#[test]
fn explicit_float_coordinate_budget() {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::float::overlay::FloatOverlay;

    fn check_adapter<I: OverlayInt + TryFrom<i64> + Into<i64>>() {
        let limit = 1_i64 << (I::BITS - 3);
        // Power-of-two, non-power-of-two, and sub-unit bounds exercise scale rounding.
        for half_extent in [1.0, 1.5, 0.25] {
            let rect = FloatRect::new(-half_extent, half_extent, -half_extent, half_extent);
            let adapter = FloatPointAdapter::<[f64; 2], I>::with_coordinate_bits(rect, I::BITS - 3);
            let points = vec![
                [-half_extent, -half_extent],
                [half_extent, -half_extent],
                [half_extent, half_extent],
                [-half_extent, half_extent],
            ];
            for point in &points {
                let p = adapter.float_to_int(point);
                assert!((-limit..=limit).contains(&p.x.into()));
                assert!((-limit..=limit).contains(&p.y.into()));
            }
            if half_extent != 1.5 {
                assert_eq!(adapter.float_to_int(&points[0]).x.into(), -limit);
                assert_eq!(adapter.float_to_int(&points[2]).x.into(), limit);
            }
            for solver in [Solver::LIST, Solver::TREE, Solver::FRAG, Solver::AUTO] {
                let result = FloatOverlay::new_custom(adapter.clone(), Default::default(), solver, 4)
                    .unsafe_add_source(&points, ShapeType::Subject)
                    .overlay(OverlayRule::Subject, FillRule::EvenOdd);
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].len(), 1);
                let mut actual = result[0][0].clone();
                let mut expected = points.clone();
                actual.sort_by(|a, b| a.partial_cmp(b).unwrap());
                expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
                assert_eq!(actual, expected);
            }
        }
    }
    check_adapter::<i16>();
    check_adapter::<i32>();
    check_adapter::<i64>();
}

#[test]
fn spiral_vector_area_at_coordinate_limits() {
    fn check_spiral<I: OverlayInt + TryFrom<i64> + Into<i64>>() {
        let lo = -(1_i64 << (I::BITS - 2));
        let hi = -lo - 1;
        // A simple, one-unit-wide spiral. Its shoelace partial sums encompass
        // several full squares before the return path cancels them out.
        let mut points = vec![
            [lo, lo],
            [hi, lo],
            [hi, hi],
            [lo, hi],
            [lo, lo + 4],
            [hi - 4, lo + 4],
            [hi - 4, hi - 4],
            [lo + 4, hi - 4],
            [lo + 4, lo + 8],
            [hi - 8, lo + 8],
        ];
        points.extend([
            [hi - 8, lo + 9],
            [lo + 5, lo + 9],
            [lo + 5, hi - 5],
            [hi - 5, hi - 5],
            [hi - 5, lo + 5],
            [lo + 1, lo + 5],
            [lo + 1, hi - 1],
            [hi - 1, hi - 1],
            [hi - 1, lo + 1],
            [lo, lo + 1],
        ]);
        let input = vec![contour::<I>(&points)];
        for solver in [Solver::LIST, Solver::TREE, Solver::FRAG, Solver::AUTO] {
            let mut overlay = Overlay::<I>::with_contours_custom(&input, &[], Default::default(), solver);
            // Sum the nine rectangular runs of the corridor, subtracting their
            // corner overlaps. Check filtering at the exact area and one above.
            let area = I::MAX.to_wide() * I::Wide::from_u32(9) - I::Wide::from_u32(56);
            overlay.options.min_output_area = area.to_uint();
            let vectors = overlay.build_shape_vectors(FillRule::EvenOdd, OverlayRule::Subject);
            let actual = vectors
                .iter()
                .map(|s| {
                    s.iter()
                        .map(|c| c.iter().map(|e| [e.a.x.into(), e.a.y.into()]).collect())
                        .collect()
                })
                .collect();
            assert_eq!(canonical(actual), canonical(vec![vec![points.clone()]]));
            overlay.options.min_output_area = (area + I::Wide::ONE).to_uint();
            assert!(
                overlay
                    .build_shape_vectors(FillRule::EvenOdd, OverlayRule::Subject)
                    .is_empty()
            );
        }
    }
    check_spiral::<i16>();
    check_spiral::<i32>();
    check_spiral::<i64>();
}

#[test]
fn fragment_radius_at_coordinate_limits() {
    use i_overlay::core::solver::Precision;
    fn check_radius<I: OverlayInt + TryFrom<i64> + Into<i64>>() {
        let hi = (1_i64 << (I::BITS - 2)) - 1;
        for lo in [-hi, -hi - 1] {
            let input = vec![contour::<I>(&[[lo, lo], [hi, hi], [lo, hi], [hi, lo]])];
            // Symmetric bounds give an exact crossing; the odd span gives a rounded
            // crossing and a subsequent repair pass. Check the squared-radius cap
            // and exponent saturation without narrowing the threshold to I.
            let cap = (2 * (I::BITS - 4)) as usize;
            for start in [cap - 1, cap, cap + 1, usize::MAX] {
                let solver = Solver {
                    precision: Precision {
                        start,
                        progression: 1,
                    },
                    ..Solver::FRAG
                };
                let actual = Overlay::<I>::with_contours_custom(&input, &[], Default::default(), solver)
                    .overlay(OverlayRule::Subject, FillRule::EvenOdd);
                let actual = actual
                    .iter()
                    .map(|s| {
                        s.iter()
                            .map(|c| c.iter().map(|p| [p.x.into(), p.y.into()]).collect())
                            .collect()
                    })
                    .collect();
                let expected = vec![
                    vec![vec![[lo, lo], [hi, lo], [0, 0]]],
                    vec![vec![[lo, hi], [0, 0], [hi, hi]]],
                ];
                assert_eq!(canonical(actual), canonical(expected));
            }
        }
    }
    check_radius::<i16>();
    check_radius::<i32>();
    check_radius::<i64>();
}

#[test]
fn seeded_boundary_overlays_match_a_wider_engine() {
    use rand::{RngExt, SeedableRng, rngs::StdRng};

    fn check_engine<I: OverlayInt + TryFrom<i64> + Into<i64>>() {
        let mut rng = StdRng::seed_from_u64(88);
        let lo = -(1_i64 << (I::BITS - 2));
        let hi = -lo - 1;
        let extremes = [lo, lo + 1, lo + 2, -1, 0, 1, hi - 1, hi];
        for case in 0..256 {
            let mut coordinate = || {
                if rng.random_bool(0.75) {
                    extremes[rng.random_range(0..extremes.len())]
                } else {
                    rng.random_range(lo..=hi)
                }
            };
            let subject: Contour = (0..6).map(|_| [coordinate(), coordinate()]).collect();
            let clip: Contour = (0..6).map(|_| [coordinate(), coordinate()]).collect();
            let rules = [
                OverlayRule::Subject,
                OverlayRule::Union,
                OverlayRule::Intersect,
                OverlayRule::Difference,
                OverlayRule::Xor,
            ];
            let rule = rules[case % rules.len()];
            for solver in [Solver::LIST, Solver::TREE, Solver::FRAG, Solver::AUTO] {
                let wide = Overlay::<i64>::with_contours_custom(
                    &[contour::<i64>(&subject)],
                    &[contour::<i64>(&clip)],
                    Default::default(),
                    solver,
                )
                .overlay(rule, FillRule::EvenOdd);
                let narrow = Overlay::<I>::with_contours_custom(
                    &[contour::<I>(&subject)],
                    &[contour::<I>(&clip)],
                    Default::default(),
                    solver,
                )
                .overlay(rule, FillRule::EvenOdd);
                let expected = wide
                    .iter()
                    .map(|s| s.iter().map(|c| c.iter().map(|p| [p.x, p.y]).collect()).collect())
                    .collect();
                let actual = narrow
                    .iter()
                    .map(|s| {
                        s.iter()
                            .map(|c| c.iter().map(|p| [p.x.into(), p.y.into()]).collect())
                            .collect()
                    })
                    .collect();
                assert_eq!(
                    canonical(actual),
                    canonical(expected),
                    "{} case {case} {:?}; subject={subject:?}, clip={clip:?}",
                    core::any::type_name::<I>(),
                    solver.strategy
                );
            }
        }
    }
    check_engine::<i16>();
    check_engine::<i32>();
}
