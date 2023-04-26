use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::button;

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    NavButton(bool), // Pass active state as a boolean
    EntryListButton(bool), // Pass active state as a boolean
}

impl button::StyleSheet for KeyboltTheme {
    type Style = ButtonStyle;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let (background_color, text_color) = match _style {
            ButtonStyle::Primary => (self.palette().primary, self.palette().light_text),
            ButtonStyle::Secondary => (self.palette().secondary, self.palette().light_text),
            ButtonStyle::NavButton(active) => {
                if *active {
                    (self.palette().active_button, self.palette().light_text)
                } else {
                    (self.palette().secondary, self.palette().light_text)
                }
            }
            ButtonStyle::EntryListButton(active) => {
                if *active {
                    (self.palette().active_button, self.palette().text)
                } else {
                    (self.palette().muted_accent, self.palette().text)
                }
            }
        };

        button::Appearance {
            background: Some(iced::Background::Color(background_color)),
            text_color,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let (background_color, text_color) = match _style {
            ButtonStyle::Primary => (self.palette().active_button, self.palette().light_text),
            ButtonStyle::Secondary => (self.palette().active_button, self.palette().light_text),
            ButtonStyle::NavButton(active) => {
                if *active {
                    (self.palette().active_button, self.palette().light_text)
                } else {
                    (self.palette().active_button, self.palette().light_text)
                }
            }
            ButtonStyle::EntryListButton(active) => {
                if *active {
                    (self.palette().active_button, self.palette().light_text)
                } else {
                    (self.palette().muted_accent, self.palette().subdued_text)
                }
            }
        };

        let (border_width, border_color) = match _style {
            ButtonStyle::NavButton(_) => (2.0, self.palette().accent),
            ButtonStyle::EntryListButton(_) => (4.0, self.palette().accent),
            _ => (2.0, self.palette().accent),
        };

        button::Appearance {
            background: Some(iced::Background::Color(background_color)),
            text_color,
            border_width,
            border_color,
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}