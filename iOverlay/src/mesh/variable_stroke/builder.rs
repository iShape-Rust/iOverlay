use crate::mesh::rotator::Rotator;
use crate::mesh::variable_stroke::section::{RadiusTrend, Section};
use crate::mesh::variable_stroke::style::{StrokeVertex, VariableStrokeStyle};
use crate::segm::boolean::ShapeCountBoolean;
use crate::segm::segment::Segment;
use alloc::vec::Vec;
use core::f64::consts::PI;
use i_float::adapter::FloatPointAdapter;
use i_float::float::compatible::FloatPointCompatible;
use i_float::float::number::FloatNumber;
use i_float::float::vector::FloatPointMath;
use i_float::int::number::int::IntNumber;
use i_float::int::number::wide_int::WideIntNumber;

#[cfg(feature = "variable_stroke_debug")]
use crate::mesh::variable_stroke::{VariableStrokeDebugEdge, VariableStrokeDebugEdgeKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cap {
    Butt,
    Round,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArcSweep {
    Minor,
    Major,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SubSegment {
    start: usize,
    end: usize,
    start_cap: Cap,
    end_cap: Cap,
}

pub(super) struct VariableStrokeBuilder<T: FloatNumber> {
    round_angle: T,
}

impl<T: FloatNumber> VariableStrokeBuilder<T> {
    pub(super) fn new(style: VariableStrokeStyle<T>) -> Self {
        Self {
            round_angle: style.normalized().round_angle,
        }
    }

    pub(super) fn build<P, I>(
        &self,
        path: &[StrokeVertex<P>],
        adapter: &FloatPointAdapter<P, I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if path.is_empty() {
            return;
        }

        let subsegments = Self::find_subsegments(path, adapter);
        let mut output = SegmentBuilder {
            adapter,
            segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        for subsegment in subsegments.iter() {
            self.add_subsegment(subsegment, path, &mut output);
        }
    }

    #[cfg(feature = "variable_stroke_debug")]
    pub(super) fn build_debug<P, I>(
        &self,
        path: &[StrokeVertex<P>],
        path_index: usize,
        adapter: &FloatPointAdapter<P, I>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
        debug_edges: &mut Vec<VariableStrokeDebugEdge<P>>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if path.is_empty() {
            return;
        }

        let subsegments = Self::find_subsegments(path, adapter);
        let mut output = SegmentBuilder {
            adapter,
            segments,
            debug_edges: Some(debug_edges),
            debug_path_index: path_index,
        };

        for subsegment in subsegments.iter() {
            self.add_subsegment(subsegment, path, &mut output);
        }
    }

    fn add_subsegment<P, I>(
        &self,
        subsegment: &SubSegment,
        path: &[StrokeVertex<P>],
        output: &mut SegmentBuilder<P, I>,
    ) where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if subsegment.start == subsegment.end {
            if subsegment.start_cap != Cap::Butt || subsegment.end_cap != Cap::Butt {
                let vertex = &path[subsegment.start];
                output.add_circle(&vertex.point, vertex.radius(), self.round_angle);
            }
            return;
        }

        let adapter = output.adapter;
        let mut sections = (subsegment.start..subsegment.end)
            .filter_map(|index| Section::try_new(&path[index], &path[index + 1], adapter));
        let Some(mut previous) = sections.next() else {
            return;
        };

        output.add_section(&previous);
        output.add_start_cap(&previous, subsegment.start_cap, self.round_angle);

        for section in sections {
            output.add_section(&section);
            output.add_join(&previous, &section, self.round_angle);
            previous = section;
        }

        output.add_end_cap(&previous, subsegment.end_cap, self.round_angle);
    }

    fn find_subsegments<P, I>(path: &[StrokeVertex<P>], adapter: &FloatPointAdapter<P, I>) -> Vec<SubSegment>
    where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        if path.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut start = 0;
        let mut start_cap = Cap::Round;
        let mut final_end_cap = Cap::Round;

        for (index, pair) in path.windows(2).enumerate() {
            final_end_cap = Cap::Round;

            if let Some((end_cap, next_start_cap)) = Self::break_caps(&pair[0], &pair[1], adapter) {
                result.push(SubSegment {
                    start,
                    end: index,
                    start_cap,
                    end_cap,
                });

                start = index + 1;
                start_cap = next_start_cap;
                continue;
            }

            if index > 0 && Self::circle_is_covered_by_section(&path[index - 1], &pair[0], &pair[1], adapter)
            {
                result.push(SubSegment {
                    start,
                    end: index,
                    start_cap,
                    end_cap: Cap::Round,
                });

                start = index;
                start_cap = Cap::Butt;
                final_end_cap = Cap::Butt;
            }
        }

        result.push(SubSegment {
            start,
            end: path.len() - 1,
            start_cap,
            end_cap: final_end_cap,
        });
        result
    }

    fn break_caps<P, I>(
        a: &StrokeVertex<P>,
        b: &StrokeVertex<P>,
        adapter: &FloatPointAdapter<P, I>,
    ) -> Option<(Cap, Cap)>
    where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        let int_a = adapter.float_to_int(&a.point);
        let int_b = adapter.float_to_int(&b.point);
        let a_radius = adapter.round_len_to_int(a.radius());
        let b_radius = adapter.round_len_to_int(b.radius());
        let radius_delta = a_radius.to_wide() - b_radius.to_wide();
        let distance_sqr = (int_b - int_a).sqr_length();

        if radius_delta * radius_delta < distance_sqr {
            return None;
        }

        if a_radius >= b_radius {
            Some((Cap::Round, Cap::Butt))
        } else {
            Some((Cap::Butt, Cap::Round))
        }
    }

    fn circle_is_covered_by_section<P, I>(
        a: &StrokeVertex<P>,
        b: &StrokeVertex<P>,
        c: &StrokeVertex<P>,
        adapter: &FloatPointAdapter<P, I>,
    ) -> bool
    where
        P: FloatPointCompatible<Scalar = T>,
        I: IntNumber,
    {
        let a_radius = adapter.round_len_to_int(a.radius());
        let b_radius = adapter.round_len_to_int(b.radius());
        let c_radius = adapter.round_len_to_int(c.radius());
        if a_radius.max(b_radius) <= c_radius {
            return false;
        }

        let Some(section) = Section::try_new(a, b, adapter) else {
            return false;
        };

        let points = [
            adapter.float_to_int(&section.a_left),
            adapter.float_to_int(&section.b_left),
            adapter.float_to_int(&section.b_right),
            adapter.float_to_int(&section.a_right),
        ];
        let center = adapter.float_to_int(&c.point);
        let radius = c_radius.to_wide();
        let first_edge = points[1] - points[0];
        let orientation = first_edge.cross_product(points[2] - points[1]);
        if orientation == I::Wide::ZERO {
            return false;
        }

        for index in 0..points.len() {
            let a = points[index];
            let b = points[(index + 1) % points.len()];
            let edge = b - a;
            let side = edge.cross_product(center - a);
            let interior_distance = if orientation > I::Wide::ZERO { side } else { -side };
            if interior_distance < I::Wide::ZERO {
                return false;
            }

            let length_sqr = edge.sqr_length();
            let mut length = length_sqr.isqrt();
            if length * length < length_sqr {
                length = length + I::Wide::ONE;
            }
            if interior_distance < radius * length {
                return false;
            }
        }

        true
    }

    pub(super) fn capacity(&self, paths_count: usize, points_count: usize) -> usize {
        let edge_count = points_count.saturating_sub(paths_count);
        let round_count = (T::from_float(2.0 * PI) / self.round_angle)
            .to_usize()
            .saturating_add(1);
        2 * edge_count + 2 * round_count * points_count
    }

    pub(super) fn additional_offset(&self, max_radius: T) -> T {
        T::from_float(1.1) * max_radius
    }
}

struct SegmentBuilder<'a, P: FloatPointCompatible, I: IntNumber> {
    adapter: &'a FloatPointAdapter<P, I>,
    segments: &'a mut Vec<Segment<ShapeCountBoolean, I>>,
    #[cfg(feature = "variable_stroke_debug")]
    debug_edges: Option<&'a mut Vec<VariableStrokeDebugEdge<P>>>,
    #[cfg(feature = "variable_stroke_debug")]
    debug_path_index: usize,
}

impl<P: FloatPointCompatible, I: IntNumber> SegmentBuilder<'_, P, I> {
    fn add_circle(&mut self, center: &P, radius: P::Scalar, angle: P::Scalar) {
        let int_radius = self.adapter.round_len_to_int(radius);
        if int_radius <= I::ONE {
            return;
        }

        let center = self.adapter.int_to_float(&self.adapter.float_to_int(center));
        let radius = self.adapter.len_to_float(int_radius);
        let count = (P::Scalar::from_float(2.0 * PI) / angle)
            .to_usize()
            .saturating_add(1)
            .clamp(3, 1024);
        let rotator = Rotator::with_angle(P::Scalar::from_float(2.0 * PI) / P::Scalar::from_usize(count));
        let mut vector = P::from_xy(radius, P::Scalar::ZERO);
        let first = FloatPointMath::add(&center, &vector);
        let mut a = first;

        for i in 1..=count {
            let b = if i == count {
                first
            } else {
                vector = rotator.rotate(&vector);
                FloatPointMath::add(&center, &vector)
            };
            self.add_edge(
                &a,
                &b,
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::CircleArc,
            );
            a = b;
        }
    }

    #[inline]
    fn add_section(&mut self, section: &Section<P>) {
        self.add_edge(
            &section.b_left,
            &section.a_left,
            #[cfg(feature = "variable_stroke_debug")]
            VariableStrokeDebugEdgeKind::SectionBoundary,
        );
        self.add_edge(
            &section.a_right,
            &section.b_right,
            #[cfg(feature = "variable_stroke_debug")]
            VariableStrokeDebugEdgeKind::SectionBoundary,
        );
    }

    fn add_join(&mut self, prev: &Section<P>, next: &Section<P>, angle: P::Scalar) -> usize {
        let prev_center = self.adapter.float_to_int(&prev.b);
        let next_center = self.adapter.float_to_int(&next.a);
        if prev_center != next_center {
            // A non-drawable section between these sections was filtered out. They belong to
            // separate chains, so close both chains instead of building an arc between centers.
            self.add_end_cap(prev, Cap::Butt, angle);
            self.add_start_cap(next, Cap::Butt, angle);
            return 0;
        }

        let prev_a_left = self.adapter.float_to_int(&prev.a_left);
        let prev_b_left = self.adapter.float_to_int(&prev.b_left);
        let prev_a_right = self.adapter.float_to_int(&prev.a_right);
        let prev_b_right = self.adapter.float_to_int(&prev.b_right);
        let next_a_left = self.adapter.float_to_int(&next.a_left);
        let next_b_left = self.adapter.float_to_int(&next.b_left);
        let next_a_right = self.adapter.float_to_int(&next.a_right);
        let next_b_right = self.adapter.float_to_int(&next.b_right);

        let prev_left = prev_b_left - prev_a_left;
        let prev_right = prev_b_right - prev_a_right;
        let next_left = next_b_left - next_a_left;
        let next_right = next_b_right - next_a_right;

        let mut arc_count = 0;
        let left_cross = next_left.cross_product(prev_left);

        let right_cross = prev_right.cross_product(next_right);

        let prev_a = self.adapter.float_to_int(&prev.a);
        let prev_b = prev_center;
        let next_a = self.adapter.float_to_int(&next.a);
        let next_b = self.adapter.float_to_int(&next.b);

        let prev_middle = prev_b - prev_a;
        let next_middle = next_b - next_a;

        let middle_cross = prev_middle.cross_product(next_middle);

        let left_arc = left_cross > I::Wide::ZERO || middle_cross < I::Wide::ZERO;
        let right_arc = right_cross > I::Wide::ZERO || middle_cross >= I::Wide::ZERO;

        if left_arc {
            arc_count += self.add_arc_ccw(
                &prev.b,
                &next.a_left,
                &prev.b_left,
                angle,
                ArcSweep::Minor,
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::JoinArc,
            ) as usize;
        } else {
            self.add_edge(
                &next.a_left,
                &prev.b_left,
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::JoinClosure,
            );
        }

        if right_arc {
            arc_count += self.add_arc_ccw(
                &prev.b,
                &prev.b_right,
                &next.a_right,
                angle,
                ArcSweep::Major,
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::JoinArc,
            ) as usize;
        } else {
            self.add_edge(
                &prev.b_right,
                &next.a_right,
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::JoinClosure,
            );
        }

        arc_count
    }

    fn add_start_cap(&mut self, section: &Section<P>, cap: Cap, angle: P::Scalar) {
        match cap {
            Cap::Butt => self.add_edge(
                &section.a_left,
                &section.a_right,
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::CapClosure,
            ),
            Cap::Round => {
                let sweep = if section.radius_trend == RadiusTrend::Decreasing {
                    ArcSweep::Major
                } else {
                    ArcSweep::Minor
                };
                self.add_arc_ccw(
                    &section.a,
                    &section.a_left,
                    &section.a_right,
                    angle,
                    sweep,
                    #[cfg(feature = "variable_stroke_debug")]
                    VariableStrokeDebugEdgeKind::CapArc,
                );
            }
        }
    }

    fn add_end_cap(&mut self, section: &Section<P>, cap: Cap, angle: P::Scalar) {
        match cap {
            Cap::Butt => self.add_edge(
                &section.b_right,
                &section.b_left,
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::CapClosure,
            ),
            Cap::Round => {
                let sweep = if section.radius_trend == RadiusTrend::Increasing {
                    ArcSweep::Major
                } else {
                    ArcSweep::Minor
                };
                self.add_arc_ccw(
                    &section.b,
                    &section.b_right,
                    &section.b_left,
                    angle,
                    sweep,
                    #[cfg(feature = "variable_stroke_debug")]
                    VariableStrokeDebugEdgeKind::CapArc,
                );
            }
        }
    }

    fn arc_sweep_ccw(&self, center: &P, from: &P, to: &P, aligned_sweep: ArcSweep) -> ArcSweep {
        let center = self.adapter.float_to_int(center);
        let from_vector = self.adapter.float_to_int(from) - center;
        let to_vector = self.adapter.float_to_int(to) - center;
        let cross = from_vector.cross_product(to_vector);

        if cross > I::Wide::ZERO {
            ArcSweep::Minor
        } else if cross < I::Wide::ZERO {
            ArcSweep::Major
        } else if from_vector.dot_product(to_vector) < I::Wide::ZERO {
            // Both choices describe the same half-circle.
            ArcSweep::Minor
        } else {
            // Coincident directions can mean either a collapsed minor arc or a full major arc.
            aligned_sweep
        }
    }

    fn add_arc_ccw(
        &mut self,
        center: &P,
        from: &P,
        to: &P,
        angle: P::Scalar,
        aligned_sweep: ArcSweep,
        #[cfg(feature = "variable_stroke_debug")] edge_kind: VariableStrokeDebugEdgeKind,
    ) -> bool {
        let sweep = self.arc_sweep_ccw(center, from, to, aligned_sweep);
        if sweep == ArcSweep::Minor && self.adapter.float_to_int(from) == self.adapter.float_to_int(to) {
            return false;
        }

        let from_point = *from;
        let to_point = *to;
        let from_vector = FloatPointMath::sub(&from_point, center);
        let from_unit = FloatPointMath::normalize(&from_vector);
        let to_unit = FloatPointMath::normalize(&FloatPointMath::sub(&to_point, center));
        let dot = FloatPointMath::dot_product(&from_unit, &to_unit)
            .max(-P::Scalar::ONE)
            .min(P::Scalar::ONE);
        let base = dot.acos();
        let sweep = match sweep {
            ArcSweep::Minor => base,
            ArcSweep::Major => P::Scalar::from_float(2.0 * PI) - base,
        };
        let count = (sweep / angle).to_usize().saturating_add(1).clamp(1, 1024);
        let rotator = Rotator::with_angle(sweep / P::Scalar::from_usize(count));

        let mut vector = from_vector;
        let mut a = from_point;
        for i in 1..=count {
            let b = if i == count {
                to_point
            } else {
                vector = rotator.rotate(&vector);
                FloatPointMath::add(center, &vector)
            };
            #[cfg(not(feature = "variable_stroke_debug"))]
            self.add_edge(&a, &b);
            #[cfg(feature = "variable_stroke_debug")]
            self.add_edge(&a, &b, edge_kind);
            a = b;
        }

        true
    }

    #[inline]
    fn add_edge(
        &mut self,
        a: &P,
        b: &P,
        #[cfg(feature = "variable_stroke_debug")] kind: VariableStrokeDebugEdgeKind,
    ) {
        let a = self.adapter.float_to_int(a);
        let b = self.adapter.float_to_int(b);
        if a != b {
            #[cfg(feature = "variable_stroke_debug")]
            if let Some(debug_edges) = self.debug_edges.as_mut() {
                debug_edges.push(VariableStrokeDebugEdge {
                    a: self.adapter.int_to_float(&a),
                    b: self.adapter.int_to_float(&b),
                    kind,
                    path_index: self.debug_path_index,
                    order: debug_edges.len(),
                });
            }
            self.segments.push(Segment::subject(a, b));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArcSweep, Cap, SegmentBuilder, SubSegment, VariableStrokeBuilder};
    #[cfg(feature = "variable_stroke_debug")]
    use crate::mesh::variable_stroke::VariableStrokeDebugEdgeKind;
    use crate::mesh::variable_stroke::offset::VariableStrokeOffset;
    use crate::mesh::variable_stroke::section::Section;
    use crate::mesh::variable_stroke::style::{StrokeVertex, VariableStrokeStyle};
    use crate::segm::boolean::ShapeCountBoolean;
    use crate::segm::segment::Segment;
    use alloc::vec;
    use alloc::vec::Vec;
    use i_float::adapter::FloatPointAdapter;
    use i_float::float::rect::FloatRect;

    fn adapter() -> FloatPointAdapter<[f64; 2], i32> {
        FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 1.0)
    }

    #[test]
    fn empty_path_does_not_create_subsegments_or_edges() {
        let path: [StrokeVertex<[f64; 2]>; 0] = [];
        let adapter = adapter();
        let builder = VariableStrokeBuilder::new(VariableStrokeStyle::new());
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();

        assert!(VariableStrokeBuilder::<f64>::find_subsegments(&path, &adapter).is_empty());
        builder.build(&path, &adapter, &mut segments);

        assert!(segments.is_empty());
    }

    #[test]
    fn single_round_vertex_builds_a_circle() {
        let path = [StrokeVertex::new([0.0, 0.0], 4.0)];
        let adapter = adapter();
        let builder = VariableStrokeBuilder::new(VariableStrokeStyle::new());
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();

        builder.build(&path, &adapter, &mut segments);

        assert!(!segments.is_empty());
    }

    #[test]
    fn covered_break_uses_butt_on_smaller_side() {
        let path = [
            StrokeVertex::new([-20.0, 0.0], 4.0),
            StrokeVertex::new([0.0, 0.0], 4.0),
            StrokeVertex::new([2.0, 0.0], 20.0),
            StrokeVertex::new([22.0, 0.0], 20.0),
        ];
        let subsegments = VariableStrokeBuilder::<f64>::find_subsegments(&path, &adapter());
        assert_eq!(subsegments.len(), 2);
        assert_eq!(subsegments[0].start, 0);
        assert_eq!(subsegments[0].end, 1);
        assert_eq!(subsegments[0].end_cap, Cap::Butt);
        assert_eq!(subsegments[1].start, 2);
        assert_eq!(subsegments[1].end, 3);
        assert_eq!(subsegments[1].start_cap, Cap::Round);
    }

    #[test]
    fn reverse_covered_break_uses_butt_on_smaller_side() {
        let path = [
            StrokeVertex::new([-20.0, 0.0], 20.0),
            StrokeVertex::new([0.0, 0.0], 20.0),
            StrokeVertex::new([2.0, 0.0], 4.0),
            StrokeVertex::new([22.0, 0.0], 4.0),
        ];
        let subsegments = VariableStrokeBuilder::<f64>::find_subsegments(&path, &adapter());
        assert_eq!(subsegments.len(), 2);
        assert_eq!(subsegments[0].start, 0);
        assert_eq!(subsegments[0].end, 1);
        assert_eq!(subsegments[0].end_cap, Cap::Round);
        assert_eq!(subsegments[1].start, 2);
        assert_eq!(subsegments[1].end, 3);
        assert_eq!(subsegments[1].start_cap, Cap::Butt);
    }

    #[test]
    fn near_covered_sections_stay_in_one_subsegment() {
        let path = [
            StrokeVertex::new([0.0, 0.0], 6.0),
            StrokeVertex::new([7.57, 3.86], 18.0),
            StrokeVertex::new([19.2, 7.12], 42.0),
        ];
        let precise_adapter: FloatPointAdapter<[f64; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 100.0);
        let subsegments = VariableStrokeBuilder::<f64>::find_subsegments(&path, &precise_adapter);
        assert_eq!(subsegments.len(), 1);
        assert_eq!(subsegments[0].start, 0);
        assert_eq!(subsegments[0].end, 2);
        assert_eq!(subsegments[0].start_cap, Cap::Round);
        assert_eq!(subsegments[0].end_cap, Cap::Round);
    }

    #[test]
    fn trapezoid_cover_requires_a_larger_source_circle() {
        let equal_a = StrokeVertex::new([0.0, 0.0], 20.0);
        let equal_b = StrokeVertex::new([100.0, 0.0], 20.0);
        let c = StrokeVertex::new([50.0, 0.0], 20.0);
        let larger_a = StrokeVertex::new([0.0, 0.0], 40.0);
        let larger_b = StrokeVertex::new([100.0, 0.0], 40.0);

        assert!(!VariableStrokeBuilder::<f64>::circle_is_covered_by_section(
            &equal_a,
            &equal_b,
            &c,
            &adapter(),
        ));
        assert!(VariableStrokeBuilder::<f64>::circle_is_covered_by_section(
            &larger_a,
            &larger_b,
            &c,
            &adapter(),
        ));
    }

    #[test]
    fn zero_length_butt_subsegment_is_not_drawn() {
        let path = [
            StrokeVertex::new([-2.0, 0.0], 20.0),
            StrokeVertex::new([0.0, 0.0], 2.0),
            StrokeVertex::new([2.0, 0.0], 20.0),
        ];
        let adapter = adapter();
        let subsegments = VariableStrokeBuilder::<f64>::find_subsegments(&path, &adapter);

        assert_eq!(subsegments.len(), 3);
        assert_eq!(
            subsegments[1],
            SubSegment {
                start: 1,
                end: 1,
                start_cap: Cap::Butt,
                end_cap: Cap::Butt,
            }
        );

        let builder = VariableStrokeBuilder::new(VariableStrokeStyle::new());
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &adapter,
            segments: &mut segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };
        builder.add_subsegment(&subsegments[1], &path, &mut output);

        assert!(segments.is_empty());
    }

    #[test]
    fn join_keeps_all_tangent_contacts() {
        let path = [
            StrokeVertex::new([-20.0, 0.0], 8.0),
            StrokeVertex::new([0.0, 0.0], 20.0),
            StrokeVertex::new([15.0, 18.0], 12.0),
        ];
        let adapter: FloatPointAdapter<[f64; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 1_000.0);
        let previous = Section::try_new(&path[0], &path[1], &adapter).unwrap();
        let next = Section::try_new(&path[1], &path[2], &adapter).unwrap();
        let contacts = [previous.b_left, previous.b_right, next.a_left, next.a_right];
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &adapter,
            segments: &mut segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        output.add_join(&previous, &next, core::f64::consts::FRAC_PI_4);

        for contact in contacts {
            let point = adapter.float_to_int(&contact);
            assert!(
                segments
                    .iter()
                    .any(|segment| segment.x_segment.a == point || segment.x_segment.b == point),
                "missing tangent contact {point:?}"
            );
        }
    }

    fn join_arc_count(path: [StrokeVertex<[f64; 2]>; 3]) -> usize {
        let adapter: FloatPointAdapter<[f64; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 1_000.0);
        let previous = Section::try_new(&path[0], &path[1], &adapter).unwrap();
        let next = Section::try_new(&path[1], &path[2], &adapter).unwrap();
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &adapter,
            segments: &mut segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        output.add_join(&previous, &next, core::f64::consts::FRAC_PI_4)
    }

    #[test]
    fn width_peak_builds_two_join_arcs() {
        let path = [
            StrokeVertex::new([-10.0, 0.0], 4.0),
            StrokeVertex::new([0.0, 0.0], 10.0),
            StrokeVertex::new([10.0, 0.0], 4.0),
        ];

        assert_eq!(join_arc_count(path), 2);
    }

    #[test]
    fn ordinary_turn_builds_one_join_arc() {
        let path = [
            StrokeVertex::new([-10.0, 0.0], 4.0),
            StrokeVertex::new([0.0, 0.0], 4.0),
            StrokeVertex::new([0.0, 10.0], 4.0),
        ];

        assert_eq!(join_arc_count(path), 1);
    }
    #[test]
    fn coarse_arc_is_one_exact_contact_segment() {
        let adapter: FloatPointAdapter<[f64; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 1_000.0);
        let center = [0.0, 0.0];
        let from = [10.0, 0.0];
        let sweep = 0.1_f64;
        let to = [10.0 * sweep.cos(), 10.0 * sweep.sin()];
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &adapter,
            segments: &mut segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        assert!(output.add_arc_ccw(
            &center,
            &from,
            &to,
            core::f64::consts::FRAC_PI_4,
            ArcSweep::Minor,
            #[cfg(feature = "variable_stroke_debug")]
            VariableStrokeDebugEdgeKind::JoinArc,
        ));
        assert_eq!(segments.len(), 1);
        let edge = segments[0].x_segment;
        let from = adapter.float_to_int(&from);
        let to = adapter.float_to_int(&to);
        assert!(edge.a == from || edge.b == from);
        assert!(edge.a == to || edge.b == to);
    }

    #[test]
    fn coincident_contacts_keep_topological_major_arc() {
        let adapter: FloatPointAdapter<[f64; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-100.0, 100.0, -100.0, 100.0), 1_000.0);
        let center = [0.0, 0.0];
        let contact = [10.0, 0.0];
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &adapter,
            segments: &mut segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        assert!(output.add_arc_ccw(
            &center,
            &contact,
            &contact,
            core::f64::consts::FRAC_PI_4,
            ArcSweep::Major,
            #[cfg(feature = "variable_stroke_debug")]
            VariableStrokeDebugEdgeKind::JoinArc,
        ));
        assert!(segments.len() >= 3);
    }

    #[test]
    fn coarse_missed_arc_0() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 8.0_f32),
            StrokeVertex::new([60.0_f32, 0.0_f32], 20.0_f32),
            StrokeVertex::new([5.0_f32, 8.0_f32], 10.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.17999999_f32);
        let result = paths.variable_stroke(style);

        assert!(!result.is_empty());
    }

    #[test]
    fn coarse_missed_arc_1() {
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 8.0_f32),
            StrokeVertex::new([60.0_f32, 0.0_f32], 20.0_f32),
            StrokeVertex::new([60.0_f32, -60.0_f32], 10.0_f32),
        ]];
        let style = VariableStrokeStyle::new().round_angle(0.17999999_f32);
        let result = paths.variable_stroke(style);

        assert!(!result.is_empty());
    }

    #[test]
    fn missed_arc_1() {
        let paths = vec![vec![
            StrokeVertex::new([-86.0_f32, 2.0_f32], 10.0_f32),
            StrokeVertex::new([100.0_f32, 0.0_f32], 100.0_f32),
            StrokeVertex::new([99.0_f32, -45.0_f32], 10.0_f32),
        ]];
        let precise_adapter: FloatPointAdapter<[f32; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-200.0, 200.0, -200.0, 200.0), 1_000.0);
        let previous = Section::try_new(&paths[0][0], &paths[0][1], &precise_adapter).unwrap();
        let next = Section::try_new(&paths[0][1], &paths[0][2], &precise_adapter).unwrap();
        let mut segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &precise_adapter,
            segments: &mut segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        assert_eq!(
            output.add_join(&previous, &next, 0.17999999_f32),
            2,
            "the wide reversal exposes both join arcs"
        );

        let style = VariableStrokeStyle::new().round_angle(0.17999999_f32);
        let result = paths.variable_stroke(style);

        assert!(!result.is_empty());
    }

    #[test]
    fn missed_arc_2() {
        // Dynamic Width repro: test=11 width_scale=2.2
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 22.0_f32),
            StrokeVertex::new([100.0_f32, 0.0_f32], 220.0_f32),
            StrokeVertex::new([100.0_f32, -100.0_f32], 22.0_f32),
        ]];
        let precise_adapter: FloatPointAdapter<[f32; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-250.0, 250.0, -250.0, 250.0), 1_000.0);
        let first = Section::try_new(&paths[0][0], &paths[0][1], &precise_adapter).unwrap();
        let second = Section::try_new(&paths[0][1], &paths[0][2], &precise_adapter).unwrap();
        let mut join_segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &precise_adapter,
            segments: &mut join_segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        assert_eq!(
            output.arc_sweep_ccw(&first.b, &second.a_left, &first.b_left, ArcSweep::Minor,),
            ArcSweep::Major,
            "the left CCW join crosses the major radial sector"
        );
        assert_eq!(output.add_join(&first, &second, 0.21_f32), 2);
        assert!(join_segments.len() > 20, "the major join arc was not built");

        let style = VariableStrokeStyle::new().round_angle(0.21_f32);
        let result = paths.variable_stroke(style);

        assert!(!result.is_empty());
    }

    #[test]
    fn moderate_width_peak_builds_one_arc() {
        // Dynamic Width repro: test=11 width_scale=0.88
        let paths = vec![vec![
            StrokeVertex::new([0.0_f32, 0.0_f32], 8.8_f32),
            StrokeVertex::new([100.0_f32, 0.0_f32], 88.0_f32),
            StrokeVertex::new([100.0_f32, -100.0_f32], 8.8_f32),
        ]];
        let precise_adapter: FloatPointAdapter<[f32; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-150.0, 150.0, -150.0, 150.0), 1_000.0);
        let first = Section::try_new(&paths[0][0], &paths[0][1], &precise_adapter).unwrap();
        let second = Section::try_new(&paths[0][1], &paths[0][2], &precise_adapter).unwrap();
        let mut join_segments = Vec::<Segment<ShapeCountBoolean, i32>>::new();
        let mut output = SegmentBuilder {
            adapter: &precise_adapter,
            segments: &mut join_segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        assert_eq!(output.add_join(&first, &second, 0.615_f32), 1);

        let style = VariableStrokeStyle::new().round_angle(0.615_f32);
        let result = paths.variable_stroke(style);

        assert!(!result.is_empty());
    }

    #[test]
    fn middle_left_reversal_closes_both_sections() {
        let paths = vec![vec![
            StrokeVertex::new([-86.0_f32, 2.0_f32], 21.800001_f32),
            StrokeVertex::new([100.0_f32, 0.0_f32], 218.0_f32),
            StrokeVertex::new([-20.699999_f32, -16.029999_f32], 21.800001_f32),
        ]];
        let precise_adapter: FloatPointAdapter<[f32; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-250.0, 250.0, -250.0, 250.0), 1_000.0);
        let subsegments = VariableStrokeBuilder::<f32>::find_subsegments(&paths[0], &precise_adapter);

        assert_eq!(subsegments.len(), 2);
        assert_eq!(subsegments[0].start, 0);
        assert_eq!(subsegments[0].end, 1);
        assert_eq!(subsegments[0].end_cap, Cap::Round);
        assert_eq!(subsegments[1].start, 1);
        assert_eq!(subsegments[1].end, 2);
        assert_eq!(subsegments[1].start_cap, Cap::Butt);
        assert_eq!(subsegments[1].end_cap, Cap::Butt);

        let result = paths.variable_stroke(VariableStrokeStyle::new().round_angle(0.75_f32));
        let has_tooth = result.iter().flatten().flatten().any(|point| {
            let dx = point[0] - 100.0;
            let dy = point[1];
            point[0] > 20.0 && point[1] < -70.0 && dx * dx + dy * dy < 108.5 * 108.5
        });

        assert_eq!(result.len(), 1);
        assert!(!has_tooth);
    }

    #[test]
    fn middle_right_reversal_closes_both_sections() {
        let paths = vec![vec![
            StrokeVertex::new([-86.0_f32, -2.0_f32], 21.800001_f32),
            StrokeVertex::new([100.0_f32, 0.0_f32], 218.0_f32),
            StrokeVertex::new([-20.699999_f32, 16.029999_f32], 21.800001_f32),
        ]];
        let precise_adapter: FloatPointAdapter<[f32; 2], i32> =
            FloatPointAdapter::with_scale(FloatRect::new(-250.0, 250.0, -250.0, 250.0), 1_000.0);
        let subsegments = VariableStrokeBuilder::<f32>::find_subsegments(&paths[0], &precise_adapter);

        assert_eq!(subsegments.len(), 2);
        assert_eq!(subsegments[0].end_cap, Cap::Round);
        assert_eq!(subsegments[1].start_cap, Cap::Butt);
        assert_eq!(subsegments[1].end_cap, Cap::Butt);

        let result = paths.variable_stroke(VariableStrokeStyle::new().round_angle(0.75_f32));
        let has_tooth = result.iter().flatten().flatten().any(|point| {
            let dx = point[0] - 100.0;
            let dy = point[1];
            point[0] > 20.0 && point[1] > 70.0 && dx * dx + dy * dy < 108.5 * 108.5
        });

        assert_eq!(result.len(), 1);
        assert!(!has_tooth);
    }
}
