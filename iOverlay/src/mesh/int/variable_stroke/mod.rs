mod builder;
#[cfg(feature = "variable_stroke_debug")]
pub mod debug;
mod resource;
mod section;
mod style;
pub use resource::IntVariableStrokeSource;
pub use style::{IntStrokeVertex, IntVariableStrokeStyle};

pub mod offset;
