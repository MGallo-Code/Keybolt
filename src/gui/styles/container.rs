use iced::widget::container::Appearance;
use iced::Theme;
use iced::Background;

use crate::gui::styles::types::palette::get_colors;
use crate::gui::styles::types::style_tuple::StyleTuple;

use super::types::element_type::ElementType;

impl From<StyleTuple> for iced::theme::Container {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Container::Custom(Box::new(tuple))
    }
}

impl iced::widget::container::StyleSheet for StyleTuple {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            text_color: Some(
                match self.1 {
                    ElementType::NavHeader => colors.selected_item_bg,
                    _ => colors.primary_text,
                }
            ),
            background: Some(Background::Color(
                match self.1 {
                    ElementType::NavColumn => colors.nav_bg,
                    ElementType::ItemListColumn => colors.item_list_bg,
                    ElementType::DetailsColumn => colors.third_col_bg,
                    ElementType::NavHeader => colors.nav_bg,
                    _ => colors.primary_accent,
                }
            )),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: colors.border,
        }
    }
}
