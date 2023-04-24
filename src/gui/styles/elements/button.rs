use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::button;
use iced::{color, Color};

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Default,
    New,
}

impl button::StyleSheet for KeyboltTheme {
    type Style = ButtonStyle;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(
                match _style {
                    ButtonStyle::Default => iced::Background::Color(self.palette().primary),
                    ButtonStyle::New => iced::Background::Color(self.palette().secondary),
                }
            ),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(color!(0x324731))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}
