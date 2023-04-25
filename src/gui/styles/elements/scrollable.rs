use crate::gui::styles::keybolt_theme::KeyboltTheme;
use iced::widget::scrollable::{self, Scrollbar};
use iced::Color;

#[derive(Default)]
pub enum ScrollableStyle {
    #[default]
    Primary,
}

impl scrollable::StyleSheet for KeyboltTheme {
    type Style = ScrollableStyle;

    fn active(&self, _style: &Self::Style) -> Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(self.palette().secondary)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.palette().primary,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(self.palette().secondary)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.palette().primary,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        self.hovered(style, true)
    }

    fn active_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.active(style)
    }

    fn hovered_horizontal(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> Scrollbar {
        self.hovered(style, is_mouse_over_scrollbar)
    }

    fn dragging_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.hovered_horizontal(style, true)
    }
}
