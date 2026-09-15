/// Arithmetic used to construct stroke offsets, joins, and caps.
///
/// Both modes retain integer coordinates and use the same integer boolean engine.
/// They can produce different rounded vertices and arc tessellations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathMode {
    /// Existing fixed-point directions and integer trigonometry.
    #[default]
    Integer,
    /// Experimental f64 normalization and trigonometry. Directions are stored
    /// as UnitIntVector, then scaled with integer arithmetic; cross-platform bitwise
    /// reproducibility is not promised. Arc rotation_precision is ignored.
    Float,
}
