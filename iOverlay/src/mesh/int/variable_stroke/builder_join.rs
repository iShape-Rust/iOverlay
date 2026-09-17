#[cfg(feature = "variable_stroke_debug")]
use super::debug::{IntVariableStrokeDebugEdge, VariableStrokeDebugEdgeKind};
use super::section::{RadiusTrend, Section};
use crate::mesh::{
    int::{
        arc::{ArcDirection, ArcMath},
        math::{backend::MeshMath, integer::IntegerMath, point, scaled_point},
    },
    subject::SubjectSegments,
};
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::{
    number::{int::IntNumber, uint::UIntNumber, wide_int::WideIntNumber},
    point::IntPoint,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Cap {
    Butt,
    Round,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ArcSweep {
    Minor,
    Major,
}

pub(super) struct SegmentBuilder<'a, I: IntNumber, M: MeshMath<I> = IntegerMath> {
    pub(super) arc: &'a mut M::Arc,
    pub(super) segments: &'a mut Vec<Segment<ShapeCountBoolean, I>>,
    #[cfg(feature = "variable_stroke_debug")]
    pub(super) debug_edges: Option<&'a mut Vec<IntVariableStrokeDebugEdge<I>>>,
    #[cfg(feature = "variable_stroke_debug")]
    pub(super) debug_path_index: usize,
}

impl<I: IntNumber, M: MeshMath<I>> SegmentBuilder<'_, I, M> {
    pub(super) fn add_circle(
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
    pub(super) fn add_section(&mut self, section: &Section<I>) {
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

    pub(super) fn add_join(&mut self, prev: &Section<I>, next: &Section<I>) -> usize {
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

    pub(super) fn add_start_cap(&mut self, section: &Section<I>, cap: Cap) {
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

    pub(super) fn add_end_cap(&mut self, section: &Section<I>, cap: Cap) {
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

    pub(super) fn arc_sweep_ccw(
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

    pub(super) fn add_arc_ccw(
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

        let Some(from_unit) = M::normalize(*from - *center) else {
            return false;
        };
        let Some(to_unit) = M::normalize(*to - *center) else {
            return false;
        };
        let from_vector = *from - *center;
        let radius = I::from_uint(from_vector.sqr_length().isqrt());
        if M::angle_between(from_unit, to_unit).bits() == 0 && sweep == ArcSweep::Major {
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

    pub(super) fn arc_edges(
        &mut self,
        center: &IntPoint<I>,
        from: &IntPoint<I>,
        to: &IntPoint<I>,
        radius: I,
        #[cfg(feature = "variable_stroke_debug")] kind: VariableStrokeDebugEdgeKind,
    ) {
        let Some(a) = M::normalize(*from - *center) else {
            return;
        };
        let Some(b) = M::normalize(*to - *center) else {
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
    pub(super) fn add_edge(
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
