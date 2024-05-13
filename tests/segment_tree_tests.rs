#[cfg(test)]
mod tests {
    use i_float::point::IntPoint;
    use i_overlay::line_range::LineRange;
    use i_overlay::split::scan_tree::ScanSplitTree;
    use i_overlay::x_segment::XSegment;

    #[test]
    fn test_0() {
        let mut tree = ScanSplitTree::new(LineRange { min: -28, max: 28 }, 16);
        tree.insert(XSegment { a: IntPoint { x: -28, y: -28 }, b: IntPoint { x: -20, y: 0 } });
        let result = tree.intersect_and_remove_other(XSegment { a: IntPoint { x: -21, y: 1 }, b: IntPoint { x: -15, y: -13 } });


        assert!(!result.is_none())
    }

    #[test]
    fn test_1() {
        let mut tree = ScanSplitTree::new(LineRange { min: -28, max: 28 }, 16);
        tree.insert(XSegment { a: IntPoint { x: -28, y: -28 }, b: IntPoint { x: -20, y: 0 } });
        tree.insert(XSegment { a: IntPoint { x: -28, y: -28 }, b: IntPoint { x: 0, y: -20 } });
        tree.insert(XSegment { a: IntPoint { x: -28, y: 28 }, b: IntPoint { x: -20, y: 0 } });
        tree.insert(XSegment { a: IntPoint { x: -28, y: 28 }, b: IntPoint { x: -20, y: 0 } });
        tree.insert(XSegment { a: IntPoint { x: -28, y: 28 }, b: IntPoint { x: 0, y: 20 } });

        let result = tree.intersect_and_remove_other(XSegment { a: IntPoint { x: -21, y: 1 }, b: IntPoint { x: -15, y: -13 } });


        assert!(!result.is_none())
    }
}
