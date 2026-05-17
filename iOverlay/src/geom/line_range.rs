use i_float::int::number::int::IntNumber;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LineRange<I: IntNumber = i32> {
    pub(crate) min: I,
    pub(crate) max: I,
}
