use crate::build::offset::{NegativeSubjectOffsetStrategy, PositiveSubjectOffsetStrategy};
use crate::core::extract::BooleanExtractionBuffer;
use crate::core::fill_rule::FillRule;
use crate::core::overlay::{ContourDirection, Overlay, ShapeType};
use crate::core::overlay_rule::OverlayRule;
use crate::float::overlay::OverlayOptions;
use crate::float::scale::FixedScaleOverlayError;
use crate::mesh::outline::builder::OutlineBuilder;
use crate::mesh::overlay::OffsetOverlay;
use crate::mesh::style::OutlineStyle;
use alloc::vec;
use alloc::vec::Vec;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::rect::FloatRect;
use i_shape::base::data::Shapes;
use i_shape::flat::buffer::FlatContoursBuffer;
use i_shape::float::adapter::ShapesToFloat;
use i_shape::float::despike::DeSpikeContour;
use i_shape::float::int_area::IntArea;
use i_shape::float::simple::SimplifyContour;
use i_shape::source::resource::ShapeResource;

pub trait OutlineOffset<P: FloatPointCompatible<T>, T: FloatNumber> {
    /// Generates an outline shapes for contours, or shapes.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    fn outline(&self, style: &OutlineStyle<T>) -> Shapes<P>;

    /// Generates an outline shapes for contours, or shapes with optional filtering.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    /// - `options`: Adjust custom behavior.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn outline_custom(&self, style: &OutlineStyle<T>, options: OverlayOptions<T>) -> Shapes<P>;

    /// Generates an outline shapes for contours, or shapes with a fixed float-to-integer scale.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    /// - `scale`: Fixed float-to-integer scale. Use `scale = 1.0 / grid_size` if you prefer grid size semantics.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a counterclockwise order, and holes have a clockwise order.
    fn outline_fixed_scale(
        &self,
        style: &OutlineStyle<T>,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>;

    /// Generates an outline shapes for contours, or shapes with optional filtering and fixed scaling.
    ///
    /// - `style`: Defines the outline properties, including offset, and joins.
    /// - `options`: Adjust custom behavior.
    /// - `scale`: Fixed float-to-integer scale. Use `scale = 1.0 / grid_size` if you prefer grid size semantics.
    ///
    /// # Returns
    /// A collection of `Shapes<P>` representing the outline geometry.
    /// Note: Outer boundary paths have a **main_direction** order, and holes have an opposite to **main_direction** order.
    fn outline_custom_fixed_scale(
        &self,
        style: &OutlineStyle<T>,
        options: OverlayOptions<T>,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError>;
}

impl<S, P, T> OutlineOffset<P, T> for S
where
    S: ShapeResource<P, T>,
    P: FloatPointCompatible<T> + 'static,
    T: FloatNumber + 'static,
{
    fn outline(&self, style: &OutlineStyle<T>) -> Shapes<P> {
        self.outline_custom(style, Default::default())
    }

    fn outline_custom(&self, style: &OutlineStyle<T>, options: OverlayOptions<T>) -> Shapes<P> {
        match OutlineSolver::prepare(self, style) {
            Some(solver) => solver.build(self, options),
            None => vec![],
        }
    }

    fn outline_fixed_scale(
        &self,
        style: &OutlineStyle<T>,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        self.outline_custom_fixed_scale(style, Default::default(), scale)
    }

    fn outline_custom_fixed_scale(
        &self,
        style: &OutlineStyle<T>,
        options: OverlayOptions<T>,
        scale: T,
    ) -> Result<Shapes<P>, FixedScaleOverlayError> {
        let s = FixedScaleOverlayError::validate_scale(scale)?;
        let mut solver = match OutlineSolver::prepare(self, style) {
            Some(solver) => solver,
            None => return Ok(vec![]),
        };
        solver.apply_scale(s)?;
        Ok(solver.build(self, options))
    }
}

struct OutlineSolver<P: FloatPointCompatible<T>, T: FloatNumber> {
    outer_builder: OutlineBuilder<P, T>,
    inner_builder: OutlineBuilder<P, T>,
    adapter: FloatPointAdapter<P, T>,
    points_count: usize,
    paths_count: usize,
}

impl<P: FloatPointCompatible<T> + 'static, T: FloatNumber + 'static> OutlineSolver<P, T> {
    fn prepare<S: ShapeResource<P, T>>(source: &S, style: &OutlineStyle<T>) -> Option<Self> {
        let (points_count, paths_count) = {
            let mut points_count = 0;
            let mut paths_count = 0;
            for path in source.iter_paths() {
                points_count += path.len();
                paths_count += 1;
            }
            (points_count, paths_count)
        };

        if paths_count == 0 {
            return None;
        }

        let join = style.join.clone().normalize();
        let outer_builder = OutlineBuilder::new(-style.outer_offset, &join);
        let inner_builder = OutlineBuilder::new(-style.inner_offset, &join);

        let outer_radius = style.outer_offset;
        let inner_radius = style.inner_offset;

        let outer_additional_offset = outer_builder.additional_offset(outer_radius);
        let inner_additional_offset = inner_builder.additional_offset(inner_radius);

        let additional_offset = outer_additional_offset.abs() + inner_additional_offset.abs();

        let mut rect = FloatRect::with_iter(source.iter_paths().flatten()).unwrap_or(FloatRect::zero());
        rect.add_offset(additional_offset);

        let adapter = FloatPointAdapter::new(rect);

        Some(Self {
            outer_builder,
            inner_builder,
            adapter,
            points_count,
            paths_count,
        })
    }

    fn apply_scale(&mut self, scale: f64) -> Result<(), FixedScaleOverlayError> {
        let s = T::from_float(scale);
        if self.adapter.dir_scale < s {
            return Err(FixedScaleOverlayError::ScaleTooLarge);
        }

        self.adapter.dir_scale = s;
        self.adapter.inv_scale = T::from_float(1.0 / scale);

        Ok(())
    }

    fn build<S: ShapeResource<P, T>>(self, source: &S, options: OverlayOptions<T>) -> Shapes<P> {
        let int_min_area = self.adapter.sqr_float_to_int(options.min_output_area).max(1);

        let shapes = if self.paths_count <= 1 {
            // fast solution for a single path
            let path = if let Some(first) = source.iter_paths().next() {
                first
            } else {
                return vec![];
            };

            let area = path.unsafe_int_area(&self.adapter);
            if area >= -1 {
                // single path must be clock-wised
                return vec![];
            }

            let capacity = self.outer_builder.capacity(path.len());
            let mut segments = Vec::with_capacity(capacity);
            self.outer_builder.build(path, &self.adapter, &mut segments);

            OffsetOverlay::with_segments(segments)
                .build_graph_view_with_solver::<PositiveSubjectOffsetStrategy>(Default::default())
                .map(|graph| {
                    graph.extract_offset(options.output_direction, int_min_area, &mut Default::default())
                })
                .unwrap_or_default()
        } else {
            let total_capacity = self.outer_builder.capacity(self.points_count);
            let mut overlay = Overlay::new_custom(
                total_capacity,
                options.int_with_adapter(&self.adapter),
                Default::default(),
            );

            let mut offset_overlay = OffsetOverlay::new(128);

            let mut segments = Vec::new();
            let mut extraction_buffer = BooleanExtractionBuffer::default();
            let mut flat_buffer = FlatContoursBuffer::default();

            for path in source.iter_paths() {
                let area = path.unsafe_int_area(&self.adapter);
                if area.abs() <= 1 {
                    // ignore degenerate paths
                    continue;
                }

                let (offset_graph, direction) = if area < 0 {
                    let capacity = self.outer_builder.capacity(path.len());
                    let additional = capacity.saturating_sub(segments.capacity());
                    if additional > 0 {
                        segments.reserve(additional);
                    }
                    segments.clear();

                    self.outer_builder.build(path, &self.adapter, &mut segments);

                    offset_overlay.clear();
                    offset_overlay.add_segments(&segments);

                    let graph = offset_overlay
                        .build_graph_view_with_solver::<PositiveSubjectOffsetStrategy>(Default::default());
                    (graph, ContourDirection::CounterClockwise)
                } else {
                    let capacity = self.inner_builder.capacity(path.len());
                    let additional = capacity.saturating_sub(segments.capacity());
                    if additional > 0 {
                        segments.reserve(additional);
                    }
                    segments.clear();

                    self.inner_builder.build(path, &self.adapter, &mut segments);

                    offset_overlay.clear();
                    offset_overlay.add_segments(&segments);

                    let graph = offset_overlay
                        .build_graph_view_with_solver::<NegativeSubjectOffsetStrategy>(Default::default());
                    (graph, ContourDirection::Clockwise)
                };

                if let Some(graph) = offset_graph {
                    graph.extract_contours_into(direction, 0, &mut extraction_buffer, &mut flat_buffer);
                    overlay.add_flat_buffer(&flat_buffer, ShapeType::Subject);
                }
            }

            overlay.overlay(OverlayRule::Subject, FillRule::Positive)
        };

        if options.clean_result {
            let mut float = shapes.to_float(&self.adapter);
            if options.preserve_output_collinear {
                float.despike_contour(&self.adapter);
            } else {
                float.simplify_contour(&self.adapter);
            }
            float
        } else {
            shapes.to_float(&self.adapter)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::outline::offset::OutlineOffset;
    use crate::mesh::style::{LineJoin, OutlineStyle};
    use alloc::vec;
    use core::f32::consts::PI;

    #[test]
    fn test_doc() {
        let shape = vec![
            vec![
                [2.0, 1.0],
                [4.0, 1.0],
                [5.0, 2.0],
                [13.0, 2.0],
                [13.0, 3.0],
                [12.0, 3.0],
                [12.0, 4.0],
                [11.0, 4.0],
                [11.0, 3.0],
                [10.0, 3.0],
                [9.0, 4.0],
                [8.0, 4.0],
                [8.0, 3.0],
                [5.0, 3.0],
                [5.0, 4.0],
                [4.0, 5.0],
                [2.0, 5.0],
                [1.0, 4.0],
                [1.0, 2.0],
            ],
            vec![[2.0, 4.0], [4.0, 4.0], [4.0, 2.0], [2.0, 2.0]],
        ];

        let style = OutlineStyle::new(0.2).line_join(LineJoin::Round(0.1));
        let shapes = shape.outline(&style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 2);
    }

    #[test]
    fn test_triangle_round_corner() {
        let path = [[0.0, 0.0f32], [10.0, 0.0f32], [0.0, 10.0f32]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);
    }

    #[test]
    fn test_reversed_triangle_round_corner() {
        let path = [[0.0, 0.0f32], [0.0, 10.0f32], [10.0, 0.0f32]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Round(0.25 * PI));
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_square() {
        let path = [[-5.0, -5.0f32], [5.0, -5.0], [5.0, 5.0], [-5.0, 5.0]];

        let style = OutlineStyle::new(10.0);
        let shapes = path.outline_fixed_scale(&style, 10.0).unwrap();

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);

        let path = shape.first().unwrap();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_square_round_offset() {
        let path = [[-5.0, -5.0f32], [5.0, -5.0], [5.0, 5.0], [-5.0, 5.0]];

        let angle = PI / 3.0f32;
        let style = OutlineStyle::new(10.0).line_join(LineJoin::Round(angle));

        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_square_negative_offset() {
        let path = [[-5.0, -5.0f32], [5.0, -5.0], [5.0, 5.0], [-5.0, 5.0]];

        let style = OutlineStyle::new(-20.0);
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_square_negative_round_offset() {
        let path = [[-5.0, -5.0f32], [5.0, -5.0], [5.0, 5.0], [-5.0, 5.0]];

        let angle = PI / 3.0f32;
        let style = OutlineStyle::new(-20.0).line_join(LineJoin::Round(angle));

        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_rhombus_miter() {
        let path = [[-10.0, 0.0], [0.0, -10.0], [10.0, 0.0], [0.0, 10.0]];

        let style = OutlineStyle::new(5.0).line_join(LineJoin::Miter(0.01));
        let shapes = path.outline(&style);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes.first().unwrap().len(), 1);
    }

    #[test]
    fn test_window() {
        let window = vec![
            vec![[-10.0, -10.0], [10.0, -10.0], [10.0, 10.0], [-10.0, 10.0]],
            vec![[-5.0, -5.0], [-5.0, 5.0], [5.0, 5.0], [5.0, -5.0]],
        ];

        let style = OutlineStyle::new(1.0).line_join(LineJoin::Bevel);
        let shapes = window.outline_fixed_scale(&style, 10.0).unwrap();

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 2);
        assert_eq!(shapes[0][0].len(), 8);
        assert_eq!(shapes[0][1].len(), 4);
    }

    // [[[[300.0, 300.0], [500.0, 300.0], [500.0, 500.0], [300.0, 500.0]]]]
    #[test]
    fn test_float_square_0() {
        let shape = vec![vec![
            [300.0, 300.0],
            [500.0, 300.0],
            [500.0, 500.0],
            [300.0, 500.0],
        ]];

        let style = OutlineStyle::default().outer_offset(50.0).inner_offset(50.0);

        let shapes = shape.outline(&style);

        assert_eq!(shapes.len(), 1);

        let shape = shapes.first().unwrap();
        assert_eq!(shape.len(), 1);

        let path = shape.first().unwrap();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_outline_fixed_scale_ok() {
        let path = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let style = OutlineStyle::new(1.0);

        let shapes = path.outline_fixed_scale(&style, 10.0).unwrap();

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_outline_fixed_scale_invalid() {
        let path = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let style = OutlineStyle::new(1.0);

        assert!(path.outline_fixed_scale(&style, 0.0).is_err());
        assert!(path.outline_fixed_scale(&style, -1.0).is_err());
        assert!(path.outline_fixed_scale(&style, f64::NAN).is_err());
        assert!(path.outline_fixed_scale(&style, f64::INFINITY).is_err());
    }

    #[test]
    fn test_zero_length_segment_0() {
        let path = [
            [2681.39599938213, 5892784.488998892],
            [5419.06964821636, 5891947.742386343],
            [5419.1446127397, 5891949.316633703],
            [5422.8669123155, 5892027.484991552],
            [5034.8682417375, 5892817.151239874],
            [4804.8188261491, 5892876.799252035],
            [4804.81882805645, 5892876.799253942],
            [4551.3436274034, 5892942.5211854],
            [2681.39599938213, 5892784.488998892],
        ];

        let angle = 10.0f64 / (core::f64::consts::PI / 2.0f64);
        let style = OutlineStyle::new(150.0).line_join(LineJoin::Round(angle));

        if let Some(shape) = path.outline(&style).first() {
            assert!(shape[0].len() < 1_000);
        };
    }

    #[test]
    fn test_zero_length_segment_1() {
        let path = [
            [2681.39599938213, 5892876.0],
            [5400.0, 5891947.742386343],
            [5400.0, 5892817.151239874],
            [4804.8188261491, 5892876.799252035],
            [4804.81882805645, 5892876.799253942],
        ];

        let angle = 10.0f64 / (core::f64::consts::PI / 2.0f64);
        let style = OutlineStyle::new(150.0).line_join(LineJoin::Round(angle));

        if let Some(shape) = path.outline(&style).first() {
            assert!(shape[0].len() < 1_000);
        };
    }

    #[test]
    fn test_real_case_0() {
        let main = vec![
            [411162.0470393328, 5848155.806033095],
            [411162.3299983172, 5848152.285037002],
            [411162.44901687186, 5848149.446047744],
            [411167.5609553484, 5848148.9709500875],
            [411175.2629817156, 5848147.891970595],
            [411186.7560237078, 5848146.501955947],
            [411203.86503249686, 5848144.432009658],
            [411214.44804030936, 5848143.314944228],
            [411221.0470393328, 5848142.421999892],
            [411227.85697585624, 5848141.499026259],
            [411233.74100905936, 5848140.505007705],
            [411238.4249690203, 5848139.349978408],
            [411242.85697585624, 5848138.25305458],
            [411249.1400569109, 5848136.395022353],
            [411256.6129573015, 5848134.406008681],
            [411262.81803542655, 5848132.916018447],
            [411275.2460139422, 5848129.93298622],
            [411284.6999934344, 5848127.662966689],
            [411292.3739436297, 5848125.3869657125],
            [411295.41703445, 5848123.3430204],
            [411297.0079768328, 5848121.340945205],
            [411297.43900710624, 5848119.1510037985],
            [411293.11698562186, 5848105.54602333],
            [411287.24100905936, 5848076.412966689],
            [411286.6709407, 5848062.798953017],
            [411286.98099929374, 5848053.410037002],
            [411288.3879817156, 5848038.451052627],
            [411294.0620539813, 5848006.396975478],
            [411294.9409602312, 5847995.477053603],
            [411295.2140315203, 5847988.534060439],
            [411296.3359797625, 5847983.056033095],
            [411297.8600276141, 5847976.624026259],
            [411297.86698562186, 5847976.590945205],
            [411298.2679865984, 5847974.952029189],
            [411301.5709651141, 5847965.958010634],
            [411303.7980158953, 5847955.29602333],
            [411305.12797195, 5847948.927004775],
            [411307.15897780936, 5847937.4279813375],
            [411307.711956325, 5847934.313967666],
            [411310.8889582781, 5847916.500979384],
            [411311.8309748797, 5847911.5959500875],
            [411312.3839533953, 5847898.51098915],
            [411311.64603835624, 5847891.3459500875],
            [411308.97904616874, 5847878.494997939],
            [411303.793987575, 5847862.650027236],
            [411301.5509455828, 5847857.5080594625],
            [411297.4499934344, 5847849.958010634],
            [411294.81303054374, 5847846.331057509],
            [411281.3550227312, 5847828.88598915],
            [411261.0709651141, 5847805.2369412985],
            [411259.2100032, 5847804.3619412985],
            [411258.0150569109, 5847803.80005165],
            [411254.8910334734, 5847803.62805458],
            [411251.86503249686, 5847805.3430204],
            [411249.4499934344, 5847802.375002822],
            [411248.2519953875, 5847800.202029189],
            [411248.1909602312, 5847794.432009658],
            [411253.23197585624, 5847785.8170194235],
            [411255.7970393328, 5847788.224978408],
            [411257.6870539813, 5847789.534060439],
            [411259.8690608172, 5847790.333010634],
            [411262.0520442156, 5847790.332034072],
            [411268.8910334734, 5847788.8420438375],
            [411269.4320490984, 5847790.333010634],
            [411270.6329768328, 5847793.6369657125],
            [411269.1820490984, 5847794.332034072],
            [411267.65397292655, 5847796.224001845],
            [411266.9259455828, 5847798.990969619],
            [411267.6709407, 5847800.479006728],
            [411268.7440608172, 5847802.625979384],
            [411281.10795241874, 5847816.8850125875],
            [411283.4420588641, 5847819.822024306],
            [411294.9740412859, 5847834.3430204],
            [411304.0699885515, 5847846.6369657125],
            [411307.27103835624, 5847852.748049697],
            [411309.8900569109, 5847857.912966689],
            [411311.78300124686, 5847864.460940322],
            [411313.6019709734, 5847869.698977431],
            [411314.9850276141, 5847872.171999892],
            [411317.53104812186, 5847875.446047744],
            [411320.7970393328, 5847877.480959853],
            [411325.86600905936, 5847879.2180204],
            [411335.5499690203, 5847882.012942275],
            [411368.4549983172, 5847890.26098915],
            [411387.668987575, 5847895.4010037985],
            [411397.7240412859, 5847898.576052627],
            [411405.50297195, 5847902.333010634],
            [411411.0599787859, 5847905.931033095],
            [411418.5199397234, 5847911.750979384],
            [411434.2660334734, 5847926.797976455],
            [411436.82304030936, 5847934.838015517],
            [411437.5780451922, 5847936.113039931],
            [411434.0089533953, 5847946.812991103],
            [411431.19804030936, 5847949.901980361],
            [411411.5659602312, 5847985.880984267],
            [411407.6529963641, 5847993.0510282125],
            [411404.77994948905, 5848000.453982314],
            [411402.93803054374, 5848007.354983291],
            [411399.39701491874, 5848029.913943252],
            [411392.9289973406, 5848080.029055556],
            [411390.43998366874, 5848099.298953017],
            [411388.8789485125, 5848106.744021377],
            [411386.1429865984, 5848113.750979384],
            [411383.2870295672, 5848120.22595497],
            [411379.1269953875, 5848126.2580594625],
            [411373.0499690203, 5848132.165041884],
            [411368.44901687186, 5848135.741946181],
            [411362.3199885515, 5848138.749026259],
            [411354.7980158953, 5848141.105959853],
            [411345.8729670672, 5848143.8990506735],
            [411334.5969660906, 5848146.394045791],
            [411322.7279475359, 5848149.957034072],
            [411321.0050471453, 5848151.457034072],
            [411319.7229426531, 5848152.791995009],
            [411319.23698073905, 5848154.2740506735],
            [411319.336956325, 5848156.656008681],
            [411339.95194655936, 5848207.255007705],
            [411351.7620051531, 5848236.444949111],
            [411364.9020198015, 5848268.477053603],
            [411376.5170100359, 5848297.156008681],
            [411377.7340510515, 5848300.22595497],
            [411395.8690608172, 5848345.97595497],
            [411411.2689631609, 5848381.8459500875],
            [411413.1310237078, 5848382.543948134],
            [411405.27994948905, 5848384.8010282125],
            [411332.1410334734, 5848206.078005752],
            [411309.5890315203, 5848150.895998916],
            [411307.8690608172, 5848147.239993056],
            [411305.3419612078, 5848144.284060439],
            [411301.6329768328, 5848141.729983291],
            [411296.9740412859, 5848139.974978408],
            [411293.77494460624, 5848139.145022353],
            [411290.1350520281, 5848139.240969619],
            [411276.68998366874, 5848140.473025283],
            [411274.44901687186, 5848140.531008681],
            [411266.961956325, 5848144.207034072],
            [411247.6239436297, 5848146.937014541],
            [411246.2670100359, 5848147.2369412985],
            [411240.0699885515, 5848148.041018447],
            [411234.7219660906, 5848150.973025283],
            [411224.2479670672, 5848149.892947158],
            [411223.6759455828, 5848148.834963759],
            [411222.2269709734, 5848148.297976455],
            [411213.2560237078, 5848149.380984267],
            [411189.6649592547, 5848152.199953994],
            [411162.0470393328, 5848155.806033095],
        ];

        let hole = vec![
            [411294.2500422625, 5848072.3189725485],
            [411373.9180110125, 5848124.016970595],
            [411377.22904616874, 5848118.990969619],
            [411393.0859797625, 5848020.979983291],
            [411394.7030451922, 5848005.639040908],
            [411397.4359553484, 5848003.376955947],
            [411431.1029475359, 5847937.537966689],
            [411431.2639582781, 5847933.187991103],
            [411314.8390315203, 5848005.308962783],
            [411314.5590022234, 5848009.708010634],
            [411309.3459895281, 5848009.447024306],
            [411305.3719905047, 5848010.244997939],
            [411294.2500422625, 5848072.3189725485],
        ];

        let shape = vec![main, hole];

        let angle = 10.0f64 / (core::f64::consts::PI / 2.0f64);
        let style = OutlineStyle::new(600.0).line_join(LineJoin::Round(angle));

        if let Some(shape) = shape.outline(&style).first() {
            assert!(shape[0].len() < 1_000);
        };
    }

    #[test]
    fn test_real_case_0_simplified() {
        let main = vec![
            [410_000.0, 5847_000.0],
            [413_000.0, 5847_000.0],
            [413_000.0, 5850_000.0],
            [410_000.0, 5850_000.0],
        ];

        let hole = vec![
            [411_294.2500422625, 5848_072.3189725485],
            [411_373.9180110125, 5848_124.016970595],
            [411_377.22904616874, 5848_118.990969619],
            [411_393.0859797625, 5848_020.979983291],
            [411_394.7030451922, 5848_005.639040908],
            [411_397.4359553484, 5848_003.376955947],
            [411_431.1029475359, 5847_937.537966689],
            [411_431.2639582781, 5847_933.187991103],
            [411_314.8390315203, 5848_005.308962783],
            [411_314.5590022234, 5848_009.708010634],
            [411_309.3459895281, 5848_009.447024306],
            [411_305.3719905047, 5848_010.244997939],
        ];

        let shape = vec![main, hole];

        let angle = 10.0f64 / (core::f64::consts::PI / 2.0f64);
        let style = OutlineStyle::new(600.0).line_join(LineJoin::Round(angle));

        if let Some(shape) = shape.outline(&style).first() {
            assert!(shape[0].len() < 1_000);
        };
    }
}
