/// Represents the rule used to determine the "hole" of a shape, affecting how shapes are filled. For a visual description, see [Fill Rules](https://ishape-rust.github.io/iShape-js/overlay/filling_rules.html).
/// - `EvenOdd`: A point is part of a hole if a line from that point to infinity crosses an odd number of shape edges.
/// - `NonZero`: A point is part of a hole if the number of left-to-right crossings differs from right-to-left crossings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillRule {
    EvenOdd,
    NonZero
}