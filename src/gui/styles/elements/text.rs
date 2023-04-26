use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::text;

#[derive(Default, Copy, Clone)]
pub enum TextStyle {
    #[default]
    Primary,
    NavHeader,
    DetailsTitle,
}

impl text::StyleSheet for KeyboltTheme {
    type Style = TextStyle;

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        match _style {
            TextStyle::Primary => text::Appearance {
                ..text::Appearance::default()
            },
            TextStyle::NavHeader => text::Appearance {
                color: Some(self.palette().accent),
                ..text::Appearance::default()
            },
            TextStyle::DetailsTitle => text::Appearance {
                color: Some(self.palette().secondary),
                ..text::Appearance::default()
            },
        }
    }
}
