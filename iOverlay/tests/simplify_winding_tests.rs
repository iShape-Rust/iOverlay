use i_float::int::{number::int::IntNumber, point::IntPoint};
use i_overlay::core::{
    fill_rule::FillRule,
    integer::OverlayInt,
    overlay::{ContourDirection, IntOverlayOptions, Overlay},
    overlay_rule::OverlayRule,
    simplify::Simplify,
};
use i_shape::{
    flat::buffer::FlatContoursBuffer,
    int::{path::ContourExtension, shape::IntShapes},
};

// Preserve orientation; only ignore which vertex starts a closed contour.
fn canonical<I: IntNumber>(mut shapes: IntShapes<I>) -> IntShapes<I> {
    for contour in shapes.iter_mut().flatten() {
        let first = contour.iter().enumerate().min_by_key(|(_, p)| **p).unwrap().0;
        contour.rotate_left(first);
    }
    shapes
}

fn check_fill_and_output_direction<I: OverlayInt + From<i16> + core::fmt::Debug>() {
    let ccw = [(0_i16, 0_i16), (100, 0), (100, 100), (0, 100)]
        .map(|(x, y)| IntPoint::new(I::from(x), I::from(y)))
        .to_vec();
    for input_clockwise in [false, true] {
        let mut contour = ccw.clone();
        if input_clockwise {
            contour.reverse();
        }
        let shape = vec![contour.clone()];
        let shapes = vec![shape.clone()];
        for rule in [
            FillRule::EvenOdd,
            FillRule::NonZero,
            FillRule::Positive,
            FillRule::Negative,
        ] {
            let filled = match rule {
                FillRule::EvenOdd | FillRule::NonZero => true,
                FillRule::Positive => !input_clockwise,
                FillRule::Negative => input_clockwise,
            };
            for direction in [ContourDirection::CounterClockwise, ContourDirection::Clockwise] {
                let options = IntOverlayOptions {
                    output_direction: direction,
                    ..Default::default()
                };
                let expected = canonical(
                    Overlay::from_subj_custom(&shapes, options, Default::default())
                        .overlay(OverlayRule::Subject, rule),
                );
                assert_eq!(expected.len(), usize::from(filled));
                if filled {
                    assert_eq!(
                        expected[0][0].is_clockwise_ordered(),
                        direction == ContourDirection::Clockwise
                    );
                }
                let check = |actual, entry| {
                    assert_eq!(
                        canonical(actual),
                        expected,
                        "entry={entry}, input_clockwise={input_clockwise}, rule={rule:?}, output={direction:?}"
                    );
                };

                // The same single-contour fast path is reachable through all resource levels.
                check(contour.simplify(rule, options), "contour resource");
                check(shape.simplify(rule, options), "shape resource");
                check(shapes.simplify(rule, options), "shapes resource");
                let mut flat = FlatContoursBuffer::default();
                flat.set_with_contour(&contour);
                check(flat.simplify(rule, options), "flat resource");

                let mut overlay = Overlay::new_custom(4, options, Default::default());
                let result = overlay.simplify_contour(&contour, rule);
                assert_eq!(
                    result.is_none(),
                    filled && input_clockwise == (direction == ContourDirection::Clockwise),
                    "None must mean that the input contour needs no changes"
                );
                check(result.unwrap_or_else(|| shapes.clone()), "simplify_contour");
                check(
                    overlay
                        .simplify_shape(&shape, rule)
                        .unwrap_or_else(|| shapes.clone()),
                    "simplify_shape",
                );
                check(overlay.simplify_source(&shapes, rule), "simplify_source");
                overlay.simplify_flat_buffer(&mut flat, rule);
                let actual = if flat.is_empty() {
                    vec![]
                } else {
                    vec![flat.to_contours()]
                };
                check(actual, "simplify_flat_buffer");
            }
        }
    }
}

#[test]
fn simple_contour_winding_i16() {
    check_fill_and_output_direction::<i16>();
}

#[test]
fn simple_contour_winding_i32() {
    check_fill_and_output_direction::<i32>();
}

#[test]
fn simple_contour_winding_i64() {
    check_fill_and_output_direction::<i64>();
}
