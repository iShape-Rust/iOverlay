use crate::data::string::StringResource;
use crate::data::boolean::BooleanResource;

pub(crate) struct AppResource {
    pub(crate) boolean: BooleanResource,
    pub(crate) string: StringResource,
}

impl AppResource {
    pub(crate) fn new() -> Self {
        Self {
            boolean: BooleanResource::new("../../tests/boolean"),
            string: StringResource::new("../../tests/string"),
        }
    }
}