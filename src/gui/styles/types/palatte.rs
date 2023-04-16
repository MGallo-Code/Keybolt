use iced::Color;

use crate::gui::styles::types::style_types::Theme;
use crate::gui::styles::style_constants::{
    LIGHT_STYLE,
    DARK_STYLE,
};

/// Set of colors to apply to GUI
///
/// Best practices:
/// - `primary` should be a kind of neutral color
/// - `primary` and `buttons` should be similar colors
/// - `secondary` and one of `incoming` or `outgoing` should be the same color
/// - `incoming` and `outgoing` should be complementary colors if possible
/// - `text_headers` should be black or white and must have a strong contrast with `secondary`
/// - `text_body` should be black or white and must have a strong contrast with `primary`
pub struct Palette {
    /// Main color of the GUI (background, hovered buttons, active tab)
    pub primary: Color,
    /// Secondary color of the GUI (header, footer, buttons' borders, radio selection)
    pub secondary: Color,
    /// Color of active buttons (when not hovered) and inactive tabs
    pub buttons: Color,
    /// Color of incoming connections
    pub incoming: Color,
    /// Color of outgoing connections
    pub outgoing: Color,
    /// Color of header and footer text
    pub text_headers: Color,
    /// Color of body and buttons text
    pub text_body: Color,
    /// Color of round container borders and scrollbar borders
    pub round_borders: Color,
    /// Color of round containers
    pub round_containers: Color,
}

pub fn get_colors(style: Theme) -> Palette {
    match style {
        Theme::Light => LIGHT_STYLE,
        Theme::Dark => DARK_STYLE,
    }
}

/// Returns the average of two colors; color intensity is fixed to 100%
pub fn mix_colors(color_1: Color, color_2: Color) -> Color {
    Color {
        r: (color_1.r + color_2.r) / 2.0,
        g: (color_1.g + color_2.g) / 2.0,
        b: (color_1.b + color_2.b) / 2.0,
        a: 1.0,
    }
}

impl Default for Palette {
    fn default() -> Self {
        get_colors(Theme::Light)
    }
}