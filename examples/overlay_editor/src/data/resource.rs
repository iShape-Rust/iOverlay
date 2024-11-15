use crate::data::string::StringResource;
use crate::data::boolean::BooleanResource;

pub struct AppResource {
    pub(crate) boolean: BooleanResource,
    pub(crate) string: StringResource,
}

impl AppResource {
    pub(crate) fn with_paths(boolean: &str, string: &str) -> Self {
        Self {
            boolean: BooleanResource::with_path(boolean),
            string: StringResource::with_path(string),
        }
    }

    pub fn with_content(boolean: String, string: String) -> Self {
        Self {
            boolean: BooleanResource::with_content(boolean),
            string: StringResource::with_content(string),
        }
    }

}