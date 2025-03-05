use i_shape::int::shape::{IntContour, IntShape};
use crate::core::overlay::ShapeType;
use crate::core::solver::Solver;
use crate::geom::line_range::LineRange;
use crate::iso::core::data::IsoData;
use crate::iso::core::metric::Metric;
use crate::segm::segment::Segment;
use crate::segm::winding_count::ShapeCountBoolean;

#[derive(Clone)]
pub struct IsoOverlay {
    data: IsoData,
    x_range: LineRange,
}

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `OverlayGraph`. It prepares the necessary data for boolean operations.
impl IsoOverlay {
    /// Creates a new `IsoOverlay` instance and initializes it with subject and clip contours.
    /// - `subj`: An array of contours that together define the subject shape.
    /// - `clip`: An array of contours that together define the clip shape.
    pub fn with_contours(subj: &[IntContour], clip: &[IntContour]) -> Self {
        let mut metric = Metric::new();
        metric.add(subj);
        metric.add(clip);

        let mut data = IsoData::new(&metric);

        data.add_contours(ShapeType::Subject, subj);
        data.add_contours(ShapeType::Clip, clip);

        Self { data, x_range: LineRange { min: metric.min, max: metric.max } }
    }

    /// Creates a new `IsoOverlay` instance and initializes it with subject and clip shapes.
    /// - `subj`: An array of shapes to be used as the subject in the overlay operation.
    /// - `clip`: An array of shapes to be used as the clip in the overlay operation.
    pub fn with_shapes(subj: &[IntShape], clip: &[IntShape]) -> Self {
        let mut metric = Metric::new();
        for contours in subj {
            metric.add(contours);
        }
        for contours in clip {
            metric.add(contours);
        }

        let mut data = IsoData::new(&metric);

        for contours in subj {
            data.add_contours(ShapeType::Subject, contours);
        }
        for contours in clip {
            data.add_contours(ShapeType::Clip, contours);
        }

        Self { data, x_range: LineRange { min: metric.min, max: metric.max } }
    }

    pub fn into_segments(self, solver: Solver) -> Vec<Segment<ShapeCountBoolean>> {
        self.data.into_segments(&solver, self.x_range)
    }
}


#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use crate::iso::core::overlay::IsoOverlay;

    #[test]
    fn test_init_0() {
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

        assert_eq!(overlay.data.vr_segments.len(), 4);
        assert_eq!(overlay.data.hz_segments.len(), 4);
    }

    #[test]
    fn test_init_1() {
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

        assert_eq!(overlay.data.dg_pos_segments.len(), 4);
        assert_eq!(overlay.data.dg_neg_segments.len(), 4);
    }
}