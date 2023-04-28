use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::text;

#[derive(Default, Copy, Clone)]
pub enum TextStyle {
    #[default]
    Primary,
    NavHeader,
    DetailsTitle,
    EntryInputTitle
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
                color: Some(self.palette().light_text),
                ..text::Appearance::default()
            },
            TextStyle::EntryInputTitle => text::Appearance {
                color: match self {
                    KeyboltTheme::Dark => Some(self.palette().light_text),
                    _ => Some(self.palette().text),
                },
                ..text::Appearance::default()
            }
        }
    }
}
