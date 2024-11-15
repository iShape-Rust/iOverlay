use iced::widget::button;
use iced::widget::container;
use iced::widget::rule;
use iced::widget::text;
use iced::{Background, border, Color, Padding, Theme};


pub(super) struct Design {
    pub(super) action_separator: f32,
}

impl Design {
    pub(crate) fn solution_color() -> Color {
        Color::from_rgb8(32, 199, 32)
    }

    pub(crate) fn subject_color() -> Color {
        Color::from_rgb8(255, 51, 51)
    }

    pub(crate) fn clip_color() -> Color {
        Color::from_rgb8(26, 142, 255)
    }

    pub(crate) fn negative_color() -> Color {
        if iced::Theme::default().extended_palette().is_dark {
            Color::from_rgb8(224, 224, 224)
        } else {
            Color::from_rgb8(32, 32, 32)
        }
    }

    pub(crate) fn accent_color() -> Color {
        Color::from_rgb8(255, 140, 0)
    }

    pub(crate) fn both_color() -> Color {
        Color::from_rgb8(76, 217, 100)
    }

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

// Sidebar

pub(super) fn style_sidebar_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let base = button::Style {
        background: Some(Background::Color(palette.primary.strong.color)),
        text_color: palette.primary.strong.text,
        border: border::rounded(6),
        ..button::Style::default()
    };

    match status {
        button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.background.weak.color.scale_alpha(0.2))),
            ..base
        },
        button::Status::Disabled | button::Status::Active => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..base
        },
    }
}

pub(super) fn style_sidebar_button_selected(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let base = button::Style {
        background: Some(Background::Color(palette.primary.strong.color)),
        text_color: palette.primary.strong.text,
        border: border::rounded(6),
        ..button::Style::default()
    };

    match status {
        button::Status::Pressed | button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..base
        },
    }
}

pub(super) fn style_sidebar_text(theme: &Theme) -> text::Style {
    let palette = theme.palette();
    text::Style {
        color: Some(palette.text.scale_alpha(0.7))
    }
}

pub(super) fn style_sidebar_text_selected(theme: &Theme) -> text::Style {
    let palette = theme.palette();
    text::Style {
        color: Some(palette.text)
    }
}

pub(super) fn style_sidebar_background(theme: &Theme) -> container::Style {
    container::Style::default().background(theme.extended_palette().background.weak.color.scale_alpha(0.1))
}

pub(super) fn style_separator(theme: &Theme) -> rule::Style {
    let color = if theme.extended_palette().is_dark {
        Color::new(0.0, 0.0, 0.0, 0.8)
    } else {
        Color::new(1.0, 1.0, 1.0, 0.8)
    };

    rule::Style {
        color,
        width: 1,
        radius: border::Radius::new(0),
        fill_mode: rule::FillMode::Padded(0),
    }
}

pub(super) fn style_sheet_background(theme: &Theme) -> container::Style {
    if theme.extended_palette().is_dark {
        container::Style::default().background(Color::BLACK.scale_alpha(0.4))
    } else {
        container::Style::default().background(Color::WHITE.scale_alpha(0.4))
    }
}