use i_triangle::i_overlay::core::fill_rule::FillRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FillOption {
    #[default]
    NonZero,
    EvenOdd,
    Positive,
    Negative,
}

impl FillOption {
    pub(crate) const ALL: [FillOption; 4] = [
        FillOption::NonZero,
        FillOption::EvenOdd,
        FillOption::Positive,
        FillOption::Negative,
    ];

    pub(crate) fn fill_rule(&self) -> FillRule {
        match self {
            FillOption::NonZero => FillRule::NonZero,
            FillOption::EvenOdd => FillRule::EvenOdd,
            FillOption::Positive => FillRule::Positive,
            FillOption::Negative => FillRule::Negative,
        }
    }
}

impl std::fmt::Display for FillOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FillOption::NonZero => "NonZero",
                FillOption::EvenOdd => "EvenOdd",
                FillOption::Positive => "Positive",
                FillOption::Negative => "Negative",
            }
        )
    }
}