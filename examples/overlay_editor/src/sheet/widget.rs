use crate::{
    app::design::Design,
    geom::camera::Camera,
    point_editor::{point::EditorPoint, state::PointsEditorState, widget::PointEditUpdate},
    sheet::state::SheetState,
};
use eframe::egui::{self, Color32, CursorIcon, Painter, Sense, Stroke, Vec2};

pub(crate) struct SheetWidget;

impl SheetWidget {
    pub(crate) fn show(
        ui: &mut egui::Ui,
        camera: &mut Camera,
        points: &[EditorPoint],
        sheet: &mut SheetState,
        editor: &mut PointsEditorState,
    ) -> (Painter, Option<PointEditUpdate>) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size().max(Vec2::splat(1.0)),
            Sense::CLICK | Sense::DRAG,
        );
        painter.rect_filled(response.rect, 0.0, Color32::from_rgb(18, 18, 18));
        let mut update = None;
        let (cursor, pressed, down, released, scroll) = ui.input(|i| {
            (
                i.pointer.interact_pos(),
                i.pointer.primary_pressed(),
                i.pointer.primary_down(),
                i.pointer.primary_released(),
                i.smooth_scroll_delta.y,
            )
        });
        if let Some(pos) = cursor {
            let local = pos - response.rect.min;
            if response.hovered() {
                editor.hover(*camera, points, local);
                if pressed && !editor.press(*camera, points, local) {
                    sheet.press(*camera, local);
                }
                if scroll != 0.0 && !down {
                    SheetState::zoom(camera, local, scroll);
                }
            } else {
                editor.hover = None;
            }
            if down || released {
                if editor.is_dragging() {
                    update = editor.drag(*camera, points, local);
                } else {
                    sheet.drag(camera, local);
                }
            }
            if response.hovered() || editor.is_dragging() {
                response.on_hover_cursor(if editor.selected().is_some() {
                    CursorIcon::PointingHand
                } else {
                    CursorIcon::Grab
                });
            }
        }
        if released || !down {
            editor.release();
            sheet.release();
        }
        Self::grid(&painter, *camera);
        (painter, update)
    }

    fn grid(painter: &Painter, camera: Camera) {
        if camera.scale <= 20.0 {
            return;
        }
        let alpha = ((camera.scale - 20.0) / 50.0).min(1.0) * 0.5;
        let stroke = Stroke::new(1.0_f32, Design::negative_color().gamma_multiply(alpha));
        let rect = painter.clip_rect();
        let top_left = camera.view_to_world(Vec2::ZERO);
        let bottom_right = camera.view_to_world(rect.size());
        // Index from the first visible line: never increment large f32 world coordinates by one.
        let first_x = camera.world_to_view(Vec2::new(top_left.x.ceil(), 0.0)).x;
        let first_y = camera.world_to_view(Vec2::new(0.0, top_left.y.floor())).y;
        for i in 0..=((bottom_right.x - top_left.x).abs().ceil() as usize).min(4096) {
            let x = rect.left() + first_x + i as f32 * camera.scale;
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                stroke,
            );
        }
        for i in 0..=((top_left.y - bottom_right.y).abs().ceil() as usize).min(4096) {
            let y = rect.top() + first_y + i as f32 * camera.scale;
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                stroke,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point_editor::point::MultiIndex;
    use i_triangle::i_overlay::i_float::int::{point::IntPoint, rect::IntRect};

    fn button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: Default::default(),
        }
    }

    #[test]
    fn pointer_drag_edits_points_without_panning_and_background_drag_pans() {
        let ctx = egui::Context::default();
        let mut camera = Camera::new(IntRect::new(-100, 100, -100, 100), Vec2::new(800.0, 600.0));
        let mut points = vec![EditorPoint {
            pos: IntPoint::new(0, 0),
            index: MultiIndex {
                group_index: 0,
                path_index: 0,
                point_index: 0,
            },
        }];
        let mut sheet = SheetState::default();
        let mut editor = PointsEditorState::default();
        let mut frame = |events: Vec<egui::Event>| {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        Vec2::new(800.0, 600.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| {
                    let (_, update) =
                        SheetWidget::show(ui, &mut camera, &points, &mut sheet, &mut editor);
                    if let Some(update) = update {
                        points[update.index] = update.point;
                    }
                },
            );
            (camera, points[0].pos)
        };
        frame(vec![]);
        let center = egui::pos2(400.0, 300.0);
        frame(vec![egui::Event::PointerMoved(center)]);
        frame(vec![button(center, true)]);
        let moved = center + Vec2::new(30.0, -15.0);
        let (after, point) = frame(vec![egui::Event::PointerMoved(moved)]);
        assert_eq!(after.pos, Vec2::ZERO);
        assert_eq!(point, IntPoint::new(20, 10));
        frame(vec![button(moved, false)]);
        let background = egui::pos2(700.0, 500.0);
        frame(vec![egui::Event::PointerMoved(background)]);
        frame(vec![button(background, true)]);
        let (after, point) = frame(vec![egui::Event::PointerMoved(
            background + Vec2::new(30.0, 15.0),
        )]);
        assert_eq!(after.pos, Vec2::new(-20.0, 10.0));
        assert_eq!(point, IntPoint::new(20, 10));
        frame(vec![button(background + Vec2::new(30.0, 15.0), false)]);
        let (after, _) = frame(vec![egui::Event::PointerMoved(background)]);
        assert_eq!(after.pos, Vec2::new(-20.0, 10.0));
    }
}
