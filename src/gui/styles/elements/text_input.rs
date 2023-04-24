use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::text_input;
use iced::{color, Color};

impl text_input::StyleSheet for KeyboltTheme {
    type Style = ();

    /// Produces the style of an active text input.
    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(color!(0x477c47)),
            border_color: color!(0x324731),
            border_width: 1.0,
            border_radius: 5.0,
            icon_color: color!(0x477c47),
        }
    }

    /// Produces the style of a focused text input.
    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(color!(0x477c47)),
            border_color: color!(0xFFFFFF),
            border_width: 2.0,
            border_radius: 5.0,
            icon_color: color!(0x477c47),
        }
    }

    /// Produces the [`Color`] of the placeholder of a text input.
    fn placeholder_color(&self, style: &Self::Style) -> Color {
        color!(0, 0, 0)
    }

    /// Produces the [`Color`] of the value of a text input.
    fn value_color(&self, style: &Self::Style) -> Color {
        color!(255, 255, 255)
    }

    /// Produces the [`Color`] of the value of a disabled text input.
    fn disabled_color(&self, style: &Self::Style) -> Color {
        color!(255, 255, 255)
    }

    /// Produces the [`Color`] of the selection of a text input.
    fn selection_color(&self, style: &Self::Style) -> Color {
        color!(255, 255, 255)
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }

    /// Produces the style of a disabled text input.
    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.disabled(style)
    }
}