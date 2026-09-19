use core::cell::Cell;
use i_float::int::point::IntPoint;
use i_float::int::rect::IntRect;
use i_overlay::mesh::int::outline::offset::{IntOutlineError, IntOutlineOffset};
use i_overlay::mesh::int::style::{IntLineJoin, IntOutlineStyle};
use i_shape::source::int::resource::IntShapeResource;

macro_rules! check_bounds {
    ($name:ident, $int:ty) => {
        #[test]
        fn $name() {
            let limit: $int = 1 << (<$int>::BITS - 2);
            let radius: $int = 1024;
            let lo = -limit + radius + 1;
            let hi = limit - radius - 1;
            let source = |rect: IntRect<$int>| {
                [
                    IntPoint::new(rect.min_x, rect.min_y),
                    IntPoint::new(rect.max_x, rect.min_y),
                    IntPoint::new(rect.max_x, rect.max_y),
                    IntPoint::new(rect.min_x, rect.max_y),
                ]
            };
            let valid = source(IntRect::new(lo, hi, lo, hi));
            for style in [
                IntOutlineStyle::new(radius),
                IntOutlineStyle::new(-radius),
                IntOutlineStyle::new(0).outer_offset(radius),
                IntOutlineStyle::new(0).inner_offset(-radius),
            ] {
                assert_eq!(valid.validate_outline(&style), Ok(()));
                for rect in [
                    IntRect::new(lo - 1, hi, lo, hi),
                    IntRect::new(lo, hi + 1, lo, hi),
                    IntRect::new(lo, hi, lo - 1, hi),
                    IntRect::new(lo, hi, lo, hi + 1),
                ] {
                    assert_eq!(
                        source(rect).validate_outline(&style),
                        Err(IntOutlineError::CoordinateOutOfRange)
                    );
                }
            }
            // Full storage-range inputs and offsets must reject without wrapping,
            // including the magnitude of MIN, which cannot be represented in I.
            for coordinate in [<$int>::MIN, -limit, limit, <$int>::MAX] {
                let path = [IntPoint::new(coordinate, 0)];
                assert_eq!(
                    path.validate_outline(&IntOutlineStyle::new(0)),
                    Err(IntOutlineError::CoordinateOutOfRange)
                );
                assert_eq!(
                    path.validate_outline(&IntOutlineStyle::new(<$int>::MIN)),
                    Err(IntOutlineError::CoordinateOutOfRange)
                );
            }
            for offset in [<$int>::MIN, <$int>::MAX] {
                assert_eq!(
                    [IntPoint::<$int>::ZERO].validate_outline(&IntOutlineStyle::new(offset)),
                    Err(IntOutlineError::CoordinateOutOfRange)
                );
            }
        }
    };
}

check_bounds!(expanded_bounds_i16, i16);
check_bounds!(expanded_bounds_i32, i32);
check_bounds!(expanded_bounds_i64, i64);

#[test]
fn empty_input_needs_no_coordinate_space_for_any_join() {
    let empty: Vec<IntPoint> = vec![];
    assert_eq!(empty.validate_outline(&IntOutlineStyle::new(i32::MIN)), Ok(()));
    for join in [
        IntLineJoin::Miter(i_float::int::angle::Angle::from_bits(1 << 26)),
        IntLineJoin::Round(Default::default()),
    ] {
        assert_eq!(
            empty.validate_outline(&IntOutlineStyle::new(0).line_join(join)),
            Ok(())
        );
    }
}

struct CountedResource<'p> {
    path: &'p [IntPoint],
    traversals: Cell<usize>,
}

impl IntShapeResource<i32> for CountedResource<'_> {
    type ResourceIter<'a>
        = core::iter::Once<&'a [IntPoint]>
    where
        Self: 'a;

    fn iter_paths(&self) -> Self::ResourceIter<'_> {
        self.traversals.set(self.traversals.get() + 1);
        core::iter::once(self.path)
    }
}

#[test]
fn construction_does_not_repeat_the_bounds_pass() {
    let path = [
        IntPoint::new(0, 0),
        IntPoint::new(8192, 0),
        IntPoint::new(8192, 8192),
        IntPoint::new(0, 8192),
    ];
    let source = CountedResource {
        path: &path,
        traversals: Cell::new(0),
    };
    let style = IntOutlineStyle::new(1024);
    assert_eq!(source.validate_outline(&style), Ok(()));
    assert_eq!(source.traversals.get(), 1);
    source.traversals.set(0);
    assert_eq!(source.outline(&style).unwrap().len(), 1);
    assert_eq!(source.traversals.get(), 1);
}
