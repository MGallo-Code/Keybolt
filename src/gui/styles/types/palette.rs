use iced::Color;
use crate::gui::styles::style_constants::{
    DEFAULT_STYLE, DARK_STYLE, VIBRANT_STYLE, FJORD_STYLE,
};


pub struct Palette {
    /// Background color for the navigation column
    pub nav_bg: Color,
    /// Background color for the item list column
    pub item_list_bg: Color,
    /// Background color for the selected item in the list
    pub selected_item_bg: Color,
    /// Background color for the third column displaying details
    pub third_col_bg: Color,

    /// Text color for the navigation header and buttons
    pub nav_text: Color,
    /// Primary text color for the item list and third column
    pub primary_text: Color,

    /// Primary accent color for selected navigation buttons and important information
    pub primary_accent: Color,
    /// Secondary accent color for hover effects or less important information
    pub secondary_accent: Color,

    /// Color for buttons or other interactive elements
    pub buttons: Color,
    /// Color for button text
    pub button_text: Color,

    /// Color for borders and separators
    pub border: Color,
}

// pub fn get_colors(style: StyleType) -> Palette {
//     match style {
//         StyleType::Default => DEFAULT_STYLE,
//         StyleType::Dark => DARK_STYLE,
//         StyleType::Vibrant => VIBRANT_STYLE,
//         StyleType::Fjord => FJORD_STYLE,
//     }
// }

// impl Default for Palette {
//     fn default() -> Self {
//         get_colors(StyleType::Dark)
//     }
// }
