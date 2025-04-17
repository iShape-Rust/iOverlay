#[cfg(test)]
mod tests {
    use i_float::float::compatible::FloatPointCompatible;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::clip::FloatClip;
    use i_overlay::float::single::SingleFloatOverlay;
    use i_overlay::float::slice::FloatSlice;
    use i_overlay::string::clip::ClipRule;

    #[test]
    fn test_simple_union() {
        // Define the subject "O"
        let subj = [
            // main contour
            vec![
                [1.0, 0.0],
                [4.0, 0.0],
                [4.0, 5.0],
                [1.0, 5.0], // the contour is auto closed!
            ],
            // hole contour
            vec![
                [2.0, 1.0],
                [2.0, 4.0],
                [3.0, 4.0],
                [3.0, 1.0], // the contour is auto closed!
            ],
        ];

        // Define the clip "-"
        let clip = [
            // main contour
            [0.0, 2.0],
            [5.0, 2.0],
            [5.0, 3.0],
            [0.0, 3.0], // the contour is auto closed!
        ];

        let result = subj.overlay(&clip, OverlayRule::Union, FillRule::EvenOdd);

        println!("result: {:?}", result);
    }

    #[test]
    fn test_custom_point() {
        #[derive(Clone, Copy, Debug)]
        struct CustomPoint {
            x: f32,
            y: f32,
        }

        impl FloatPointCompatible<f32> for CustomPoint {
            fn from_xy(x: f32, y: f32) -> Self {
                Self { x, y }
            }

            fn x(&self) -> f32 {
                self.x
            }

            fn y(&self) -> f32 {
                self.y
            }
        }

        let subj = [
            CustomPoint { x: 0.0, y: 0.0 },
            CustomPoint { x: 0.0, y: 3.0 },
            CustomPoint { x: 3.0, y: 3.0 },
            CustomPoint { x: 3.0, y: 0.0 },
        ];

        let clip = [
            CustomPoint { x: 1.0, y: 1.0 },
            CustomPoint { x: 1.0, y: 2.0 },
            CustomPoint { x: 2.0, y: 2.0 },
            CustomPoint { x: 2.0, y: 1.0 },
        ];

        let result = subj.overlay(&clip, OverlayRule::Difference, FillRule::EvenOdd);

        println!("result: {:?}", result);
    }

    #[test]
    fn test_slice() {
        let polygon = [
            [1.0, 1.0],
            [1.0, 4.0],
            [4.0, 4.0],
            [4.0, 1.0],
        ];

        let slicing_line = [
            [3.0, 5.0],
            [2.0, 2.0],
            [3.0, 3.0],
            [2.0, 0.0],
        ];

        let result = polygon.slice_by(&slicing_line, FillRule::NonZero);

        println!("result: {:?}", result);
    }

    #[test]
    fn test_clip() {
        let polygon = [
            [1.0, 1.0],
            [1.0, 4.0],
            [4.0, 4.0],
            [4.0, 1.0],
        ];

        let string_line = [
            [3.0, 5.0],
            [2.0, 2.0],
            [3.0, 3.0],
            [2.0, 0.0],
        ];

        let clip_rule = ClipRule { invert: false, boundary_included: false };
        let result = string_line.clip_by(&polygon, FillRule::NonZero, clip_rule);

        println!("result: {:?}", result);
    }
}