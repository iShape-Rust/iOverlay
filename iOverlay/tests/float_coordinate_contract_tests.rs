use i_float::float::{number::FloatNumber, rect::FloatRectError};
use i_overlay::core::{fill_rule::FillRule, overlay_rule::OverlayRule};
use i_overlay::float::{overlay::FloatOverlay, relate::FloatPredicateOverlay, scale::FixedScaleOverlayError};
use i_overlay::mesh::float::{
    outline::offset::OutlineOffset,
    stroke::offset::StrokeOffset,
    style::{OutlineStyle, StrokeStyle},
    variable_stroke::{StrokeVertex, VariableStrokeStyle, offset::VariableStrokeOffset},
};
use i_shape::flat::float::FloatFlatContoursBuffer;

macro_rules! coordinate_contract {
    ($name:ident, $scalar:ty) => {
        #[test]
        fn $name() {
            let limit = <$scalar as FloatNumber>::MAX_COORDINATE;
            let square = [[-limit, -limit], [limit, -limit], [limit, limit], [-limit, limit]];
            let mut overlay = FloatOverlay::with_subj(&square);
            assert_eq!(overlay.overlay(OverlayRule::Subject, FillRule::NonZero).len(), 1);
            assert!(FloatOverlay::with_subj_and_clip_fixed_scale(&square, &square, 1.0 / limit).is_ok());

            let error = FixedScaleOverlayError::InvalidRect(FloatRectError::CoordinatesOutOfRange);
            for invalid in [limit * 2.0, -limit * 2.0, <$scalar>::NAN, <$scalar>::INFINITY] {
                // Exercise both initialization and expansion of variable-stroke bounds.
                for points in [[[invalid, 0.0], [0.0, 0.0]], [[0.0, 0.0], [invalid, 0.0]]] {
                    assert!(matches!(
                        FloatOverlay::with_subj_and_clip_fixed_scale(&points, &square, 1.0),
                        Err(e) if e == error
                    ));
                    assert_eq!(points.stroke_fixed_scale(StrokeStyle::new(1.0), false, 1.0), Err(error));
                    assert_eq!(points.outline_fixed_scale(&OutlineStyle::new(1.0), 1.0), Err(error));
                    let vertices = points.map(|p| StrokeVertex::new(p, 1.0));
                    assert_eq!(vertices.variable_stroke_fixed_scale(VariableStrokeStyle::new(), 1.0), Err(error));
                }
            }

            // Valid source coordinates can still yield unsupported padded bounds.
            let path = [[limit, 0.0], [limit, limit / 4.0]];
            assert_eq!(path.stroke_fixed_scale(StrokeStyle::new(limit), false, 1.0 / limit), Err(error));
            assert_eq!(path.outline_fixed_scale(&OutlineStyle::new(limit / 4.0), 1.0 / limit), Err(error));
            let vertices = path.map(|p| StrokeVertex::new(p, limit));
            assert_eq!(vertices.variable_stroke_fixed_scale(VariableStrokeStyle::new(), 1.0 / limit), Err(error));

            // Bounds errors must not clear a caller's existing output buffer.
            let mut output = FloatFlatContoursBuffer::default();
            output.add_contour(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
            let original = output.clone();
            assert_eq!(path.stroke_fixed_scale_into(StrokeStyle::new(limit), false, 1.0 / limit, &mut output), Err(error));
            assert_eq!(path.outline_fixed_scale_into(&OutlineStyle::new(limit / 4.0), 1.0 / limit, &mut output), Err(error));
            assert_eq!(vertices.variable_stroke_fixed_scale_into(VariableStrokeStyle::new(), 1.0 / limit, &mut output), Err(error));
            assert_eq!(output.points, original.points);
            assert_eq!(output.ranges, original.ranges);

            // Reciprocal overflow must be evaluated in the original scalar type.
            let tiny = <$scalar>::from_bits(1);
            assert_eq!(FixedScaleOverlayError::validate_scale(tiny), Err(FixedScaleOverlayError::ScaleTooSmall));
            for path in [vec![], vec![[0.0, 0.0], [1.0, 0.0]]] {
                assert!(matches!(FloatOverlay::with_subj_and_clip_fixed_scale(&path, &path, tiny), Err(FixedScaleOverlayError::ScaleTooSmall)));
                assert_eq!(path.stroke_fixed_scale(StrokeStyle::new(1.0), false, tiny), Err(FixedScaleOverlayError::ScaleTooSmall));
                assert_eq!(path.outline_fixed_scale(&OutlineStyle::new(1.0), tiny), Err(FixedScaleOverlayError::ScaleTooSmall));
                let vertices: Vec<_> = path.iter().map(|&p| StrokeVertex::new(p, 1.0)).collect();
                assert_eq!(vertices.variable_stroke_fixed_scale(VariableStrokeStyle::new(), tiny), Err(FixedScaleOverlayError::ScaleTooSmall));
            }
            let empty: Vec<[$scalar; 2]> = vec![];
            assert!(empty.stroke_fixed_scale(StrokeStyle::new(1.0), false, 1.0).unwrap().is_empty());
            assert!(empty.outline_fixed_scale(&OutlineStyle::new(1.0), 1.0).unwrap().is_empty());
            let empty_vertices: Vec<StrokeVertex<[$scalar; 2]>> = vec![];
            assert!(empty_vertices.variable_stroke_fixed_scale(VariableStrokeStyle::new(), 1.0).unwrap().is_empty());
        }
    };
}

coordinate_contract!(f32_coordinate_contract, f32);
coordinate_contract!(f64_coordinate_contract, f64);

#[test]
fn automatic_i16_budget_and_fixed_scale_rejection() {
    let h = 1.99999;
    let square = [[-h, -h], [h, -h], [h, h], [-h, h]];
    let mut overlay = FloatOverlay::<_, i16>::from_subj_and_clip(&square, &square);
    let result = overlay.overlay(OverlayRule::Intersect, FillRule::NonZero);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0][0].len(), 4);
    for p in &result[0][0] {
        assert!((p[0].abs() - 2.0).abs() < 0.001);
        assert!((p[1].abs() - 2.0).abs() < 0.001);
    }
    let mut predicate = FloatPredicateOverlay::<_, i16>::from_subj_and_clip(&square, &square);
    assert!(predicate.intersects());
    assert!(matches!(
        FloatOverlay::<_, i16>::from_subj_and_clip_fixed_scale(&square, &square, 8192.0),
        Err(FixedScaleOverlayError::ScaleTooLarge)
    ));
    assert!(FloatOverlay::<_, i16>::from_subj_and_clip_fixed_scale(&square, &square, 4096.0).is_ok());
}

#[test]
#[should_panic(expected = "Invalid adapter bounds")]
fn infallible_overlay_rejects_invalid_bounds() {
    let _ = FloatOverlay::with_subj(&[[f64::NAN, 0.0]]);
}

#[test]
#[should_panic(expected = "Invalid offset bounds")]
fn infallible_stroke_rejects_padded_bounds() {
    let limit = <f64 as FloatNumber>::MAX_COORDINATE;
    let _ = [[limit, 0.0], [limit, 1.0]].stroke(StrokeStyle::new(limit), false);
}
