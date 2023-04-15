use iced::Color;
use iced::theme::{Palette, Theme};

pub struct CustomTheme;

impl CustomTheme {
    pub const PALETTE: Palette = Palette {
        background: Color::from_rgb(0.9, 0.9, 0.9),
        text: Color::from_rgb(0.1, 0.1, 0.1),
        primary: Color::from_rgb(0.5, 0.5, 0.5),
        success: Color::from_rgb(0.2, 0.8, 0.2),
        danger: Color::from_rgb(0.8, 0.2, 0.2),
    };
    pub fn theme() -> Theme {
        Theme::custom(Self::PALETTE)
    }
}