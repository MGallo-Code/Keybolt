use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::Color;
use iced::widget::toggler;

#[derive(Default)]
pub enum TogglerStyle {
    #[default]
    Primary,
}

impl toggler::StyleSheet for KeyboltTheme {
    type Style = TogglerStyle;

    fn active(&self, _style: &Self::Style, active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: self.palette().active_button,
            background_border: Some(self.palette().accent),
            foreground: if active {
                self.palette().accent
            } else {
                self.palette().background
            },
            foreground_border: Some(Color::from_rgba8(0, 0, 0, 0.0)),
        }
    }

    fn hovered(&self, _style: &Self::Style, active: bool) -> toggler::Appearance {
        toggler::Appearance {
            background: self.palette().active_button,
            background_border: Some(self.palette().accent),
            foreground: if active {
                self.palette().accent
            } else {
                self.palette().background
            },
            foreground_border: Some(self.palette().accent),
        }
    }
}