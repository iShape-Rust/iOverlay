/// Arithmetic used to construct outline offsets, strokes, and variable-width strokes.
///
/// Both modes retain integer coordinates and use the same integer boolean engine.
/// They can produce different rounded vertices and arc tessellations.
/// Use Integer for cross-platform deterministic construction with identical
/// integer inputs, settings, engine, and library version. Otherwise, prefer Float
/// for more accurate normalization and arcs and generally better performance.
/// The input namespace (`mesh::int` or `mesh::float`) does not select this mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathMode {
    /// Fixed-point directions and integer trigonometry. This is the default.
    #[default]
    Integer,
    /// f64 normalization, trigonometry, and variable-width tangent contacts.
    /// Directions are stored as UnitIntVector
    /// without a norm check, then scaled with integer arithmetic. Their length
    /// may slightly exceed one. Cross-platform bitwise reproducibility is not
    /// promised. Arc rotation_precision is ignored.
    Float,
}
