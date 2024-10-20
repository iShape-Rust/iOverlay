use crate::data::polygon::PolygonResource;

pub(crate) struct AppReosurce {
    pub(crate) polygon: PolygonResource
}

impl AppReosurce {

    pub(crate) fn new() -> Self {
        Self {
            polygon: PolygonResource::new("../tests/data")
        }
    }

}