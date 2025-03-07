use crate::core::solver::Solver;
use crate::geom::line_range::LineRange;
use crate::iso::core::data::IsoData;
use crate::iso::layout::FragLayout;
use crate::iso::split::column::SplitResult;
use crate::iso::split::table::Table;
use crate::segm::merge::ShapeSegmentsMerge;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;
use crate::util::sort::SmartBinSort;

impl IsoData {
    pub(crate) fn into_segments(self, solver: &Solver, range: LineRange) -> Vec<Segment<ShapeCountBoolean>> {
        let cnt_xy = self.hz_segments.len().max(self.vr_segments.len());
        let cnt_dg = self.dg_pos_segments.len().max(self.dg_neg_segments.len());
        let count = cnt_xy.max(cnt_dg);
        let layout = FragLayout::new(count, range);

        let result = Table::new(layout, &self).split();

        let mut segments = self.divide(result);
        segments.smart_bin_sort_by(solver, |a, b| a.x_segment.cmp(&b.x_segment));
        segments.merge_if_needed();

        segments
    }
}

impl SplitResult {
    fn append(&mut self, mut other: Self) {
        self.vr_points.append(&mut other.vr_points);
        self.hz_points.append(&mut other.hz_points);
        self.dg_pos_points.append(&mut other.dg_pos_points);
        self.dg_neg_points.append(&mut other.dg_neg_points);
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

#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use crate::core::solver::Solver;
    use crate::iso::core::overlay::IsoOverlay;

    #[test]
    fn test_into_segments_0() {
        let subj = vec![vec![
            IntPoint::new(0, 0),
            IntPoint::new(0, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(20, 0),
            IntPoint::new(20, 10),
            IntPoint::new(30, 10),
            IntPoint::new(30, 0),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 8);
    }

    #[test]
    fn test_into_segments_1() {
        let subj = vec![vec![
            IntPoint::new(0, 5),
            IntPoint::new(5, 10),
            IntPoint::new(10, 5),
            IntPoint::new(5, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(20, 5),
            IntPoint::new(25, 10),
            IntPoint::new(30, 5),
            IntPoint::new(25, 0),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 8);
    }

    #[test]
    fn test_into_segments_2() {
        let subj = vec![vec![
            IntPoint::new(0, 0),
            IntPoint::new(0, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(5, 5),
            IntPoint::new(5, 15),
            IntPoint::new(15, 15),
            IntPoint::new(15, 5),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 12);
    }

    #[test]
    fn test_into_segments_3() {
        let subj = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(0, 10),
            IntPoint::new(10, 0),
            IntPoint::new(0, -10),
        ]];

        let clip = vec![vec![
            IntPoint::new(-5, -5),
            IntPoint::new(-5, 5),
            IntPoint::new(5, 5),
            IntPoint::new(5, -5),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 12);
    }

    #[test]
    fn test_into_segments_4() {
        let subj = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(0, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(0, 0),
            IntPoint::new(0, 10),
            IntPoint::new(10, 10),
            IntPoint::new(10, 0),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 7);
    }

    #[test]
    fn test_into_segments_5() {
        let subj = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(0, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(-10, 10),
            IntPoint::new(-10, 20),
            IntPoint::new(0, 20),
            IntPoint::new(0, 10),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 7);
    }

    #[test]
    fn test_into_segments_6() {
        let subj = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(0, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(0, 0),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 4);
    }

    #[test]
    fn test_into_segments_7() {
        let subj = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(0, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(0, 0),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 4);
    }

    #[test]
    fn test_into_segments_8() {
        let subj = vec![vec![
            IntPoint::new(-10, 0),
            IntPoint::new(-10, 10),
            IntPoint::new(0, 10),
            IntPoint::new(0, 0),
        ]];

        let clip = vec![vec![
            IntPoint::new(0, 2),
            IntPoint::new(0, 8),
            IntPoint::new(10, 8),
            IntPoint::new(10, 2),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 9);
    }

    #[test]
    fn test_into_segments_9() {
        let subj = vec![
            vec![
                IntPoint::new(-10, 0),
                IntPoint::new(-10, 10),
                IntPoint::new(-2, 10),
                IntPoint::new(-2, 0),
            ],
            vec![
                IntPoint::new(-8, 5),
                IntPoint::new(-5, 8),
                IntPoint::new(-2, 5),
                IntPoint::new(-5, 2),
            ]
        ];

        let clip = vec![vec![
            IntPoint::new(0, -10),
            IntPoint::new(0, -5),
            IntPoint::new(16, -5),
            IntPoint::new(16, -10),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 13);
    }

    #[test]
    fn test_into_segments_10() {
        let subj = vec![
            vec![
                IntPoint::new(-10, 0),
                IntPoint::new(-10, 10),
                IntPoint::new(-2, 10),
                IntPoint::new(-2, 0),
            ],
            vec![
                IntPoint::new(-2, 5),
                IntPoint::new(3, 10),
                IntPoint::new(8, 5),
                IntPoint::new(3, 0),
            ]
        ];

        let clip = vec![vec![
            IntPoint::new(0, -10),
            IntPoint::new(0, -5),
            IntPoint::new(16, -5),
            IntPoint::new(16, -10),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 13);
    }

    #[test]
    fn test_into_segments_11() {
        let subj = vec![
            vec![
                IntPoint::new(-10, 0),
                IntPoint::new(-10, 10),
                IntPoint::new(-2, 10),
                IntPoint::new(-2, 0),
            ],
            vec![
                IntPoint::new(0, 0),
                IntPoint::new(0, 10),
                IntPoint::new(8, 10),
                IntPoint::new(8, 0),
            ],
            vec![
                IntPoint::new(10, 0),
                IntPoint::new(10, 10),
                IntPoint::new(12, 10),
                IntPoint::new(12, 0),
            ]
        ];

        let clip = vec![vec![
            IntPoint::new(0, -10),
            IntPoint::new(0, -5),
            IntPoint::new(10, -5),
            IntPoint::new(10, -10),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 16);
    }

    #[test]
    fn test_into_segments_12() {
        let subj = vec![
            vec![
                IntPoint::new(-10, 0),
                IntPoint::new(-10, 10),
                IntPoint::new(-2, 10),
                IntPoint::new(-2, 0),
            ],
            vec![
                IntPoint::new(10, 0),
                IntPoint::new(22, 10),
                IntPoint::new(22, 10),
                IntPoint::new(10, 0),
            ]
        ];

        let clip = vec![vec![
            IntPoint::new(0, -10),
            IntPoint::new(0, -5),
            IntPoint::new(16, -5),
            IntPoint::new(16, -10),
        ]];

        let overlay = IsoOverlay::with_contours(&subj, &clip);
        let segments = overlay.into_segments(Solver::default());

        assert_eq!(segments.len(), 12);
    }
}