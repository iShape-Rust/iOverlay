use i_float::int::point::IntPoint;
use crate::geom::line_range::LineRange;
use crate::geom::x_segment::XSegment;
use crate::iso::core::data::IsoData;
use crate::iso::layout::Layout;
use crate::iso::split::column::{SplitPoint, SplitResult};
use crate::iso::split::table::Table;
use crate::segm::segment::{Segment, SegmentFill};
use crate::segm::winding_count::ShapeCountBoolean;

impl IsoData {
    pub(crate) fn into_segments(self, range: LineRange) {
        let cnt_xy = self.hz_segments.len().max(self.vr_segments.len());
        let cnt_dg = self.dg_pos_segments.len().max(self.dg_neg_segments.len());
        let count = cnt_xy.max(cnt_dg);
        let layout = Layout::new(count, range);

        let result = Table::new(layout, &self).split();

    }
}

impl SplitResult {
    fn append(&mut self, mut other: Self) {
        self.vr_points.append(&mut other.vr_points);
        self.hz_points.append(&mut other.hz_points);
        self.dg_pos_points.append(&mut other.dg_pos_points);
        self.dg_neg_points.append(&mut other.dg_neg_points);
    }

    fn sort(&mut self) {
        self.vr_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));
        self.hz_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));
        self.dg_pos_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));
        self.dg_neg_points.sort_unstable_by(|a, b| a.index.cmp(&b.index).then(a.xy.cmp(&b.xy)));
    }

}

impl Table {
    pub(crate) fn split(&mut self) -> SplitResult {
        let width = self.layout.column_width();

        let mut result = SplitResult {
            vr_points: vec![],
            hz_points: vec![],
            dg_pos_points: vec![],
            dg_neg_points: vec![],
        };

        for column in self.columns.iter_mut() {
            result.append(column.split(width));
        }

        result
    }
}