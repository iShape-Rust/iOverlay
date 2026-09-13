use i_float::int::{number::int::IntNumber, rect::IntRect};
pub(super) fn expanded_is_safe<I: IntNumber>(rect: IntRect<I>, padding: I::Wide) -> bool {
    let min = I::MIN.to_wide();
    let max = I::MAX.to_wide();
    IntRect::new(
        I::from_wide((rect.min_x.to_wide() - padding).max(min)),
        I::from_wide((rect.max_x.to_wide() + padding).min(max)),
        I::from_wide((rect.min_y.to_wide() - padding).max(min)),
        I::from_wide((rect.max_y.to_wide() + padding).min(max)),
    )
    .is_in_safe_range()
}
