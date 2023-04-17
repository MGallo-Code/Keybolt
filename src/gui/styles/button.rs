use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Vector};

use crate::gui::styles::types::palette::get_colors;
use crate::gui::styles::types::style_tuple::StyleTuple;

use super::types::element_type::ElementType;

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
            background: Some(Background::Color(
                match self.1 {
                    ElementType::NavButton => colors.nav_bg,
                    ElementType::NavButtonSelected => colors.secondary_accent,
                    ElementType::ItemListEntry => colors.item_list_bg,
                    ElementType::SelectedItem => colors.selected_item_bg,
                    ElementType::Button => colors.buttons,
                    _ => colors.secondary_accent,
                }
            )),
            border_radius: 0.0,
            border_width: 0.0,
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self.1 {
                ElementType::NavButton => colors.nav_text,
                ElementType::NavButtonSelected => colors.nav_bg,
                ElementType::Button => colors.button_text,
                _ => colors.primary_text,
            },
            border_color: colors.border,
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            shadow_offset: Vector::new(0.0, 2.0),
            background: Some(Background::Color(colors.primary_accent)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: colors.border,
            text_color: match self.1 {
                ElementType::NavButton => colors.nav_bg,
                ElementType::Button => colors.button_text,
                _ => colors.primary_text,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::StyleSheet::active(self, style)
    }
}