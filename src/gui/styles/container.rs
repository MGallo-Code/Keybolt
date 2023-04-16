//! Containers style

use iced::widget::container::Appearance;
use iced::Theme;
use iced::Background;

use crate::gui::styles::types::palette::get_colors;
use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::gui::styles::types::style_tuple::StyleTuple;

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
            text_color: Some(colors.text_body),
            background: Some(Background::Color(colors.primary)),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.primary,
        }
    }
}
