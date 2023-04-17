use iced::{Color, Font};
use crate::gui::styles::types::palette::Palette;

pub const DEFAULT_STYLE: Palette = Palette {
    nav_bg: Color {
        r: 0.05,
        g: 0.11,
        b: 0.16,
        a: 1.0,
    },
    item_list_bg: Color {
        r: 0.97,
        g: 0.97,
        b: 0.97,
        a: 1.0,
    },
    selected_item_bg: Color {
        r: 0.87,
        g: 0.87,
        b: 0.87,
        a: 1.0,
    },
    third_col_bg: Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    nav_text: Color {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    },
    primary_text: Color {
        r: 0.15,
        g: 0.15,
        b: 0.15,
        a: 1.0,
    },
    primary_accent: Color {
        r: 0.2,
        g: 0.6,
        b: 0.86,
        a: 1.0,
    },
    secondary_accent: Color {
        r: 0.6,
        g: 0.6,
        b: 0.6,
        a: 1.0,
    },
    buttons: Color {
        r: 0.2,
        g: 0.6,
        b: 0.86,
        a: 1.0,
    },
    button_text: Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    border: Color {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    },
};


pub const JOSEFIN_SANS_REG: Font = Font::External {
    name: "josefin-sans-regular",
    bytes: include_bytes!("../../../resources/fonts/JosefinSans-Regular.ttf"),
};

// font sizes
pub const FONT_SIZE_NAV: f32 = 25.0;
pub const FONT_SIZE_NAV_TITLE: f32 = 27.0;
pub const FONT_SIZE_BODY: f32 = 21.0;