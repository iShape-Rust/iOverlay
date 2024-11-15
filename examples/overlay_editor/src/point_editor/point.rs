use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_shape::int::path::IntPaths;

#[derive(Debug, Clone)]
pub(crate) struct MultiIndex {
    pub(crate) point_index: usize,
    pub(crate) path_index: usize,
    pub(crate) group_index: usize,      // subj or clip
}

#[derive(Debug, Clone)]
pub(crate) struct EditorPoint {
    pub(crate) pos: IntPoint,
    pub(crate) index: MultiIndex,
}

pub(crate) trait PathsToEditorPoints {
    fn feed_edit_points(&self, group_index: usize, edit_points: &mut Vec<EditorPoint>);
}

impl PathsToEditorPoints for IntPaths {
    fn feed_edit_points(&self, group_index: usize, edit_points: &mut Vec<EditorPoint>) {
        for (path_index, path) in self.iter().enumerate() {
            for (point_index, &pos) in path.iter().enumerate() {
                let index = MultiIndex { point_index, path_index, group_index };
                edit_points.push(EditorPoint { pos, index })
            }
        }
    }
}
