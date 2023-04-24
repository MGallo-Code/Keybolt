use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::scrollable::{self, Scrollbar};
use iced::{color, Color};

impl scrollable::StyleSheet for KeyboltTheme {
    type Style = ();

    /// Produces the style of an active scrollbar.
    fn active(&self, style: &Self::Style) -> Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(color!(0x477c47))),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: color!(0xFFFFFF),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    /// Produces the style of a scrollbar when the scrollable is being hovered.
    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(color!(0x477c47))),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: color!(0x324731),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    /// Produces the style of a scrollbar that is being dragged.
    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(color!(0x477c47))),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: color!(0x324731),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    /// Produces the style of an active horizontal scrollbar.
    fn active_horizontal(&self, style: &Self::Style) -> Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(color!(0x477c47))),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: color!(0x324731),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    /// Produces the style of a horizontal scrollbar when the scrollable is being hovered.
    fn hovered_horizontal(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> Scrollbar {
        self.hovered(style, is_mouse_over_scrollbar)
    }

    /// Produces the style of a horizontal scrollbar that is being dragged.
    fn dragging_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.hovered_horizontal(style, true)
    }
}