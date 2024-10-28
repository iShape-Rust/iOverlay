use iced::{Background, border, Color, Padding, Theme};
use iced::theme::palette;
use iced::widget::button::{Status, Style};
use iced::widget::rule::{FillMode, Style as RuleStyle};
use iced::widget::container::Style as ContainerStyle;
use iced::window::Position::Default;
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

pub(super) fn style_action_button(theme: &Theme, status: Status) -> Style {
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

pub(super) fn style_action_button_selected(theme: &Theme, status: Status) -> Style {
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

pub(super) fn style_separator(theme: &Theme) -> RuleStyle {
    let color = if theme.extended_palette().is_dark {
        Color::new(0.0, 0.0, 0.0, 0.8)
    } else {
        Color::new(1.0, 1.0, 1.0, 0.8)
    };

    RuleStyle {
        color: color,
        width: 1,
        radius: border::Radius::new(0),
        fill_mode: FillMode::Padded(0),
    }
}

pub(super) fn style_second_background(theme: &Theme) -> ContainerStyle {
    ContainerStyle::default().background(theme.extended_palette().background.weak.color.scale_alpha(0.1))
}

pub(super) fn style_sheet_background(theme: &Theme) -> ContainerStyle {
    let color = if theme.extended_palette().is_dark {
        Color::new(0.2, 0.1, 0.1, 0.9)
    } else {
        Color::new(1.0, 1.0, 1.0, 0.9)
    };
    ContainerStyle::default().background(color)
}

pub(super) fn style_red_background(theme: &Theme) -> ContainerStyle {
    ContainerStyle::default().background(Color::new(1.0, 0.4, 0.4, 1.0))
}

pub(super) fn style_green_background(theme: &Theme) -> ContainerStyle {
    ContainerStyle::default().background(Color::new(0.4, 1.0, 0.4, 1.0))
}

pub(super) fn style_blue_background(theme: &Theme) -> ContainerStyle {
    ContainerStyle::default().background(Color::new(0.4, 0.4, 1.0, 1.0))
}