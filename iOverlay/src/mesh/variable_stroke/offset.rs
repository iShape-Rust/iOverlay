use crate::core::fill_rule::FillRule;
use crate::core::integer::OverlayInt;
use crate::core::overlay::Overlay;
use crate::core::overlay_rule::OverlayRule;
use crate::float::hierarchy::FloatFlatShapeHierarchy;
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
use i_float::int::number::int::IntNumber;
use i_float::int::number::uint::UIntNumber;
use i_float::int::number::wide_int::WideIntNumber;
use i_shape::base::data::Shapes;
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::flat::float::FloatFlatContoursBuffer;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::simple::SimplifyContour;

/// Builds round-cap, round-join strokes whose width is stored at each centerline vertex.
pub trait VariableStrokeOffset<P>: VariableStrokeSource<P>
where
    P: FloatPointCompatible + 'static,
{
    fn variable_stroke(&self, style: VariableStrokeStyle<P::Scalar>) -> Shapes<P> {
        self.variable_stroke_custom(style, Default::default())
    }

    fn variable_stroke_hierarchy(&self, style: VariableStrokeStyle<P::Scalar>) -> FloatFlatShapeHierarchy<P> {
        self.variable_stroke_custom_hierarchy(style, Default::default())
    }

    fn variable_stroke_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        self.variable_stroke_custom_into(style, Default::default(), output)
    }

    fn variable_stroke_custom(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar>,
    ) -> Shapes<P> {
        self.variable_stroke_custom_as::<i32>(style, options)
    }

    fn variable_stroke_custom_hierarchy(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar>,
    ) -> FloatFlatShapeHierarchy<P> {
        self.variable_stroke_custom_hierarchy_as::<i32>(style, options)
    }

    fn variable_stroke_custom_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        self.variable_stroke_custom_into_as::<i32>(style, options, output)
    }

    fn variable_stroke_fixed_scale(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale(style, Default::default(), scale)
    }

    fn variable_stroke_fixed_scale_hierarchy(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<FloatFlatShapeHierarchy<P>, FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_hierarchy(style, Default::default(), scale)
    }

    fn variable_stroke_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_into(style, Default::default(), scale, output)
    }

    fn variable_stroke_custom_fixed_scale(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_as::<i32>(style, options, scale)
    }

    fn variable_stroke_custom_fixed_scale_hierarchy(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<FloatFlatShapeHierarchy<P>, FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_hierarchy_as::<i32>(style, options, scale)
    }

    fn variable_stroke_custom_fixed_scale_into(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError> {
        self.variable_stroke_custom_fixed_scale_into_as::<i32>(style, options, scale, output)
    }

    fn variable_stroke_as<I>(&self, style: VariableStrokeStyle<P::Scalar>) -> Shapes<P>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_as::<I>(style, Default::default())
    }

    fn variable_stroke_hierarchy_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
    ) -> FloatFlatShapeHierarchy<P>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_hierarchy_as::<I>(style, Default::default())
    }

    fn variable_stroke_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_into_as::<I>(style, Default::default(), output)
    }

    fn variable_stroke_custom_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar, I>,
    ) -> Shapes<P>
    where
        I: OverlayInt + 'static,
    {
        match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver.build(self, options),
            None => vec![],
        }
    }

    fn variable_stroke_custom_hierarchy_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar, I>,
    ) -> FloatFlatShapeHierarchy<P>
    where
        I: OverlayInt + 'static,
    {
        match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver.build_hierarchy(self, options),
            None => FloatFlatShapeHierarchy::default(),
        }
    }

    fn variable_stroke_custom_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar, I>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) where
        I: OverlayInt + 'static,
    {
        match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver.build_into(self, options, output),
            None => output.clear_and_reserve(0, 0),
        }
    }

    fn variable_stroke_fixed_scale_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_fixed_scale_as::<I>(style, Default::default(), scale)
    }

    fn variable_stroke_fixed_scale_hierarchy_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        scale: P::Scalar,
    ) -> Result<FloatFlatShapeHierarchy<P>, FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_fixed_scale_hierarchy_as::<I>(style, Default::default(), scale)
    }

    fn variable_stroke_fixed_scale_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        self.variable_stroke_custom_fixed_scale_into_as::<I>(style, Default::default(), scale, output)
    }

    fn variable_stroke_custom_fixed_scale_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar, I>,
        scale: P::Scalar,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        let mut solver = match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver,
            None => return Ok(vec![]),
        };
        solver.apply_scale(scale)?;
        Ok(solver.build(self, options))
    }

    fn variable_stroke_custom_fixed_scale_hierarchy_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar, I>,
        scale: P::Scalar,
    ) -> Result<FloatFlatShapeHierarchy<P>, FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        let mut solver = match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver,
            None => return Ok(FloatFlatShapeHierarchy::default()),
        };
        solver.apply_scale(scale)?;
        Ok(solver.build_hierarchy(self, options))
    }

    fn variable_stroke_custom_fixed_scale_into_as<I>(
        &self,
        style: VariableStrokeStyle<P::Scalar>,
        options: OverlayOptions<P::Scalar, I>,
        scale: P::Scalar,
        output: &mut FloatFlatContoursBuffer<P>,
    ) -> Result<(), FixedScaleOverlayError>
    where
        I: OverlayInt + 'static,
    {
        let mut solver = match VariableStrokeSolver::<P, I>::prepare(self, style) {
            Some(solver) => solver,
            None => {
                output.clear_and_reserve(0, 0);
                return Ok(());
            }
        };
        solver.apply_scale(scale)?;
        solver.build_into(self, options, output);
        Ok(())
    }
}

impl<S, P> VariableStrokeOffset<P> for S
where
    S: VariableStrokeSource<P>,
    P: FloatPointCompatible + 'static,
{
}

struct VariableStrokeSolver<P: FloatPointCompatible, I: IntNumber> {
    max_radius: P::Scalar,
    builder: VariableStrokeBuilder<P::Scalar>,
    adapter: FloatPointAdapter<P, I>,
    paths_count: usize,
    points_count: usize,
}

impl<P, I> VariableStrokeSolver<P, I>
where
    P: FloatPointCompatible + 'static,
    I: OverlayInt + 'static,
{
    fn prepare<S: VariableStrokeSource<P> + ?Sized>(
        source: &S,
        style: VariableStrokeStyle<P::Scalar>,
    ) -> Option<Self> {
        let mut max_radius = P::Scalar::ZERO;
        let mut paths_count = 0;
        let mut points_count = 0;
        let mut rect: Option<FloatRect<P::Scalar>> = None;

        for path in source.iter_variable_paths() {
            if path.is_empty() {
                continue;
            }
            paths_count += 1;
            points_count += path.len();
            for vertex in path {
                max_radius = max_radius.max(vertex.radius());
                if let Some(rect) = rect.as_mut() {
                    rect.add_point(&vertex.point);
                } else {
                    rect = Some(FloatRect::with_point(vertex.point));
                }
            }
        }

        if paths_count == 0 || points_count < 2 || max_radius <= P::Scalar::ZERO {
            return None;
        }

        let builder = VariableStrokeBuilder::new(style);
        let mut rect = rect?;
        rect.add_offset(builder.additional_offset(max_radius));
        let adapter = FloatPointAdapter::<P, I>::new(rect);

        Some(Self {
            max_radius,
            builder,
            adapter,
            paths_count,
            points_count,
        })
    }

    fn apply_scale(&mut self, scale: P::Scalar) -> Result<(), FixedScaleOverlayError> {
        self.adapter = FloatPointAdapter::try_with_scale(*self.adapter.rect(), scale)?;
        Ok(())
    }

    fn build<S: VariableStrokeSource<P> + ?Sized>(
        self,
        source: &S,
        options: OverlayOptions<P::Scalar, I>,
    ) -> Shapes<P> {
        if self.radius_is_too_small() {
            return vec![];
        }

        let mut segments = Vec::with_capacity(self.builder.capacity(self.paths_count, self.points_count));
        for path in source.iter_variable_paths() {
            self.builder.build(path, &self.adapter, &mut segments);
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
        }
        float
    }

    fn build_hierarchy<S: VariableStrokeSource<P> + ?Sized>(
        self,
        source: &S,
        options: OverlayOptions<P::Scalar, I>,
    ) -> FloatFlatShapeHierarchy<P> {
        if self.radius_is_too_small() {
            return FloatFlatShapeHierarchy::default();
        }

        let mut segments = Vec::with_capacity(self.builder.capacity(self.paths_count, self.points_count));
        for path in source.iter_variable_paths() {
            self.builder.build(path, &self.adapter, &mut segments);
        }

        let clean_result = options.clean_result;
        let preserve_output_collinear = options.preserve_output_collinear;
        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options.int_with_adapter(&self.adapter);
        let hierarchy = overlay.overlay_hierarchy(OverlayRule::Subject, FillRule::Positive);

        FloatFlatShapeHierarchy::from_int(hierarchy, &self.adapter, clean_result, preserve_output_collinear)
    }

    fn build_into<S: VariableStrokeSource<P> + ?Sized>(
        self,
        source: &S,
        options: OverlayOptions<P::Scalar, I>,
        output: &mut FloatFlatContoursBuffer<P>,
    ) {
        if self.radius_is_too_small() {
            output.clear_and_reserve(0, 0);
            return;
        }

        let mut segments = Vec::with_capacity(self.builder.capacity(self.paths_count, self.points_count));
        for path in source.iter_variable_paths() {
            self.builder.build(path, &self.adapter, &mut segments);
        }

        let mut overlay = Overlay::with_segments(segments);
        overlay.options = options.int_with_adapter(&self.adapter);
        let mut int_output = FlatContoursBuffer::<I>::with_capacity(0);
        overlay.overlay_into(OverlayRule::Subject, FillRule::Positive, &mut int_output);

        let iter = int_output
            .points
            .iter()
            .map(|point| self.adapter.int_to_float(point));
        output.set_with_iter(iter, &int_output.ranges);
        if options.clean_result {
            if options.preserve_output_collinear {
                output.despike_contour(&self.adapter);
            } else {
                output.simplify_contour(&self.adapter);
            }
        }
    }

    #[inline]
    fn radius_is_too_small(&self) -> bool {
        let radius = self
            .adapter
            .round_len_to_int(self.max_radius)
            .to_wide()
            .unsigned_abs();
        radius <= I::WideUInt::ONE
    }
}

#[cfg(test)]
mod tests {
    use super::VariableStrokeOffset;
    use crate::mesh::stroke::offset::StrokeOffset;
    use crate::mesh::style::{LineCap, LineJoin, StrokeStyle};
    use crate::mesh::variable_stroke::{StrokeVertex, VariableStrokeStyle};
    use alloc::vec;
    use alloc::vec::Vec;
    use i_shape::float::area::Area;

    #[test]
    fn equal_width_builds_round_stroke() {
        let path = vec![
            StrokeVertex::new([0.0, 0.0], 4.0),
            StrokeVertex::new([10.0, 0.0], 4.0),
        ];
        let shapes = path.variable_stroke(VariableStrokeStyle::new());
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
    }

    #[test]
    fn supports_i64_engine() {
        let path = [
            StrokeVertex::new([0.0, 0.0], 2.0),
            StrokeVertex::new([10.0, 0.0], 6.0),
        ];
        let shapes = path.variable_stroke_as::<i64>(VariableStrokeStyle::new());
        assert!(!shapes.is_empty());
    }

    #[test]
    fn hierarchy_links_nested_stroke_shapes() {
        let paths = vec![
            closed_square_variable_path(0.0, 100.0, 10.0),
            closed_square_variable_path(30.0, 70.0, 10.0),
        ];
        let style = VariableStrokeStyle::new().round_angle(0.1);
        let hierarchy = paths
            .variable_stroke_fixed_scale_hierarchy(style, 1_000.0)
            .unwrap();
        let regular_shapes = paths.variable_stroke_fixed_scale(style, 1_000.0).unwrap();

        assert_eq!(hierarchy.shapes.to_shapes(), regular_shapes);
        assert_eq!(hierarchy.shapes.shape_ranges.len(), 2);
        assert_eq!(hierarchy.links.len(), 1);

        let link = hierarchy.links[0];
        assert_ne!(link.parent_shape_index, link.child_shape_index);
        assert!(hierarchy.shapes.shape_ranges[link.parent_shape_index].contains(&link.parent_contour_index));
    }

    #[test]
    fn zero_width_is_empty() {
        let path = [
            StrokeVertex::new([0.0, 0.0], 0.0),
            StrokeVertex::new([10.0, 0.0], 0.0),
        ];
        let shapes = path.variable_stroke(VariableStrokeStyle::new());
        assert!(shapes.is_empty());
    }

    #[test]
    fn constant_width_matches_static_round_stroke_area() {
        let points = [[0.0f64, 0.0], [10.0, 0.0], [15.0, 8.0]];
        let path = points.map(|point| StrokeVertex::new(point, 4.0));
        let angle = 0.1;
        let actual = path
            .variable_stroke_fixed_scale(VariableStrokeStyle::new().round_angle(angle), 1_000.0)
            .unwrap();
        let expected = points
            .stroke_fixed_scale(
                StrokeStyle::new(4.0)
                    .start_cap(LineCap::Round(angle))
                    .end_cap(LineCap::Round(angle))
                    .line_join(LineJoin::Round(angle)),
                false,
                1_000.0,
            )
            .unwrap();

        let delta = (actual.area() - expected.area()).abs();
        assert!(delta < 0.1, "area delta: {delta}");
    }

    #[test]
    fn reversing_regular_path_preserves_area() {
        let path = vec![
            StrokeVertex::new([0.0f64, 0.0], 2.0),
            StrokeVertex::new([10.0, 3.0], 7.0),
            StrokeVertex::new([18.0, -2.0], 4.0),
        ];
        let mut reversed = path.clone();
        reversed.reverse();
        let style = VariableStrokeStyle::new().round_angle(0.08);
        let forward = path.variable_stroke_fixed_scale(style, 10_000.0).unwrap();
        let backward = reversed.variable_stroke_fixed_scale(style, 10_000.0).unwrap();

        assert!((forward.area() - backward.area()).abs() < 0.01);
    }

    #[test]
    fn variable_tangent_outline_does_not_leave_center_notch() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 6.0_f32),
            StrokeVertex::new([46.829_998_f32, 14.88_f32], 18.0_f32),
            StrokeVertex::new([70.0_f32, 0.0_f32], 42.0_f32),
            StrokeVertex::new([105.0_f32, 25.0_f32], 15.0_f32),
            StrokeVertex::new([140.0_f32, 15.0_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style);
        let join = paths[0][1].point;
        let has_center_notch = result
            .iter()
            .flatten()
            .flatten()
            .any(|point| (point[0] - join[0]).abs() < 0.001 && (point[1] - join[1]).abs() < 0.001);

        assert!(!has_center_notch);
    }

    #[test]
    fn reversed_tangent_order_builds_outline() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 4.5_f32),
            StrokeVertex::new([45.32_f32, 9.559_999_f32], 13.5_f32),
            StrokeVertex::new([91.299_995_f32, 0.89_f32], 31.5_f32),
            StrokeVertex::new([102.88_f32, -2.44_f32], 11.25_f32),
            StrokeVertex::new([140.0_f32, 15.0_f32], 22.5_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn near_u_turn_preserves_both_edge_widths() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 6.0_f32),
            StrokeVertex::new([96.93_f32, 0.669_999_96_f32], 18.0_f32),
            StrokeVertex::new([70.0_f32, 0.0_f32], 42.0_f32),
            StrokeVertex::new([105.0_f32, 25.0_f32], 15.0_f32),
            StrokeVertex::new([140.0_f32, 15.0_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn u_turn_keeps_round_outline_at_reversal_vertex() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 8.0_f32),
            StrokeVertex::new([60.0_f32, 0.0_f32], 20.0_f32),
            StrokeVertex::new([12.309_999_f32, 5.91_f32], 10.0_f32),
            StrokeVertex::new([65.0_f32, 20.0_f32], 16.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style);
        let min_y_near_reversal = result
            .iter()
            .flatten()
            .flatten()
            .filter(|point| point[0] > 50.0)
            .map(|point| point[1])
            .fold(f32::MAX, f32::min);

        assert_eq!(result.len(), 1);
        assert!(
            min_y_near_reversal < -9.5,
            "round outline at the reversal vertex was lost: y={min_y_near_reversal}"
        );
    }

    #[test]
    fn wide_turn_preserves_outer_radius() {
        let paths = vec![vec![
            StrokeVertex::new([-11.599_999_f32, 35.16_f32], 6.0_f32),
            StrokeVertex::new([149.599_99_f32, 86.88_f32], 18.0_f32),
            StrokeVertex::new([70.0_f32, 0.0_f32], 42.0_f32),
            StrokeVertex::new([107.149_994_f32, 27.82_f32], 15.0_f32),
            StrokeVertex::new([156.72_f32, -34.079_998_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn covered_open_taper_uses_larger_vertex_as_round_start() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 6.0_f32),
            StrokeVertex::new([3.86_f32, 0.28_f32], 18.0_f32),
            StrokeVertex::new([75.06_f32, 40.12_f32], 42.0_f32),
            StrokeVertex::new([145.72_f32, 11.719_999_f32], 15.0_f32),
            StrokeVertex::new([159.519_99_f32, 60.34_f32], 30.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.179_999_99_f32);
        let result = paths.variable_stroke(style);

        assert_eq!(result.len(), 1);

        let center = paths[0][1].point;
        let next = paths[0][2].point;
        let vector = [next[0] - center[0], next[1] - center[1]];
        let length = (vector[0] * vector[0] + vector[1] * vector[1]).sqrt();
        let direction = [vector[0] / length, vector[1] / length];
        let min_projection = result
            .iter()
            .flatten()
            .flatten()
            .map(|point| (point[0] - center[0]) * direction[0] + (point[1] - center[1]) * direction[1])
            .fold(f32::MAX, f32::min);

        assert!(
            min_projection < -8.5,
            "round start does not cover the larger circle: projection={min_projection}"
        );
    }

    fn closed_square_variable_path(min: f64, max: f64, width: f64) -> Vec<StrokeVertex<[f64; 2]>> {
        [[min, min], [max, min], [max, max], [min, max], [min, min]]
            .map(|point| StrokeVertex::new(point, width))
            .to_vec()
    }
}
