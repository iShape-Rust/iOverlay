pub(crate) mod build;
mod builder;
mod builder_join;
#[cfg(feature = "variable_stroke_debug")]
pub mod debug;
mod math;
mod resource;
mod section;
mod style;
pub use resource::IntVariableStrokeSource;
pub use style::{IntStrokeVertex, IntVariableStrokeStyle};

pub mod offset;
