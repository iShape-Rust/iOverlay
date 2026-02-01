use crate::float::overlay::OverlayOptions;
use crate::float::scale::FixedScaleOverlayError;
use crate::i_shape::source::resource::ShapeResource;
use crate::mesh::overlay::OffsetOverlay;
use crate::mesh::stroke::builder::StrokeBuilder;
use crate::mesh::stroke::offset::vec::Vec;
use crate::mesh::style::StrokeStyle;
use alloc::vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::simple::SimplifyContour;

pub trait StrokeOffset<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Generates a stroke shapes for paths, contours, or shapes.
    ///
    /// - `style`: Defines the stroke properties, including width, line caps, and joins.
    /// - `is_closed_path`: Specifies whether the path is closed (true) or open (false).
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the stroke geometry.
    ///
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    fn stroke(&self, style: StrokeStyle<P, T>, is_closed_path: bool) -> Shapes<P>;

    /// Generates a stroke mesh for paths, contours, or shapes with optional filtering and scaling.
    ///
    /// - `style`: Defines the stroke properties, including width, line caps, and joins.
    /// - `is_closed_path`: Specifies whether the path is closed (true) or open (false).
    /// - `options`: Adjust custom behavior.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the stroke geometry.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn stroke_custom(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        options: OverlayOptions<T>,
    ) -> Shapes<P>;

    /// Generates a stroke shapes for paths, contours, or shapes with a fixed float-to-integer scale.
    ///
    /// - `style`: Defines the stroke properties, including width, line caps, and joins.
    /// - `is_closed_path`: Specifies whether the path is closed (true) or open (false).
    /// - `scale`: Fixed float-to-integer scale. Use `scale = 1.0 / grid_size` if you prefer grid size semantics.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the stroke geometry.
    ///
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    fn stroke_fixed_scale(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>;

    /// Generates a stroke mesh for paths, contours, or shapes with optional filtering and fixed scaling.
    ///
    /// - `style`: Defines the stroke properties, including width, line caps, and joins.
    /// - `is_closed_path`: Specifies whether the path is closed (true) or open (false).
    /// - `options`: Adjust custom behavior.
    /// - `scale`: Fixed float-to-integer scale. Use `scale = 1.0 / grid_size` if you prefer grid size semantics.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the stroke geometry.
    ///
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn stroke_custom_fixed_scale(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        options: OverlayOptions<T>,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>;
}

impl<S, P, T> StrokeOffset<P, T> for S
where
    S: ShapeResource<P, T>,
    P: FloatPointCompatible<T> + 'static,
    T: FloatNumber + 'static,
{
    fn stroke(&self, style: StrokeStyle<P, T>, is_closed_path: bool) -> Shapes<P> {
        self.stroke_custom(style, is_closed_path, Default::default())
    }

    fn stroke_fixed_scale(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        self.stroke_custom_fixed_scale(style, is_closed_path, Default::default(), scale)
    }

    fn stroke_custom(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        options: OverlayOptions<T>,
    ) -> Shapes<P> {
        match StrokeSolver::prepare(self, style) {
            Some(solver) => solver.build(self, is_closed_path, options),
            None => vec![],
        }
    }

    fn stroke_custom_fixed_scale(
        &self,
        style: StrokeStyle<P, T>,
        is_closed_path: bool,
        options: OverlayOptions<T>,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        let mut solver = match StrokeSolver::prepare(self, style) {
            Some(solver) => solver,
            None => return Ok(vec![]),
        };
        solver.apply_scale(scale)?;
        Ok(solver.build(self, is_closed_path, options))
    }
}

struct StrokeSolver<P: FloatPointCompatible<T>, T: FloatNumber> {
    r: T,
    builder: StrokeBuilder<P, T>,
    adapter: FloatPointAdapter<P, T>,
    paths_count: usize,
    points_count: usize,
}

impl<P: 'static + FloatPointCompatible<T>, T: 'static + FloatNumber> StrokeSolver<P, T> {
    fn prepare<S: ShapeResource<P, T>>(source: &S, style: StrokeStyle<P, T>) -> Option<Self> {
        let mut paths_count = 0;
        let mut points_count = 0;
        for path in source.iter_paths() {
            paths_count += 1;
            points_count += path.len();
        }

        if paths_count == 0 {
            return None;
        }

        let r = T::from_float(0.5 * style.width.to_f64());
        let builder = StrokeBuilder::new(style);
        let a = builder.additional_offset(r);

        let mut rect = FloatRect::with_iter(source.iter_paths().flatten()).unwrap_or(FloatRect::zero());
        rect.add_offset(a);
        let adapter = FloatPointAdapter::new(rect);

        Some(Self {
            r,
            builder,
            adapter,
            paths_count,
            points_count,
        })
    }

    fn apply_scale(&mut self, scale: T) -> Result<(), FixedScaleOverlayError> {
        let s = FixedScaleOverlayError::validate_scale(scale)?;
        if self.adapter.dir_scale < scale {
            return Err(FixedScaleOverlayError::ScaleTooLarge);
        }

        self.adapter.dir_scale = scale;
        self.adapter.inv_scale = T::from_float(1.0 / s);

        Ok(())
    }

    fn build<S: ShapeResource<P, T>>(
        self,
        source: &S,
        is_closed_path: bool,
        options: OverlayOptions<T>,
    ) -> Shapes<P> {
        let ir = self.adapter.len_float_to_int(self.r).abs();
        if ir <= 1 {
            // offset is too small
            return vec![];
        }

        let capacity = self
            .builder
            .capacity(self.paths_count, self.points_count, is_closed_path);
        let mut segments = Vec::with_capacity(capacity);

        for path in source.iter_paths() {
            self.builder
                .build(path, is_closed_path, &self.adapter, &mut segments);
        }

        let min_area = self.adapter.sqr_float_to_int(options.min_output_area);
        let shapes = OffsetOverlay::with_segments(segments)
            .build_graph_view_with_solver(Default::default())
            .map(|graph| graph.extract_offset(options.output_direction, min_area))
            .unwrap_or_default();

        let mut float = shapes.to_float(&self.adapter);

        if options.clean_result {
            if options.preserve_output_collinear {
                float.despike_contour(&self.adapter);
            } else {
                float.simplify_contour(&self.adapter);
            }
        };

        float
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::stroke::offset::StrokeOffset;
    use crate::mesh::style::{LineCap, LineJoin, StrokeStyle};
    use alloc::vec;
    use alloc::vec::Vec;
    use core::f32::consts::PI;

    #[test]
    fn test_doc() {
        let path = [
            [2.0, 1.0],
            [5.0, 1.0],
            [8.0, 4.0],
            [11.0, 4.0],
            [11.0, 1.0],
            [8.0, 1.0],
            [5.0, 4.0],
            [2.0, 4.0],
        ];

        let style = StrokeStyle::new(1.0)
            .line_join(LineJoin::Miter(1.0))
            .start_cap(LineCap::Round(0.1))
            .end_cap(LineCap::Square);

        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 2);
    }

    #[test]
    fn test_simple() {
        let path = [[0.0, 0.0], [10.0, 0.0]];

        let style = StrokeStyle::new(2.0);
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_bevel_join() {
        let path = [[-10.0, 0.0], [0.0, 0.0], [0.0, 10.0]];

        let style = StrokeStyle::new(2.0);
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);

        let path = shape.first().unwrap();
        assert_eq!(path.len(), 7);
    }

    #[test]
    fn test_round_join() {
        let path = [[-10.0, 0.0], [0.0, 0.0], [0.0, 10.0]];

        let style = StrokeStyle::new(2.0).line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_miter_join_turn_right() {
        let path = [[-6.0, -12.0], [0.0, 0.0], [6.0, -12.0]];

        let style = StrokeStyle::new(2.0).line_join(LineJoin::Miter(5.0 * PI / 180.0));
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_simple_closed() {
        let path = [[-5.0, -5.0], [-5.0, 5.0], [5.0, 5.0], [5.0, -5.0]];

        let style = StrokeStyle::new(2.0);

        let shapes = path.stroke_custom(style, true, Default::default());
        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 2);

        assert_eq!(shape[0].len(), 8);
        assert_eq!(shape[1].len(), 4);
    }

    #[test]
    fn test_miter_0() {
        let path = [
            [550.0, 225.0],
            [500.0, 250.0],
            [450.0, 275.0],
            [500.0, 300.0],
            [550.0, 325.0],
        ];

        let style = StrokeStyle::new(10.0).line_join(LineJoin::Miter(0.1));

        let shapes = path.stroke_custom(style, false, Default::default());
        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_miter_1() {
        let path = [[100.0, 100.0], [200.0, 200.0], [150.0, 250.0]];

        let style = StrokeStyle::new(10.0).line_join(LineJoin::Miter(0.1));

        let shapes = path.stroke_custom(style, false, Default::default());
        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_degenerate_0() {
        let path: Vec<[f64; 2]> = Vec::new();

        let style = StrokeStyle::new(2.0);
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_degenerate_1() {
        let path = [[0.0, 0.0]];

        let style = StrokeStyle::new(2.0);
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_degenerate_2() {
        let path = [[0.0, 0.0]];

        let style = StrokeStyle::new(2.0).end_cap(LineCap::Round(0.25 * PI));
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_degenerate_3() {
        let path = [[0.0, 0.0]];

        let style = StrokeStyle::new(2.0)
            .start_cap(LineCap::Butt)
            .end_cap(LineCap::Round(0.25 * PI));
        let shapes = path.stroke(style, false);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_many_paths() {
        let paths = [vec![[0.0, 0.0], [5.0, 0.0]], vec![[0.0, 0.0], [5.0, -5.0]]];

        let style = StrokeStyle::new(2.0)
            .start_cap(LineCap::Butt)
            .end_cap(LineCap::Butt);
        let shapes = paths.stroke(style, false);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 8);
    }

    #[test]
    fn test_stroke_fixed_scale_ok() {
        let path = [[0.0, 0.0], [10.0, 0.0]];
        let style = StrokeStyle::new(2.0);

        let shapes = path.stroke_fixed_scale(style, false, 10.0).unwrap();

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_stroke_fixed_scale_invalid() {
        let path = [[0.0, 0.0], [10.0, 0.0]];
        let style = StrokeStyle::new(2.0);

        assert!(path.stroke_fixed_scale(style.clone(), false, 0.0).is_err());
        assert!(path.stroke_fixed_scale(style.clone(), false, -1.0).is_err());
        assert!(path.stroke_fixed_scale(style.clone(), false, f64::NAN).is_err());
        assert!(
            path.stroke_fixed_scale(style.clone(), false, f64::INFINITY)
                .is_err()
        );
    }
}
