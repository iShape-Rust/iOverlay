use alloc::vec::Vec;
use crate::segm::segment::Segment;
use crate::segm::winding::WindingCount;

pub(crate) trait ShapeSegmentsMerge {
    fn merge_if_needed(&mut self) -> bool;
    fn copy_and_merge(&mut self, resource: &Self);
}

impl<C: WindingCount> ShapeSegmentsMerge for Vec<Segment<C>> {
    fn merge_if_needed(&mut self) -> bool {
        if self.len() < 2 { return false; }

        let mut prev = &self[0].x_segment;
        for i in 1..self.len() {
            let this = &self[i].x_segment;
            if prev.eq(this) {
                let new_len = merge(self, i);
                self.truncate(new_len);
                return true;
            }
            prev = this;
        }

        false
    }

    fn copy_and_merge(&mut self, resource: &Self) {
        self.clear();
        let mut iter = resource.iter();
        let first_item = if let Some(first) = iter.next() {
            first
        } else {
            return;
        };

        let mut prev = *first_item;

        for item in iter {
            if prev.x_segment.eq(&item.x_segment) {
                prev.count.apply(item.count);
            } else {
                if prev.count.is_not_empty() {
                    self.push(prev);
                }
                prev = *item;
            }
        }

        if prev.count.is_not_empty() {
            self.push(prev);
        }
    }

}

fn merge<C: WindingCount>(segments: &mut [Segment<C>], after: usize) -> usize {
    let mut i = after;
    let mut j = i - 1;
    let mut prev = segments[j];

    while i < segments.len() {
        if prev.x_segment.eq(&segments[i].x_segment) {
            prev.count.apply(segments[i].count);
        } else {
            if prev.count.is_not_empty() {
                segments[j] = prev;
                j += 1;
            }
            prev = segments[i];
        }
        i += 1;
    }

    if prev.count.is_not_empty() {
        segments[j] = prev;
        j += 1;
    }

    j
}

#[cfg(test)]
mod tests {
    use alloc::vec;
use crate::segm::boolean::ShapeCountBoolean;
use i_float::int::point::IntPoint;
    use super::*;

    #[test]
    fn test_merge_if_needed_empty() {
        let mut segments: Vec<Segment<ShapeCountBoolean>> = Vec::new();
        segments.merge_if_needed();
        assert!(segments.is_empty(), "Empty vector should remain empty after merge");
    }

    #[test]
    fn test_merge_if_needed_single_element() {
        let a = IntPoint::new(1, 2);
        let b = IntPoint::new(3, 4);
        let count = ShapeCountBoolean::new(1, 1);
        let segment = Segment::create_and_validate(a, b, count);
        let mut segments = vec![segment];
        segments.merge_if_needed();
        assert_eq!(segments.len(), 1, "Single segment should remain unchanged");
        assert_eq!(segments[0], segment, "Segment should be unchanged after merge");
    }

    #[test]
    fn test_merge_if_needed_no_merge() {
        let a1 = IntPoint::new(1, 2);
        let b1 = IntPoint::new(3, 4);
        let count1 = ShapeCountBoolean::new(1, 0);
        let segment1 = Segment::create_and_validate(a1, b1, count1);

        let a2 = IntPoint::new(5, 6);
        let b2 = IntPoint::new(7, 8);
        let count2 = ShapeCountBoolean::new(0, 1);
        let segment2 = Segment::create_and_validate(a2, b2, count2);

        let mut segments = vec![segment1, segment2];
        segments.merge_if_needed();

        assert_eq!(segments.len(), 2, "Segments with different x_segments should not be merged");
        assert_eq!(segments[0], segment1, "First segment should remain unchanged");
        assert_eq!(segments[1], segment2, "Second segment should remain unchanged");
    }

    #[test]
    fn test_merge_if_needed_single_merge() {
        let a1 = IntPoint::new(1, 2);
        let b1 = IntPoint::new(3, 4);
        let count1 = ShapeCountBoolean::new(1, 0);
        let segment1 = Segment::create_and_validate(a1, b1, count1);

        let a2 = IntPoint::new(1, 2);
        let b2 = IntPoint::new(3, 4);
        let count2 = ShapeCountBoolean::new(0, 1);
        let segment2 = Segment::create_and_validate(a2, b2, count2);

        let mut segments = vec![segment1, segment2];
        segments.merge_if_needed();

        assert_eq!(segments.len(), 1, "Segments should be merged into one");
        let merged_count = ShapeCountBoolean::new(1, 1);
        let expected_segment = Segment::create_and_validate(a1, b1, merged_count);
        assert_eq!(segments[0], expected_segment, "Merged segment should have combined counts");
    }

    #[test]
    fn test_merge_if_needed_multiple_merges() {
        let a = IntPoint::new(1, 2);
        let b = IntPoint::new(3, 4);

        let count1 = ShapeCountBoolean::new(1, 0);
        let count2 = ShapeCountBoolean::new(0, 1);
        let count3 = ShapeCountBoolean::new(2, 2);

        let segment1 = Segment::create_and_validate(a, b, count1);
        let segment2 = Segment::create_and_validate(a, b, count2);
        let segment3 = Segment::create_and_validate(a, b, count3);

        let mut segments = vec![segment1, segment2, segment3];
        segments.merge_if_needed();

        assert_eq!(segments.len(), 1, "All segments should be merged into one");
        let merged_count = ShapeCountBoolean::new(3, 3);
        let expected_segment = Segment::create_and_validate(a, b, merged_count);
        assert_eq!(segments[0], expected_segment, "Merged segment should have combined counts");
    }

    #[test]
    fn test_merge_if_needed_segments_with_inverted_order() {
        let a1 = IntPoint::new(3, 4);
        let b1 = IntPoint::new(1, 2);
        let count1 = ShapeCountBoolean::new(1, 0);
        // create_and_validate should order the points
        let segment1 = Segment::create_and_validate(a1, b1, count1);

        let a2 = IntPoint::new(1, 2);
        let b2 = IntPoint::new(3, 4);
        let count2 = ShapeCountBoolean::new(0, 1);
        let segment2 = Segment::create_and_validate(a2, b2, count2);

        let mut segments = vec![segment1, segment2];
        segments.merge_if_needed();

        // Both segments should have the same ordered x_segment
        assert_eq!(segments.len(), 1, "Segments with inverted points should be merged");

        let merged_count = ShapeCountBoolean::new(1, 1);
        let expected_segment = Segment::create_and_validate(IntPoint::new(1, 2), IntPoint::new(3, 4), merged_count);
        assert_eq!(segments[0], expected_segment, "Merged segment should have combined counts and ordered points");
    }

    #[test]
    fn test_merge_if_needed_no_merge_different_x_segments() {
        let a1 = IntPoint::new(1, 1);
        let b1 = IntPoint::new(2, 2);
        let count1 = ShapeCountBoolean::new(1, 1);
        let segment1 = Segment::create_and_validate(a1, b1, count1);

        let a2 = IntPoint::new(3, 3);
        let b2 = IntPoint::new(4, 4);
        let count2 = ShapeCountBoolean::new(2, 2);
        let segment2 = Segment::create_and_validate(a2, b2, count2);

        let a3 = IntPoint::new(5, 5);
        let b3 = IntPoint::new(6, 6);
        let count3 = ShapeCountBoolean::new(3, 3);
        let segment3 = Segment::create_and_validate(a3, b3, count3);

        let mut segments = vec![segment1, segment2, segment3];
        segments.merge_if_needed();

        assert_eq!(segments.len(), 3, "Segments with different x_segments should not be merged");
        assert_eq!(segments[0], segment1, "First segment should remain unchanged");
        assert_eq!(segments[1], segment2, "Second segment should remain unchanged");
        assert_eq!(segments[2], segment3, "Third segment should remain unchanged");
    }
}
