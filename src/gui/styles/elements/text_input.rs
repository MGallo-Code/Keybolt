use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::text_input;
use iced::Color;

#[derive(Default)]
pub enum TextInputStyle {
    #[default]
    Primary,
    EntrySearchBar,
}

impl text_input::StyleSheet for KeyboltTheme {
    type Style = TextInputStyle;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(self.palette().background),
            border_color: self.palette().primary,
            border_width: 1.0,
            border_radius:
                match _style {
                    TextInputStyle::Primary => 5.0,
                    TextInputStyle::EntrySearchBar => 0.0,
                },
            icon_color: self.palette().primary,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(self.palette().background),
            border_color: self.palette().accent,
            border_width: 2.0,
            border_radius:
                match _style {
                    TextInputStyle::Primary => 5.0,
                    TextInputStyle::EntrySearchBar => 0.0,
                },
            icon_color: self.palette().accent,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette().subdued_text
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.palette().text
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.palette().subdued_text
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.palette().accent
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        self.focused(_style)
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        self.active(_style)
    }
}
