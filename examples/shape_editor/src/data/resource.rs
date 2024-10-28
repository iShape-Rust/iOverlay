use crate::data::polygon::PolygonResource;

pub(crate) struct AppResource {
    pub(crate) polygon: PolygonResource
}

impl AppResource {

    pub(crate) fn new() -> Self {
        Self {
            polygon: PolygonResource::new("../tests/data")
        }
    }

}