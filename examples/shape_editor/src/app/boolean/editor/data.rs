use crate::util::point::EditorPoint;

pub(crate) struct StatelessData {
    pub(crate) editor_points: Vec<EditorPoint>,
}

impl Default for StatelessData {
    fn default() -> Self {
        StatelessData { editor_points: vec![] }
    }
}