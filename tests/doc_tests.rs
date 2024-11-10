#[cfg(test)]
mod tests {
    use i_float::float::compatible::FloatPointCompatible;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::single::SingleFloatOverlay;
    use i_overlay::float::slice::FloatSlice;

    #[test]
    fn test_simple_union() {
        // Define the subject "O"
        let subj = [
            // main contour
            vec![
                [1.0, 0.0],
                [1.0, 5.0],
                [4.0, 5.0],
                [4.0, 0.0], // the contour is auto closed!
            ],
            // hole contour
            vec![
                [2.0, 1.0],
                [3.0, 1.0],
                [3.0, 4.0],
                [2.0, 4.0], // the contour is auto closed!
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
        let result = [
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0]
        ].slice_by(&[
            [1.0, 0.0],
            [1.0, 3.0],
        ], FillRule::NonZero);


        println!("result: {:?}", result);
    }
}