use crate::segm::winding_count::{ShapeCountBoolean, WindingCount};
use i_shape::int::shape::{IntContour, IntShape};

use crate::core::overlay::ShapeType;
use crate::geom::line_range::LineRange;
use crate::iso::core::data::IsoData;
use crate::iso::core::metric::Metric;

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

    pub fn into_graph(self) {

    }
}
