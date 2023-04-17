use iced::{Color, Font};
use crate::gui::styles::types::palette::Palette;

pub const DEFAULT_STYLE: Palette = Palette {
    nav_bg: Color {
        r: 0.1,
        g: 0.18,
        b: 0.24,
        a: 1.0,
    },
    item_list_bg: Color {
        r: 0.95,
        g: 0.95,
        b: 0.95,
        a: 1.0,
    },
    selected_item_bg: Color {
        r: 0.9,
        g: 0.9,
        b: 0.9,
        a: 1.0,
    },
    third_col_bg: Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    nav_text: Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
    },
    primary_text: Color {
        r: 0.15,
        g: 0.15,
        b: 0.15,
        a: 1.0,
    },
    primary_accent: Color {
        r: 0.3,
        g: 0.6,
        b: 0.8,
        a: 1.0,
    },
    secondary_accent: Color {
        r: 0.6,
        g: 0.6,
        b: 0.6,
        a: 1.0,
    },
    buttons: Color {
        r: 0.3,
        g: 0.6,
        b: 0.8,
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

pub const DARK_STYLE: Palette = Palette {
    nav_bg: Color {
        r: 0.12,
        g: 0.12,
        b: 0.16,
        a: 1.0,
    },
    item_list_bg: Color {
        r: 0.2,
        g: 0.2,
        b: 0.25,
        a: 1.0,
    },
    selected_item_bg: Color {
        r: 0.25,
        g: 0.25,
        b: 0.3,
        a: 1.0,
    },
    third_col_bg: Color {
        r: 0.16,
        g: 0.16,
        b: 0.2,
        a: 1.0,
    },
    nav_text: Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
    },
    primary_text: Color {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    },
    primary_accent: Color {
        r: 0.4,
        g: 0.6,
        b: 0.8,
        a: 1.0,
    },
    secondary_accent: Color {
        r: 0.6,
        g: 0.6,
        b: 0.6,
        a: 1.0,
    },
    buttons: Color {
        r: 0.4,
        g: 0.6,
        b: 0.8,
        a: 1.0,
    },
    button_text: Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    border: Color {
        r: 0.3,
        g: 0.3,
        b: 0.3,
        a: 1.0,
    },
};

pub const VIBRANT_STYLE: Palette = Palette {
    nav_bg: Color {
        r: 0.18,
        g: 0.2,
        b: 0.25,
        a: 1.0,
    },
    item_list_bg: Color {
        r: 0.95,
        g: 0.95,
        b: 0.95,
        a: 1.0,
    },
    selected_item_bg: Color {
        r: 0.9,
        g: 0.9,
        b: 0.9,
        a: 1.0,
    },
    third_col_bg: Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    nav_text: Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
    },
    primary_text: Color {
        r: 0.15,
        g: 0.15,
        b: 0.15,
        a: 1.0,
    },
    primary_accent: Color {
        r: 0.85,
        g: 0.35,
        b: 0.35,
        a: 1.0,
    },
    secondary_accent: Color {
        r: 0.35,
        g: 0.85,
        b: 0.35,
        a: 1.0,
    },
    buttons: Color {
        r: 0.35,
        g: 0.35,
        b: 0.85,
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

pub const FJORD_STYLE: Palette = Palette {
    nav_bg: Color {
        r: 68.0 / 255.0,
        g: 89.0 / 255.0,
        b: 98.0 / 255.0,
        a: 1.0,
    },
    item_list_bg: Color {
        r: 0.95,
        g: 0.95,
        b: 0.95,
        a: 1.0,
    },
    selected_item_bg: Color {
        r: 0.9,
        g: 0.9,
        b: 0.9,
        a: 1.0,
    },
    third_col_bg: Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    nav_text: Color {
        r: 0.7,
        g: 0.7,
        b: 0.7,
        a: 1.0,
    },
    primary_text: Color {
        r: 0.15,
        g: 0.15,
        b: 0.15,
        a: 1.0,
    },
    primary_accent: Color {
        r: 45.0 / 255.0,
        g: 137.0 / 255.0,
        b: 149.0 / 255.0,
        a: 1.0,
    },
    secondary_accent: Color {
        r: 106.0 / 255.0,
        g: 168.0 / 255.0,
        b: 79.0 / 255.0,
        a: 1.0,
    },
    buttons: Color {
        r: 88.0 / 255.0,
        g: 88.0 / 255.0,
        b: 88.0 / 255.0,
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

// pub const RALEWAY_REG: Font = Font::External {
//     name: "raleway_reg",
//     bytes: include_bytes!("../../../resources/fonts/Raleway-Regular.ttf"),
// };

pub const RALEWAY_BOLD: Font = Font::External {
    name: "raleway_bold",
    bytes: include_bytes!("../../../resources/fonts/Raleway-Bold.ttf"),
};

// font sizes
pub const FONT_SIZE_NAV: f32 = 25.0;
pub const FONT_SIZE_NAV_TITLE: f32 = 27.0;
pub const FONT_SIZE_BODY: f32 = 21.0;