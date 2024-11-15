#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SolverOption {
    #[default]
    Auto,
    Average,
    Precise,
}

impl SolverOption {
    pub(crate) const ALL: [SolverOption; 3] = [
        SolverOption::Auto,
        SolverOption::Average,
        SolverOption::Precise
    ];
}

impl std::fmt::Display for SolverOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SolverOption::Auto => "Auto",
                SolverOption::Average => "Average",
                SolverOption::Precise => "Precise",
            }
        )
    }
}