use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::container;

#[derive(Default)]
pub enum ContainerStyle {
    #[default]
    Primary,
    Secondary,
}

impl container::StyleSheet for KeyboltTheme {
    type Style = ContainerStyle;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(
                match _style {
                    ContainerStyle::Primary => iced::Background::Color(self.palette().primary),
                    ContainerStyle::Secondary => iced::Background::Color(self.palette().secondary),
                }
            ),
            text_color: Some(self.palette().background),
            ..Default::default()
        }
    }
}
