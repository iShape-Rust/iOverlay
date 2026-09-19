use i_float::float::compatible::FloatPointCompatible;

pub use crate::mesh::int::variable_stroke::debug::VariableStrokeDebugEdgeKind;

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
