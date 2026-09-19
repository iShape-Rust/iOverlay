// The original partition-then-build traversal, retained only as a regression oracle.
use super::*;
use i_float::int::number::{uint::UIntNumber, wide_int::WideIntNumber};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SubSegment {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) start_cap: Cap,
    pub(super) end_cap: Cap,
}

impl<I: IntNumber> VariableStrokeBuilder<I, IntegerMath> {
    pub(super) fn add_subsegment(
        subsegment: &SubSegment,
        path: &[IntStrokeVertex<I>],
        output: &mut SegmentBuilder<I>,
    ) {
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
            .filter_map(|index| Section::try_new::<IntegerMath>(&path[index], &path[index + 1]));
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

    pub(super) fn find_subsegments(path: &[IntStrokeVertex<I>]) -> Vec<SubSegment> {
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

    pub(super) fn break_caps(a: &IntStrokeVertex<I>, b: &IntStrokeVertex<I>) -> Option<(Cap, Cap)> {
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

    pub(super) fn circle_is_covered_by_section(
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

        let Some(section) = Section::try_new::<IntegerMath>(a, b) else {
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
