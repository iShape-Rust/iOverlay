use i_float::point::IntPoint;
use i_shape::int::simple::Simple;
use crate::core::overlay::ShapeType;
use crate::segm::segment::Segment;
use crate::segm::shape_count::ShapeCount;
use crate::segm::x_segment::XSegment;

pub(crate) trait BuildSegments {
    fn append_segments(&mut self, path: &[IntPoint], shape_type: ShapeType);
    // fn append_private_segments(&mut self, path: &[IntPoint], shape_type: ShapeType);
}

impl BuildSegments for Vec<Segment> {
    #[inline]
    fn append_segments(&mut self, path: &[IntPoint], shape_type: ShapeType) {
        if path.is_simple() {
            append_private_segments(self, path, shape_type);
        } else {
            let path = path.to_simple();
            if path.len() > 2 {
                append_private_segments(self, path.as_slice(), shape_type);
            }
        }
    }
}

fn append_private_segments(segments: &mut Vec<Segment>, path: &[IntPoint], shape_type: ShapeType) {
    let mut p0 = path[path.len() - 1];

    match shape_type {
        ShapeType::Subject => {
            for &p1 in path {
                let segment = if p0 < p1 {
                    Segment { x_segment: XSegment { a: p0, b: p1 }, count: ShapeCount::new(1, 0) }
                } else {
                    Segment { x_segment: XSegment { a: p1, b: p0 }, count: ShapeCount::new(-1, 0) }
                };
                segments.push(segment);
                p0 = p1
            }
        }
        ShapeType::Clip => {
            for &p1 in path {
                let segment = if p0 < p1 {
                    Segment { x_segment: XSegment { a: p0, b: p1 }, count: ShapeCount::new(0, 1) }
                } else {
                    Segment { x_segment: XSegment { a: p1, b: p0 }, count: ShapeCount::new(0, -1) }
                };
                segments.push(segment);
                p0 = p1
            }
        }
    }
}