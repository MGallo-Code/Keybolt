use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::text;

impl text::StyleSheet for KeyboltTheme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance::default()
    }
}
