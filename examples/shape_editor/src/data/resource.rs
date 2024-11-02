use crate::data::polygon::BooleanResource;

pub(crate) struct AppResource {
    pub(crate) boolean: BooleanResource
}

impl AppResource {

    pub(crate) fn new() -> Self {
        Self {
            boolean: BooleanResource::new("../../tests/data")
        }
    }

}