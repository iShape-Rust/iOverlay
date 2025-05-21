use alloc::vec::Vec;
use crate::core::solver::Solver;
use crate::geom::end::End;
use crate::geom::v_segment::VSegment;
use crate::segm::segment::{NONE, Segment, SegmentFill};
use crate::segm::winding::WindingCount;
use crate::util::log::Int;
use i_float::triangle::Triangle;
use i_key_sort::sort::layout::BinStore;
use i_tree::key::exp::KeyExpCollection;
use i_tree::key::list::KeyExpList;
use i_tree::key::tree::KeyExpTree;
use crate::core::link::OverlayLink;
use crate::geom::id_point::IdPoint;

pub(super) trait FillStrategy<C> {
    fn add_and_fill(this: C, bot: C) -> (C, SegmentFill);
}

pub(super) trait InclusionFilterStrategy {
    fn is_included(fill: SegmentFill) -> bool;
}

pub(crate) trait GraphNode {
    fn with_indices(indices: &[usize]) -> Self;
}

pub(crate) struct GraphBuilder<C, N> {
    list: Option<KeyExpList<VSegment, i32, C>>,
    tree: Option<KeyExpTree<VSegment, i32, C>>,
    pub(super) links: Vec<OverlayLink>,
    pub(super) nodes: Vec<N>,
    pub(super) fills: Vec<SegmentFill>,
    pub(super) ends: Vec<End>,
    pub(super) bin_store: BinStore<i32>,
}

impl<C: WindingCount, N: GraphNode> GraphBuilder<C, N> {

    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            list: None,
            tree: None,
            links: Vec::new(),
            nodes: Vec::new(),
            fills: Vec::new(),
            ends: Vec::new(),
            bin_store: BinStore::empty(0, 0),
        }
    }

    #[inline]
    pub(super) fn build_fills_with_strategy<F: FillStrategy<C>>(&mut self, solver: &Solver, segments: &[Segment<C>]) {
        let count = segments.len();
        if solver.is_list_fill(segments) {
            let capacity = count.log2_sqrt().max(4) * 2;
            let mut list = self.take_scan_list(capacity);
            self.build_fills::<F, KeyExpList<VSegment, i32, C>>(&mut list, segments);
            self.list = Some(list);
        } else {
            let capacity = count.log2_sqrt().max(8);
            let mut tree = self.take_scan_tree(capacity);
            self.build_fills::<F, KeyExpTree<VSegment, i32, C>>(&mut tree, segments);
            self.tree = Some(tree);
        }
    }

    #[inline]
    fn build_fills<F: FillStrategy<C>, S: KeyExpCollection<VSegment, i32, C>>(&mut self, scan_list: &mut S, segments: &[Segment<C>]) {
        let mut node = Vec::with_capacity(4);

        let n = segments.len();

        self.fills.resize(n, NONE);

        let mut i = 0;

        while i < n {
            let p = segments[i].x_segment.a;

            node.push(End {
                index: i,
                point: segments[i].x_segment.b,
            });
            i += 1;

            while i < n && segments[i].x_segment.a == p {
                node.push(End {
                    index: i,
                    point: segments[i].x_segment.b,
                });
                i += 1;
            }

            if node.len() > 1 {
                node.sort_by(|s0, s1| Triangle::clock_order_point(p, s1.point, s0.point));
            }

            let mut sum_count =
                scan_list.first_less_or_equal_by(p.x, C::new(0, 0), |s| s.is_under_point_order(p));
            let mut fill: SegmentFill;

            for se in node.iter() {
                let sid = unsafe { segments.get_unchecked(se.index) };
                (sum_count, fill) = F::add_and_fill(sid.count, sum_count);
                unsafe { *self.fills.get_unchecked_mut(se.index) = fill }
                if sid.x_segment.is_not_vertical() {
                    scan_list.insert(sid.x_segment.into(), sum_count, p.x);
                }
            }

            node.clear();
        }
    }

    #[inline]
    pub(super) fn build_links_by_filter<F: InclusionFilterStrategy>(&mut self, segments: &[Segment<C>]) {
        let additional = segments.len().saturating_sub(self.links.len());
        if additional > 0 {
            self.links.reserve(additional);
        }
        self.links.clear();

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
        let additional = segments.len().saturating_sub(self.links.len());
        if additional > 0 {
            self.links.reserve(additional);
        }
        self.links.clear();

        for (segment, &fill) in segments.iter().zip(&self.fills) {
            self.links.push(OverlayLink::new(
                IdPoint::new(0, segment.x_segment.a),
                IdPoint::new(0, segment.x_segment.b),
                fill,
            ));
        }
    }

    #[inline]
    fn take_scan_list(&mut self, capacity: usize) -> KeyExpList<VSegment, i32, C> {
        if let Some(mut list) = self.list.take() {
            list.clear();
            list.reserve_capacity(capacity);
            list
        } else {
            KeyExpList::new(capacity)
        }
    }

    #[inline]
    fn take_scan_tree(&mut self, capacity: usize) -> KeyExpTree<VSegment, i32, C> {
        if let Some(mut tree) = self.tree.take() {
            tree.clear();
            tree.reserve_capacity(capacity);
            tree
        } else {
            KeyExpTree::new(capacity)
        }
    }
}
