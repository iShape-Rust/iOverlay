use i_float::float::compatible::FloatPointCompatible;

/// The variable-stroke construction operation that emitted a raw pre-overlay edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableStrokeDebugEdgeKind {
    /// One of the two tangent boundaries of a drawable centerline section.
    SectionBoundary,
    /// One chord of a round join.
    JoinArc,
    /// A straight edge closing the gap between adjacent sections.
    JoinClosure,
    /// One chord of a round end cap.
    CapArc,
    /// A butt edge closing an end cap.
    CapClosure,
    /// One chord of a circle emitted for an isolated drawable vertex.
    CircleArc,
}

/// One directed edge submitted by `SegmentBuilder` before overlay processing.
#[derive(Debug, Clone, Copy)]
pub struct VariableStrokeDebugEdge<P: FloatPointCompatible> {
    pub a: P,
    pub b: P,
    pub kind: VariableStrokeDebugEdgeKind,
    /// Index of the source variable-width path.
    pub path_index: usize,
    /// Global insertion order across all source paths.
    pub order: usize,
}

/// The raw construction edges and the regular post-overlay stroke result.
#[derive(Debug, Clone)]
pub struct VariableStrokeDebugResult<P: FloatPointCompatible> {
    pub edges: alloc::vec::Vec<VariableStrokeDebugEdge<P>>,
    pub shapes: i_shape::base::data::Shapes<P>,
}
