use crate::core::fill_rule::FillRule;
use crate::core::overlay::Overlay;
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::OverlayOptions;
use crate::float::scale::FixedScaleOverlayError;
use crate::mesh::variable_stroke::builder::VariableStrokeBuilder;
use crate::mesh::variable_stroke::resource::VariableStrokeSource;
use crate::mesh::variable_stroke::style::VariableStrokeStyle;
use alloc::vec;
use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::flat::float::FloatFlatContoursBuffer;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::simple::SimplifyContour;

pub trait VariableStrokeOffset<P: FloatPointCompatible> {
    fn variable_stroke(&self, style: VariableStrokeStyle<P>, is_closed_path: bool) -> Shapes<P>;

    fn variable_stroke_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        output: &mut FloatFlatContoursBuffer<P>,
    );

    fn variable_stroke_custom(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
    ) -> Shapes<P>;

    fn variable_stroke_custom_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
        output: &mut FloatFlatContoursBuffer<P>,
    );

    fn variable_stroke_fixed_scale(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>;

    fn variable_stroke_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError>;

    fn variable_stroke_custom_fixed_scale(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>;

    fn variable_stroke_custom_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError>;
}

impl<S, P> VariableStrokeOffset<P> for S
where
    S: VariableStrokeSource<P>,
    P: FloatPointCompatible + 'static,
{
    fn variable_stroke(&self, style: VariableStrokeStyle<P>, is_closed_path: bool) -> Shapes<P> {
        self.variable_stroke_custom(style, is_closed_path, Default::default())
    }

    fn variable_stroke_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        self.variable_stroke_custom_into(style, is_closed_path, Default::default(), output)
    }

    fn variable_stroke_custom(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
    ) -> Shapes<P> {
        match VariableStrokeSolver::prepare(self, style) {
            Some(solver) => solver.build(self, is_closed_path, options),
            None => vec![],
        }
    }

    fn variable_stroke_custom_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        match VariableStrokeSolver::prepare(self, style) {
            Some(solver) => solver.build_into(self, is_closed_path, options, output),
            None => output.clear_and_reserve(0, 0),
        }
    }

    fn variable_stroke_fixed_scale(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale(style, is_closed_path, Default::default(), scale)
    }

    fn variable_stroke_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_into(style, is_closed_path, Default::default(), scale, output)
    }

    fn variable_stroke_custom_fixed_scale(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        let mut solver = match VariableStrokeSolver::prepare(self, style) {
            Some(solver) => solver,
            None => return Ok(vec![]),
        };
        solver.apply_scale(scale)?;
        Ok(solver.build(self, is_closed_path, options))
    }

    fn variable_stroke_custom_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P>,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError> {
        let mut solver = match VariableStrokeSolver::prepare(self, style) {
            Some(solver) => solver,
            None => {
                output.clear_and_reserve(0, 0);
                return Ok(());
            }
        };
        solver.apply_scale(scale)?;
        solver.build_into(self, is_closed_path, options, output);
        Ok(())
    }
}

struct VariableStrokeSolver<P: FloatPointCompatible> {
    max_radius: P::Scalar,
    builder: VariableStrokeBuilder<P>,
    adapter: FloatPointAdapter<P>,
    paths_count: usize,
    points_count: usize,
}

impl<P: 'static + FloatPointCompatible> VariableStrokeSolver<P> {
    fn prepare<S: VariableStrokeSource<P>>(source: &S, style: VariableStrokeStyle<P>) -> Option<Self> {
        let mut paths_count = 0;
        let mut points_count = 0;
        let mut max_radius = P::Scalar::from_float(0.0);
        let mut rect: Option<FloatRect<P::Scalar>> = None;

        for path in source.iter_variable_paths() {
            paths_count += 1;
            points_count += path.len();
            for vertex in path {
                max_radius = max_radius.max(vertex.radius());
                if let Some(rect) = &mut rect {
                    rect.add_point(&vertex.point);
                } else {
                    rect = Some(FloatRect::with_point(vertex.point));
                }
            }
        }

        if paths_count == 0 || points_count == 0 {
            return None;
        }

        let builder = VariableStrokeBuilder::new(style);
        let additional_offset = builder.additional_offset(max_radius);

        let mut rect = rect.unwrap_or(FloatRect::zero());
        rect.add_offset(additional_offset);
        let adapter = FloatPointAdapter::new(rect);

        Some(Self {
            max_radius,
            builder,
            adapter,
            paths_count,
            points_count,
        })
    }

    fn apply_scale(&mut self, scale: P::Scalar) -> Result<(), FixedScaleOverlayError> {
        let s = FixedScaleOverlayError::validate_scale(scale)?;
        if self.adapter.dir_scale < scale {
            return Err(FixedScaleOverlayError::ScaleTooLarge);
        }

        self.adapter.dir_scale = scale;
        self.adapter.inv_scale = P::Scalar::from_float(1.0 / s);

        Ok(())
    }

    fn build<S: VariableStrokeSource<P>>(
        self,
        source: &S,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
    ) -> Shapes<P> {
        let ir = self.adapter.len_float_to_int(self.max_radius).abs();
        if ir <= 1 {
            return vec![];
        }

        let capacity = self
            .builder
            .capacity(self.paths_count, self.points_count, is_closed_path);
        let mut segments = Vec::with_capacity(capacity);

        for path in source.iter_variable_paths() {
            self.builder
                .build(path, is_closed_path, &self.adapter, &mut segments);
        }

        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options.int_with_adapter(&self.adapter);

        let shapes = overlay.overlay(OverlayRule::Subject, FillRule::Positive);
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

    fn build_into<S: VariableStrokeSource<P>>(
        self,
        source: &S,
        is_closed_path: bool,
        options: OverlayOptions<P::Scalar>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        let ir = self.adapter.len_float_to_int(self.max_radius).abs();
        if ir <= 1 {
            output.clear_and_reserve(0, 0);
            return;
        }

        let capacity = self
            .builder
            .capacity(self.paths_count, self.points_count, is_closed_path);
        let mut segments = Vec::with_capacity(capacity);

        for path in source.iter_variable_paths() {
            self.builder
                .build(path, is_closed_path, &self.adapter, &mut segments);
        }

        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options.int_with_adapter(&self.adapter);

        let mut int_output = FlatContoursBuffer::default();
        overlay.overlay_into(OverlayRule::Subject, FillRule::Positive, &mut int_output);

        let iter = int_output.points.iter().map(|p| self.adapter.int_to_float(p));
        output.set_with_iter(iter, &int_output.ranges);

        if options.clean_result {
            if options.preserve_output_collinear {
                output.despike_contour(&self.adapter);
            } else {
                output.simplify_contour(&self.adapter);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::style::{LineCap, LineJoin};
    use crate::mesh::variable_stroke::offset::VariableStrokeOffset;
    use crate::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};
    use alloc::vec;
    use core::f64::consts::PI;

    #[test]
    fn test_simple_variable_stroke() {
        let path = vec![
            StrokeVertex::new([0.0, 0.0], 2.0),
            StrokeVertex::new([10.0, 0.0], 8.0),
            StrokeVertex::new([20.0, 10.0], 4.0),
        ];

        let style = VariableStrokeStyle::new().line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.variable_stroke(style, false);

        assert!(!shapes.is_empty());
    }

    #[test]
    fn test_variable_stroke_caps() {
        let path = vec![
            StrokeVertex::new([0.0, 0.0], 4.0),
            StrokeVertex::new([10.0, 0.0], 8.0),
        ];

        let style = VariableStrokeStyle::new()
            .start_cap(LineCap::Round(0.25 * PI))
            .end_cap(LineCap::Square);
        let shapes = path.variable_stroke(style, false);

        assert!(!shapes.is_empty());
    }

    #[test]
    fn test_variable_stroke_fixed_scale_invalid() {
        let path = vec![
            StrokeVertex::new([0.0, 0.0], 4.0),
            StrokeVertex::new([10.0, 0.0], 8.0),
        ];

        let result = path.variable_stroke_fixed_scale(VariableStrokeStyle::new(), false, 0.0);

        assert!(result.is_err());
    }
}
