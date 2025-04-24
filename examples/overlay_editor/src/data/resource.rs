use crate::data::string::StringResource;
use crate::data::boolean::BooleanResource;
use crate::data::stroke::StrokeResource;
use crate::data::outline::OutlineResource;

pub struct AppResource {
    pub(crate) boolean: BooleanResource,
    pub(crate) string: StringResource,
    pub(crate) stroke: StrokeResource,
    pub(crate) outline: OutlineResource,
}

impl AppResource {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn with_paths(boolean: &str, string: &str, stroke: &str, outline: &str) -> Self {
        Self {
            boolean: BooleanResource::with_path(boolean),
            string: StringResource::with_path(string),
            stroke: StrokeResource::with_path(stroke),
            outline: OutlineResource::with_path(outline),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn with_content(boolean: String, string: String, stroke: String, outline: String) -> Self {
        Self {
            boolean: BooleanResource::with_content(boolean),
            string: StringResource::with_content(string),
            stroke: StrokeResource::with_content(stroke),
            outline: OutlineResource::with_content(outline),
        }
    }
}