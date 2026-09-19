#[cfg(feature = "variable_stroke_debug")]
use super::debug::{IntVariableStrokeDebugEdge, VariableStrokeDebugEdgeKind};
use super::{
    builder_join::{Cap, SegmentBuilder},
    math::VariableStrokeMath,
    section::Section,
    style::{IntStrokeVertex, IntVariableStrokeStyle},
};
use crate::mesh::int::{arc::ArcMath, math::integer::IntegerMath};
use crate::segm::{boolean::ShapeCountBoolean, segment::Segment};
use alloc::vec::Vec;
use i_float::int::number::int::IntNumber;

pub(super) struct VariableStrokeBuilder<I: IntNumber, M: VariableStrokeMath<I> = IntegerMath> {
    arc: M::Arc,
}

impl<I: IntNumber, M: VariableStrokeMath<I>> VariableStrokeBuilder<I, M> {
    pub(super) fn new(style: IntVariableStrokeStyle) -> Self {
        Self {
            arc: M::Arc::new(style.arc),
        }
    }

    pub(super) fn build(
        &mut self,
        path: impl IntoIterator<Item = IntStrokeVertex<I>>,
        segments: &mut Vec<Segment<ShapeCountBoolean, I>>,
        #[cfg(feature = "variable_stroke_debug")] debug_edges: Option<
            &mut Vec<IntVariableStrokeDebugEdge<I>>,
        >,
        #[cfg(feature = "variable_stroke_debug")] path_index: usize,
    ) {
        let mut output = SegmentBuilder::<I, M> {
            arc: &mut self.arc,
            segments,
            #[cfg(feature = "variable_stroke_debug")]
            debug_edges,
            #[cfg(feature = "variable_stroke_debug")]
            debug_path_index: path_index,
        };
        let mut path = path.into_iter();
        let Some(mut a) = path.next() else {
            return;
        };
        let mut chain = Chain::new(a, Cap::Round);
        let mut previous_section: Option<Section<I>> = None;
        let mut previous_radius = I::ZERO;
        let mut end_cap = Cap::Round;

        for b in path {
            end_cap = Cap::Round;
            let section = Section::try_new::<M>(&a, &b);
            if let Some((end, start)) = break_caps(&a, &b) {
                chain.finish(end, &mut output);
                chain = Chain::new(b, start);
            } else {
                if previous_radius > b.radius()
                    && previous_section.is_some_and(|s| s.covers_circle(b.point, b.radius()))
                {
                    chain.finish(Cap::Round, &mut output);
                    chain = Chain::new(a, Cap::Butt);
                    end_cap = Cap::Butt;
                }
                chain.has_edge = true;
                if let Some(section) = section {
                    output.add_section(&section);
                    if let Some(previous) = chain.last {
                        output.add_join(&previous, &section);
                    } else {
                        output.add_start_cap(&section, chain.start_cap);
                    }
                    chain.last = Some(section);
                }
            }
            previous_section = section;
            previous_radius = a.radius().max(b.radius());
            a = b;
        }
        chain.finish(end_cap, &mut output);
    }
}

struct Chain<I: IntNumber> {
    start: IntStrokeVertex<I>,
    start_cap: Cap,
    last: Option<Section<I>>,
    has_edge: bool,
}

impl<I: IntNumber> Chain<I> {
    fn new(start: IntStrokeVertex<I>, start_cap: Cap) -> Self {
        Self {
            start,
            start_cap,
            last: None,
            has_edge: false,
        }
    }

    fn finish<M: VariableStrokeMath<I>>(&self, end_cap: Cap, output: &mut SegmentBuilder<I, M>) {
        if let Some(last) = self.last {
            output.add_end_cap(&last, end_cap);
        } else if !self.has_edge && (self.start_cap != Cap::Butt || end_cap != Cap::Butt) {
            output.add_circle(
                &self.start.point,
                self.start.radius(),
                #[cfg(feature = "variable_stroke_debug")]
                VariableStrokeDebugEdgeKind::CircleArc,
            );
        }
    }
}

fn break_caps<I: IntNumber>(a: &IntStrokeVertex<I>, b: &IntStrokeVertex<I>) -> Option<(Cap, Cap)> {
    use i_float::int::number::wide_int::WideIntNumber;
    let ra = a.radius();
    let rb = b.radius();
    let delta = ra.to_wide() - rb.to_wide();
    if (delta * delta).to_uint() < (b.point - a.point).sqr_length() {
        None
    } else if ra >= rb {
        Some((Cap::Round, Cap::Butt))
    } else {
        Some((Cap::Butt, Cap::Round))
    }
}

#[cfg(test)]
#[path = "tests_reference.rs"]
mod reference;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
