/// Represents the rule used to determine the "bind" of a shape, affecting how shapes are filled. For a visual description, see [Fill Rules](https://ishape-rust.github.io/iShape-js/overlay/filling_rules/filling_rules.html).
/// - `EvenOdd`: Only odd-numbered sub-regions are filled.
/// - `NonZero`: Only non-zero sub-regions are filled.
/// - `Positive`: Fills regions where the winding number is positive.
/// - `Negative`: Fills regions where the winding number is negative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillRule {
    EvenOdd,
    NonZero,
    Positive,
    Negative
}