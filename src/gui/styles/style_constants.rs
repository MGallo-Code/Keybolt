//! Module defining the constants used for aesthetic purposes (colors, borders...)
use iced::{Color};
use crate::gui::styles::types::palatte::Palette;

const PRIMARY_DARK: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const SECONDARY_DARK: Color = Color {
    r: 0.7,
    g: 0.35,
    b: 0.0,
    a: 1.0,
};
const BUTTONS_DARK: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};
pub const DARK_STYLE: Palette = Palette {
    primary: PRIMARY_DARK,
    secondary: SECONDARY_DARK,
    buttons: BUTTONS_DARK,
    incoming: SECONDARY_DARK,
    outgoing: SECONDARY_LIGHT,
    text_headers: Color::BLACK,
    text_body: Color::WHITE,
    round_borders: Color {
        a: 0.3,
        ..Color::BLACK
    },
    round_containers: Color {
        a: 0.2,
        ..Color::BLACK
    },
};

const PRIMARY_LIGHT: Color = Color::WHITE;
const SECONDARY_LIGHT: Color = Color {
    r: 0.0,
    g: 0.35,
    b: 0.7,
    a: 1.0,
};
const BUTTONS_LIGHT: Color = Color {
    r: 0.8,
    g: 0.8,
    b: 0.8,
    a: 1.0,
};
pub const LIGHT_STYLE: Palette = Palette {
    primary: PRIMARY_LIGHT,
    secondary: SECONDARY_LIGHT,
    buttons: BUTTONS_LIGHT,
    incoming: SECONDARY_LIGHT,
    outgoing: SECONDARY_DARK,
    text_headers: Color::WHITE,
    text_body: Color::BLACK,
    round_borders: Color {
        a: 0.25,
        ..Color::BLACK
    },
    round_containers: Color {
        a: 0.1,
        ..Color::BLACK
    },
};

// font sizes
pub const FONT_SIZE_FOOTER: f32 = 18.0;
pub const FONT_SIZE_BODY: f32 = 21.0;
pub const FONT_SIZE_SUBTITLE: f32 = 23.0;
pub const FONT_SIZE_TITLE: f32 = 25.0;

// border styles
pub const BORDER_WIDTH: f32 = 2.0;
pub const CHARTS_LINE_BORDER: u32 = 1;
pub const BORDER_ROUNDED_RADIUS: f32 = 15.0;
pub const BORDER_BUTTON_RADIUS: f32 = 180.0;

// stars yellow colors
pub const STARRED: Color = Color {
    r: 245.0 / 255.0,
    g: 193.0 / 255.0,
    b: 39.0 / 255.0,
    a: 1.0,
};
