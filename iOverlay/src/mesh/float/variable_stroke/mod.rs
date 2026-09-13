mod builder;
#[cfg(feature = "variable_stroke_debug")]
mod debug;
pub mod offset;
mod resource;
mod section;
mod style;

#[cfg(feature = "variable_stroke_debug")]
pub use debug::{VariableStrokeDebugEdge, VariableStrokeDebugEdgeKind, VariableStrokeDebugResult};
#[cfg(feature = "variable_stroke_debug")]
pub use offset::VariableStrokeDebug;
pub use resource::VariableStrokeSource;
pub use style::{StrokeVertex, VariableStrokeStyle};
