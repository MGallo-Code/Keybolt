use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::{application, color, Color};

impl application::StyleSheet for KeyboltTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0x477c47),
            text_color: Color::BLACK,
        }
    }
}