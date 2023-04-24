use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::container;
use iced::Color;

impl container::StyleSheet for KeyboltTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
                text_color: Some(Color::WHITE),
                ..Default::default()
        }
    }
}