use crate::build::sweep::{FillHandler, FillStrategy, SweepRunner};
use crate::core::link::OverlayLink;
use crate::core::solver::Solver;
use crate::geom::end::End;
use crate::geom::id_point::IdPoint;
use crate::segm::segment::{NONE, Segment, SegmentFill};
use crate::segm::winding::WindingCount;
use alloc::vec::Vec;
use core::ops::ControlFlow;
use i_shape::util::reserve::Reserve;

pub(super) trait InclusionFilterStrategy {
    fn is_included(fill: SegmentFill) -> bool;
}

pub(crate) struct StoreFillsHandler<'a> {
    fills: &'a mut Vec<SegmentFill>,
}

impl<'a> StoreFillsHandler<'a> {
    #[inline]
    pub(crate) fn new(fills: &'a mut Vec<SegmentFill>) -> Self {
        Self { fills }
    }
}

impl FillHandler for StoreFillsHandler<'_> {
    type Output = ();

    #[inline(always)]
    fn handle(&mut self, index: usize, fill: SegmentFill) -> ControlFlow<()> {
        // fills is pre-allocated to segments.len() and index is guaranteed
        // to be in range by the sweep algorithm
        unsafe { *self.fills.get_unchecked_mut(index) = fill };
        ControlFlow::Continue(())
    }

    #[inline(always)]
    fn finalize(self) {}
}

pub(crate) trait GraphNode {
    fn with_indices(indices: &[usize]) -> Self;
}

pub(crate) struct GraphBuilder<C, N> {
    sweep_runner: SweepRunner<C>,
    pub(super) links: Vec<OverlayLink>,
    pub(super) nodes: Vec<N>,
    pub(super) fills: Vec<SegmentFill>,
    pub(super) ends: Vec<End>,
}

impl<C: WindingCount, N: GraphNode> GraphBuilder<C, N> {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            sweep_runner: SweepRunner::new(),
            links: Vec::new(),
            nodes: Vec::new(),
            fills: Vec::new(),
            ends: Vec::new(),
        }
    }

    #[inline]
    pub(super) fn build_fills_with_strategy<F: FillStrategy<C>>(
        &mut self,
        solver: &Solver,
        segments: &[Segment<C>],
    ) {
        self.fills.resize(segments.len(), NONE);
        self.sweep_runner
            .run::<F, _>(solver, segments, StoreFillsHandler::new(&mut self.fills));
    }

    #[inline]
    pub(super) fn build_links_by_filter<F: InclusionFilterStrategy>(&mut self, segments: &[Segment<C>]) {
        self.links.clear();
        self.links.reserve_capacity(segments.len());

        for (segment, &fill) in segments.iter().zip(&self.fills) {
            if !F::is_included(fill) {
                continue;
            }
            self.links.push(OverlayLink::new(
                IdPoint::new(0, segment.x_segment.a),
                IdPoint::new(0, segment.x_segment.b),
                fill,
            ));
        }
    }

    #[inline]
    pub(super) fn build_links_all(&mut self, segments: &[Segment<C>]) {
        self.links.clear();
        self.links.reserve_capacity(segments.len());

        for (segment, &fill) in segments.iter().zip(&self.fills) {
            self.links.push(OverlayLink::new(
                IdPoint::new(0, segment.x_segment.a),
                IdPoint::new(0, segment.x_segment.b),
                fill,
            ));
        }
    }
}
