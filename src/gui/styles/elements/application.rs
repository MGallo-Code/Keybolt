use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::application;

impl application::StyleSheet for KeyboltTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().background,
            text_color: self.palette().text,
        }
    }
}
