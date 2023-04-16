//! Buttons style

use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Vector};

use crate::gui::styles::types::palette::get_colors;
use crate::gui::styles::style_constants::{BORDER_WIDTH};
use crate::gui::styles::types::palette::mix_colors;
use crate::gui::styles::types::style_tuple::StyleTuple;

impl From<StyleTuple> for iced::theme::Button {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Button::Custom(Box::new(tuple))
    }
}

impl button::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            background: Some(Background::Color(colors.buttons)),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: colors.text_body,
            border_color: colors.secondary,
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            shadow_offset: Vector::new(0.0, 2.0),
            background: Some(Background::Color(mix_colors(colors.primary, colors.buttons))),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            text_color: colors.text_body,
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::StyleSheet::active(self, style)
    }
}