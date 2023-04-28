use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::container;

#[derive(Default)]
pub enum ContainerStyle {
    #[default]
    Primary,
    Secondary,
    EntryListContainer,
    EntryInputContainer,
}

impl container::StyleSheet for KeyboltTheme {
    type Style = ContainerStyle;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(
                match _style {
                    ContainerStyle::Primary => iced::Background::Color(self.palette().primary),
                    ContainerStyle::Secondary => iced::Background::Color(self.palette().secondary),
                    ContainerStyle::EntryListContainer => iced::Background::Color(self.palette().muted_accent),
                    ContainerStyle::EntryInputContainer => match self {
                        KeyboltTheme::Dark => iced::Background::Color(self.palette().secondary),
                        _ => iced::Background::Color(self.palette().muted_accent),
                    },
                }
            ),
            border_radius: match _style {
                ContainerStyle::EntryInputContainer => 8.5,
                _ => 0.0,
            },
            text_color: Some(self.palette().background),
            ..Default::default()
        }
    }
}
