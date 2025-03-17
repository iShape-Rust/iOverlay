#[cfg(test)]
mod tests {
    use i_float::int::point::IntPoint;
    use i_overlay::iso::core::overlay::IsoOverlay;

    #[test]
    fn test_0() {
        let subj =
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, 0)
                ].to_vec()
            ].to_vec();
        let clip = vec![];

        let mut overlay = IsoOverlay::with_contours(&subj, &clip);
        overlay.

    }
}