use iced::{Background, border, Color, Padding, Theme};
use iced::theme::palette;
use iced::widget::button::{Status, Style};
use crate::app::main::EditorApp;

pub(super) struct Design {
    pub(super) action_separator: f32,
}

impl Design {
    pub(super) fn new() -> Self {
        Self {
            action_separator: 3.0
        }
    }

    pub(super) fn action_padding(&self) -> Padding {
        Padding {
            top: self.action_separator,
            right: 2.0 * self.action_separator,
            bottom: self.action_separator,
            left: 2.0 * self.action_separator,
        }
    }
}

pub(super) fn action_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        background: Some(Background::Color(palette.primary.strong.color)),
        text_color: palette.primary.strong.text,
        border: border::rounded(6),
        ..Style::default()
    };

    match status {
        Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.background.weak.color.scale_alpha(0.2))),
            ..base
        },
        Status::Disabled | Status::Active => Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..base
        },
    }
}

pub(super) fn action_button_selected(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        background: Some(Background::Color(palette.primary.strong.color)),
        text_color: palette.primary.strong.text,
        border: border::rounded(6),
        ..Style::default()
    };

    match status {
        Status::Pressed | Status::Active => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..base
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..base
        },
    }
}
