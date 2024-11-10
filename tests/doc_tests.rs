#[cfg(test)]
mod tests {
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::single::SingleFloatOverlay;

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



}