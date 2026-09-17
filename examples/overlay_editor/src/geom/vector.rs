use eframe::egui::Vec2;
use i_triangle::i_overlay::i_float::int::point::IntPoint;

pub(crate) fn round_to_int(value: Vec2) -> IntPoint {
    IntPoint::new(value.x.round() as i32, value.y.round() as i32)
}
