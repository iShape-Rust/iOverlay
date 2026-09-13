#[cfg(feature = "variable_stroke_debug")]
use super::debug::{IntVariableStrokeDebugEdge, VariableStrokeDebugEdgeKind};
use super::{
    section::{RadiusTrend, Section},
    style::{IntStrokeVertex, IntVariableStrokeStyle},
};
use crate::mesh::{
    int::{
        arc::{ArcBuilder, ArcDirection},
        math::{direction, point, scaled_point},
    },
    subject::SubjectSegments,
};
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::{
    angle::Angle,
    number::{int::IntNumber, uint::UIntNumber, wide_int::WideIntNumber},
    point::IntPoint,
};

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

pub(super) struct VariableStrokeBuilder<I: IntNumber> {
    arc: ArcBuilder<I>,
}

impl<I: IntNumber> VariableStrokeBuilder<I> {
    pub(super) fn new(style: IntVariableStrokeStyle) -> Self {
        Self {
            arc: ArcBuilder::new(style.arc),
        }
    }

    pub(super) fn build(
        &mut self,
        path: &[IntStrokeVertex<I>],
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
    ) {
        if path.is_empty() {
            return;
        }

        let subsegments = Self::find_subsegments(path);
        let mut output = SegmentBuilder {
            arc: &mut self.arc,
            segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges: None,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: 0,
        };

        for subsegment in subsegments.iter() {
            Self::add_subsegment(subsegment, path, &mut output);
        }
    }

    #[cfg(feature = "variable_stroke_debug")]
    pub(super) fn build_debug(
        &mut self,
        path: &[IntStrokeVertex<I>],
        path_index: usize,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
        debug_edges: &mut Vec<IntVariableStrokeDebugEdge<I>>,
    ) {
        if path.is_empty() {
            return;
        }

        let subsegments = Self::find_subsegments(path);
        let mut output = SegmentBuilder {
            arc: &mut self.arc,
            segments,
            debug_edges: Some(debug_edges),
            debug_path_index: path_index,
        };

        for subsegment in subsegments.iter() {
            Self::add_subsegment(subsegment, path, &mut output);
        }
    }

    fn add_subsegment(subsegment: &SubSegment, path: &[IntStrokeVertex<I>], output: &mut SegmentBuilder<I>) {
        if subsegment.start == subsegment.end {
            if subsegment.start_cap != Cap::Butt || subsegment.end_cap != Cap::Butt {
                let vertex = &path[subsegment.start];
                output.add_circle(
                    &vertex.point,
                    vertex.radius(),
                    #[cfg(feature = "variable_stroke_debug")]
                    VariableStrokeDebugEdgeKind::CircleArc,
                );
            }
            return;
        }

        let mut sections = (subsegment.start..subsegment.end)
            .filter_map(|index| Section::try_new(&path[index], &path[index + 1]));
        let Some(mut previous) = sections.next() else {
            return;
        };

        output.add_section(&previous);
        output.add_start_cap(&previous, subsegment.start_cap);

        for section in sections {
            output.add_section(&section);
            output.add_join(&previous, &section);
            previous = section;
        }

        output.add_end_cap(&previous, subsegment.end_cap);
    }

    fn find_subsegments(path: &[IntStrokeVertex<I>]) -> Vec<SubSegment> {
        if path.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut start = 0;
        let mut start_cap = Cap::Round;
        let mut final_end_cap = Cap::Round;

        for (index, pair) in path.windows(2).enumerate() {
            final_end_cap = Cap::Round;

            if let Some((end_cap, next_start_cap)) = Self::break_caps(&pair[0], &pair[1]) {
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

            if index > 0 && Self::circle_is_covered_by_section(&path[index - 1], &pair[0], &pair[1]) {
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

    fn break_caps(a: &IntStrokeVertex<I>, b: &IntStrokeVertex<I>) -> Option<(Cap, Cap)> {
        let int_a = a.point;
        let int_b = b.point;
        let a_radius = a.radius();
        let b_radius = b.radius();
        let radius_delta = a_radius.to_wide() - b_radius.to_wide();
        let distance_sqr = (int_b - int_a).sqr_length();

        if (radius_delta * radius_delta).to_uint() < distance_sqr {
            return None;
        }

        if a_radius >= b_radius {
            Some((Cap::Round, Cap::Butt))
        } else {
            Some((Cap::Butt, Cap::Round))
        }
    }

    fn circle_is_covered_by_section(
        a: &IntStrokeVertex<I>,
        b: &IntStrokeVertex<I>,
        c: &IntStrokeVertex<I>,
    ) -> bool {
        let a_radius = a.radius();
        let b_radius = b.radius();
        let c_radius = c.radius();
        if a_radius.max(b_radius) <= c_radius {
            return false;
        }

        let Some(section) = Section::try_new(a, b) else {
            return false;
        };

        let points = [section.a_left, section.b_left, section.b_right, section.a_right];
        let center = c.point;
        let radius = c_radius.to_wide().to_uint();
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
                length += I::WideUInt::ONE;
            }
            if interior_distance.to_uint() < radius * length {
                return false;
            }
        }

        true
    }
}

struct SegmentBuilder<'a, I: IntNumber> {
    arc: &'a mut ArcBuilder<I>,
    segments: &'a mut Vec<Segment<ShapeCountBoolean, I>>,
    #[cfg(feature = "variable_stroke_debug")]
    debug_edges: Option<&'a mut Vec<IntVariableStrokeDebugEdge<I>>>,
    #[cfg(feature = "variable_stroke_debug")]
    debug_path_index: usize,
}

impl<I: IntNumber> SegmentBuilder<'_, I> {
    fn add_circle(
        &mut self,
        center: &IntPoint<I>,
        radius: I,
        #[cfg(feature = "variable_stroke_debug")] kind: VariableStrokeDebugEdgeKind,
    ) {
        if radius <= I::ONE {
            return;
        }
        let right = point(*center, radius.to_wide(), I::Wide::ZERO);
        let left = point(*center, -radius.to_wide(), I::Wide::ZERO);
        self.arc_edges(
            center,
            &right,
            &left,
            radius,
            #[cfg(feature = "variable_stroke_debug")]
            kind,
        );
        self.arc_edges(
            center,
            &left,
            &right,
            radius,
            #[cfg(feature = "variable_stroke_debug")]
            kind,
        );
    }

    #[inline]
    fn add_section(&mut self, section: &Section<I>) {
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

    fn add_join(&mut self, prev: &Section<I>, next: &Section<I>) -> usize {
        let prev_center = prev.b;
        let next_center = next.a;
        if prev_center != next_center {
            // A non-drawable section between these sections was filtered out. They belong to
            // separate chains, so close both chains instead of building an arc between centers.
            self.add_end_cap(prev, Cap::Butt);
            self.add_start_cap(next, Cap::Butt);
            return 0;
        }

        let prev_a_left = prev.a_left;
        let prev_b_left = prev.b_left;
        let prev_a_right = prev.a_right;
        let prev_b_right = prev.b_right;
        let next_a_left = next.a_left;
        let next_b_left = next.b_left;
        let next_a_right = next.a_right;
        let next_b_right = next.b_right;

        let prev_left = prev_b_left - prev_a_left;
        let prev_right = prev_b_right - prev_a_right;
        let next_left = next_b_left - next_a_left;
        let next_right = next_b_right - next_a_right;

        let mut arc_count = 0;
        let left_cross = next_left.cross_product(prev_left);

        let right_cross = prev_right.cross_product(next_right);

        let prev_a = prev.a;
        let prev_b = prev_center;
        let next_a = next.a;
        let next_b = next.b;

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

    fn add_start_cap(&mut self, section: &Section<I>, cap: Cap) {
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
                    sweep,
                    #[cfg(feature = "variable_stroke_debug")]
                    VariableStrokeDebugEdgeKind::CapArc,
                );
            }
        }
    }

    fn add_end_cap(&mut self, section: &Section<I>, cap: Cap) {
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
                    sweep,
                    #[cfg(feature = "variable_stroke_debug")]
                    VariableStrokeDebugEdgeKind::CapArc,
                );
            }
        }
    }

    fn arc_sweep_ccw(
        &self,
        center: &IntPoint<I>,
        from: &IntPoint<I>,
        to: &IntPoint<I>,
        aligned_sweep: ArcSweep,
    ) -> ArcSweep {
        let center = *center;
        let from_vector = *from - center;
        let to_vector = *to - center;
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
        center: &IntPoint<I>,
        from: &IntPoint<I>,
        to: &IntPoint<I>,
        aligned_sweep: ArcSweep,
        #[cfg(feature = "variable_stroke_debug")] edge_kind: VariableStrokeDebugEdgeKind,
    ) -> bool {
        let sweep = self.arc_sweep_ccw(center, from, to, aligned_sweep);
        if sweep == ArcSweep::Minor && *from == *to {
            return false;
        }

        let Some(from_unit) = direction(*from - *center) else {
            return false;
        };
        let Some(to_unit) = direction(*to - *center) else {
            return false;
        };
        let from_vector = *from - *center;
        let radius = I::from_uint(from_vector.sqr_length().isqrt());
        if Angle::between(from_unit, to_unit).bits() == 0 && sweep == ArcSweep::Major {
            self.add_circle(
                center,
                radius,
                #[cfg(feature = "variable_stroke_debug")]
                edge_kind,
            );
            self.add_edge(
                from,
                to,
                #[cfg(feature = "variable_stroke_debug")]
                edge_kind,
            );
        } else {
            self.arc_edges(
                center,
                from,
                to,
                radius,
                #[cfg(feature = "variable_stroke_debug")]
                edge_kind,
            );
        }
        true
    }

    fn arc_edges(
        &mut self,
        center: &IntPoint<I>,
        from: &IntPoint<I>,
        to: &IntPoint<I>,
        radius: I,
        #[cfg(feature = "variable_stroke_debug")] kind: VariableStrokeDebugEdgeKind,
    ) {
        let Some(a) = direction(*from - *center) else {
            return;
        };
        let Some(b) = direction(*to - *center) else {
            return;
        };
        let directions = self.arc.build(a, b, ArcDirection::Counterclockwise);
        let mut previous = *from;
        // Split borrows: the reusable arc buffer stays borrowed while emitting.
        for &dir in directions {
            let next = scaled_point(*center, dir, radius);
            if previous != next {
                #[cfg(feature = "variable_stroke_debug")]
                if let Some(edges) = self.debug_edges.as_mut() {
                    edges.push(IntVariableStrokeDebugEdge {
                        a: previous,
                        b: next,
                        kind,
                        path_index: self.debug_path_index,
                        order: edges.len(),
                    });
                }
                self.segments.push_non_degenerate(previous, next);
            }
            previous = next;
        }
        self.add_edge(
            &previous,
            to,
            #[cfg(feature = "variable_stroke_debug")]
            kind,
        );
    }

    #[inline]
    fn add_edge(
        &mut self,
        a: &IntPoint<I>,
        b: &IntPoint<I>,
        #[cfg(feature = "variable_stroke_debug")] kind: VariableStrokeDebugEdgeKind,
    ) {
        let a = *a;
        let b = *b;
        if a != b {
            #[cfg(feature = "variable_stroke_debug")]
            if let Some(debug_edges) = self.debug_edges.as_mut() {
                debug_edges.push(IntVariableStrokeDebugEdge {
                    a,
                    b,
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
#[path = "tests.rs"]
mod tests;
