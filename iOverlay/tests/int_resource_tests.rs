// These tests intentionally compare the legacy constructors with the resource API.
#![allow(deprecated)]

use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::integer::OverlayInt;
use i_overlay::core::overlay::{ContourDirection, IntOverlayOptions, Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::relate::{IntRelate, PredicateOverlay};
use i_overlay::core::simplify::Simplify;
use i_overlay::core::single::SingleIntOverlay;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use i_overlay::i_shape::flat::buffer::{FlatContoursBuffer, FlatShapesBuffer};
use i_overlay::i_shape::int::shape::IntShapes;
use i_overlay::i_shape::source::int::resource::IntShapeResource;
use i_overlay::string::clip::{ClipRule, IntClip};
use i_overlay::string::overlay::StringOverlay;
use i_overlay::string::slice::IntSlice;

fn rect<I: OverlayInt>(x0: u32, y0: u32, x1: u32, y1: u32) -> [IntPoint<I>; 4] {
    [(x0, y0), (x1, y0), (x1, y1), (x0, y1)].map(|(x, y)| IntPoint::new(I::from_u32(x), I::from_u32(y)))
}

// A user-defined resource must work without collecting its paths into owned containers.
struct BorrowedContour<'a, I: OverlayInt>(&'a [IntPoint<I>]);

impl<I: OverlayInt> IntShapeResource<I> for BorrowedContour<'_, I> {
    type ResourceIter<'a>
        = core::iter::Once<&'a [IntPoint<I>]>
    where
        Self: 'a,
        I: 'a;

    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        core::iter::once(self.0)
    }
}

fn check_engines<I: OverlayInt + core::fmt::Debug>() {
    let subj = rect::<I>(0, 0, 10, 10);
    let clip = rect::<I>(5, 0, 15, 10);
    let contours = vec![subj.to_vec()];
    let shapes = vec![contours.clone()];
    let borrowed = [subj.as_slice()];
    let custom = BorrowedContour(subj.as_slice());
    let mut flat_contours = FlatContoursBuffer::default();
    flat_contours.set_with_shape(&contours);
    let mut flat_shapes = FlatShapesBuffer::default();
    flat_shapes.set_with_shapes(&vec![vec![clip.to_vec()]]);
    for rule in [
        OverlayRule::Union,
        OverlayRule::Intersect,
        OverlayRule::Difference,
        OverlayRule::Xor,
    ] {
        let expected = Overlay::with_contour(&subj, &clip).overlay(rule, FillRule::NonZero);
        assert_eq!(
            Overlay::from_subj_and_clip(&subj, &flat_shapes).overlay(rule, FillRule::NonZero),
            expected
        );
        assert_eq!(
            Overlay::from_subj_and_clip(&contours[..], &clip[..]).overlay(rule, FillRule::NonZero),
            expected
        );
        assert_eq!(
            Overlay::from_subj_and_clip(&shapes, &flat_shapes).overlay(rule, FillRule::NonZero),
            expected
        );
        assert_eq!(
            Overlay::from_subj_and_clip(&borrowed[..], &clip).overlay(rule, FillRule::NonZero),
            expected
        );
        assert_eq!(
            Overlay::from_subj_and_clip(&flat_contours, &flat_shapes).overlay(rule, FillRule::NonZero),
            expected
        );
        assert_eq!(custom.overlay(&clip[..], rule, FillRule::NonZero), expected);
        assert_eq!(subj[..].overlay(&flat_shapes, rule, FillRule::NonZero), expected);
    }
    assert!(custom.intersects(&flat_shapes));
    assert!(flat_contours.interiors_intersect(&clip[..]));
    assert!(!subj.touches(&clip));
    assert!(!subj.point_intersects(&clip));
    assert!(!subj.within(&clip));
    assert!(!subj.disjoint(&clip));
    assert!(subj.covers(&rect::<I>(2, 2, 8, 8)));
    let touch = rect::<I>(10, 10, 20, 20);
    assert!(subj.touches(&touch));
    assert!(subj.point_intersects(&touch));
    assert!(subj.disjoint(&rect::<I>(20, 20, 30, 30)));

    let expected = subj[..].simplify(FillRule::NonZero, Default::default());
    assert_eq!(custom.simplify(FillRule::NonZero, Default::default()), expected);
    assert_eq!(
        flat_contours.simplify(FillRule::NonZero, Default::default()),
        expected
    );
    assert_eq!(shapes.simplify(FillRule::NonZero, Default::default()), expected);
}

#[test]
fn resource_storage_and_integer_engines() {
    check_engines::<i16>();
    check_engines::<i32>();
    check_engines::<i64>();
}

#[test]
fn overlay_reinit_retains_options_and_replaces_both_operands() {
    let square = rect::<i32>(0, 0, 10, 10);
    let other = rect::<i32>(5, 0, 15, 10);
    let options = IntOverlayOptions {
        output_direction: ContourDirection::Clockwise,
        ..Default::default()
    };
    let mut overlay = Overlay::from_subj_and_clip_custom(&square, &other, options, Solver::LIST);
    overlay.overlay(OverlayRule::Union, FillRule::NonZero);
    overlay.reinit_with_subj(&square[..]);
    let expected = Overlay::from_subj_custom(&square, options, Solver::LIST)
        .overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(overlay.overlay(OverlayRule::Subject, FillRule::NonZero), expected);
    assert_eq!(overlay.options.output_direction, ContourDirection::Clockwise);
    assert_eq!(overlay.solver.strategy, Solver::LIST.strategy);
    overlay.reinit_with_subj_and_clip(&square, &square[..]);
    assert!(
        overlay
            .overlay(OverlayRule::Difference, FillRule::NonZero)
            .is_empty()
    );
    let empty: &[IntPoint] = &[];
    overlay.reinit_with_subj(empty);
    assert!(
        overlay
            .overlay(OverlayRule::Subject, FillRule::NonZero)
            .is_empty()
    );
    overlay.add_source(&square, ShapeType::Subject);
    assert_eq!(overlay.overlay(OverlayRule::Subject, FillRule::NonZero), expected);
}

#[test]
fn predicate_fill_rule_and_reinit() {
    let square = rect::<i32>(0, 0, 10, 10);
    let duplicate = [square.as_slice(), square.as_slice()];
    assert!(!PredicateOverlay::from_subj_and_clip(&duplicate, &square).interiors_intersect());
    let mut predicate =
        PredicateOverlay::from_subj_and_clip_custom(&duplicate, &square, FillRule::NonZero, Solver::LIST);
    assert!(predicate.interiors_intersect());
    predicate.reinit_with_subj_and_clip(&duplicate[..], &square[..]);
    assert!(predicate.interiors_intersect());
    let empty: &[IntPoint] = &[];
    predicate.reinit_with_subj_and_clip(empty, &square);
    assert!(!predicate.intersects());
    assert_eq!(predicate.solver.strategy, Solver::LIST.strategy);
}

#[test]
fn string_resources_preserve_open_and_closed_semantics() {
    let square = rect::<i32>(0, 0, 10, 10);
    let path = [IntPoint::new(2, 2), IntPoint::new(8, 2), IntPoint::new(8, 8)];
    let rule = ClipRule {
        invert: false,
        boundary_included: true,
    };
    let mut polygon = FlatShapesBuffer::default();
    polygon.set_with_contour(&square);
    let mut strings = FlatContoursBuffer::default();
    strings.set_with_contour(&path);
    let expected = square.clip_path(&path.to_vec(), FillRule::NonZero, rule);
    assert_eq!(polygon.clip_source(&strings, FillRule::NonZero, rule), expected);
    assert_eq!(
        StringOverlay::from_shape_and_string(&polygon, &path[..]).clip_string_lines(FillRule::NonZero, rule),
        expected
    );
    let mut closed = StringOverlay::from_shape(&polygon);
    closed.add_string_contour_source(&strings);
    let mut legacy = StringOverlay::with_shape_contour(&square);
    legacy.add_string_contour(&path);
    let closed_result = closed.clip_string_lines(FillRule::NonZero, rule);
    assert_eq!(closed_result, legacy.clip_string_lines(FillRule::NonZero, rule));
    assert_ne!(closed_result, expected);
}

#[test]
fn slices_and_clips_work_on_borrowed_and_flat_resources() {
    let square = rect::<i32>(0, 0, 10, 10);
    let line = [IntPoint::new(-5, 5), IntPoint::new(15, 5)];
    let borrowed = [square.as_slice()];
    let mut flat = FlatContoursBuffer::default();
    flat.set_with_contour(&square);
    let sliced = square.slice_by_line(line, FillRule::NonZero);
    assert_eq!(sliced.len(), 2);
    assert_eq!(borrowed[..].slice_by_source(&line, FillRule::NonZero), sliced);
    assert_eq!(flat.slice_by_source(&line[..], FillRule::NonZero), sliced);
    let rule = ClipRule {
        invert: false,
        boundary_included: false,
    };
    assert_eq!(
        flat.clip_source(&line[..], FillRule::NonZero, rule),
        vec![vec![IntPoint::new(0, 5), IntPoint::new(10, 5)]]
    );
}

#[test]
fn simplify_resource_handles_holes_empty_paths_and_area_filter() {
    let outer = rect::<i32>(0, 0, 20, 20);
    let mut hole = rect::<i32>(5, 5, 15, 15);
    hole.reverse();
    let shape = vec![outer.to_vec(), hole.to_vec()];
    let mut flat = FlatShapesBuffer::default();
    flat.set_with_shape(&shape);
    let expected = Overlay::with_contours(&shape, &[]).overlay(OverlayRule::Subject, FillRule::NonZero);
    assert_eq!(expected[0].len(), 2);
    assert_eq!(flat.simplify(FillRule::NonZero, Default::default()), expected);
    let paths = [outer.as_slice(), &[], hole.as_slice()];
    assert_eq!(
        paths[..].simplify(FillRule::NonZero, Default::default()),
        expected
    );
    let mut reusable = Overlay::from_subj(&outer);
    let empty: IntShapes<i32> = vec![];
    assert!(reusable.simplify_source(&empty, FillRule::NonZero).is_empty());
    let degenerate = [IntPoint::new(0, 0), IntPoint::new(1, 1)];
    assert!(
        degenerate
            .simplify(FillRule::NonZero, Default::default())
            .is_empty()
    );
    let options = IntOverlayOptions {
        min_output_area: 1_000,
        ..Default::default()
    };
    assert!(outer.simplify(FillRule::NonZero, options).is_empty());
    flat.set_with_contour(&outer);
    assert!(flat.simplify(FillRule::NonZero, options).is_empty());
}
