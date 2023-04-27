use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyboltTheme {
    #[default]
    Light,
    Dark,
    Vibrant,
    Fjord,
}

impl KeyboltTheme {
    pub fn palette(&self) -> KeyboltPalette {
        match self {
            KeyboltTheme::Light => KeyboltPalette::LIGHT,
            KeyboltTheme::Dark => KeyboltPalette::DARK,
            KeyboltTheme::Vibrant => KeyboltPalette::VIBRANT,
            KeyboltTheme::Fjord => KeyboltPalette::FJORD,
        }
    }
}

pub struct KeyboltPalette {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub muted_accent: Color,
    pub background: Color,
    pub text: Color,
    pub light_text: Color,
    pub subdued_text: Color,
    pub active_button: Color,
    pub active_entry_button: Color,
}

impl KeyboltPalette {
    pub const LIGHT: Self = Self {
        primary: Color::from_rgb(
            0x3C as f32 / 255.0,
            0x4F as f32 / 255.0,
            0x76 as f32 / 255.0,
        ),
        secondary: Color::from_rgb(
            0x65 as f32 / 255.0,
            0x7E as f32 / 255.0,
            0xA8 as f32 / 255.0,
        ),
        accent: Color::from_rgb(
            0xF2 as f32 / 255.0,
            0xC1 as f32 / 255.0,
            0x4E as f32 / 255.0,
        ),
        muted_accent: Color::from_rgb(
            0xF8 as f32 / 255.0 * 0.9 + 0xF2 as f32 / 255.0 * 0.1,
            0xF9 as f32 / 255.0 * 0.9 + 0xC1 as f32 / 255.0 * 0.1,
            0xFB as f32 / 255.0 * 0.9 + 0x4E as f32 / 255.0 * 0.1,
        ),
        background: Color::from_rgb(
            0xF8 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFB as f32 / 255.0,
        ),
        text: Color::from_rgb(
            0x21 as f32 / 255.0,
            0x25 as f32 / 255.0,
            0x29 as f32 / 255.0,
        ),
        light_text: Color::from_rgb(
            0xF8 as f32 / 255.0 * 0.9 + 0xF2 as f32 / 255.0 * 0.1,
            0xF9 as f32 / 255.0 * 0.9 + 0xC1 as f32 / 255.0 * 0.1,
            0xFB as f32 / 255.0 * 0.9 + 0x4E as f32 / 255.0 * 0.1,
        ),
        subdued_text: Color::from_rgb(
            0x6C as f32 / 255.0,
            0x75 as f32 / 255.0,
            0x7D as f32 / 255.0,
        ),
        active_button: Color::from_rgb(
            0x9A as f32 / 255.0,
            0xA2 as f32 / 255.0,
            0xB6 as f32 / 255.0,
        ),
        active_entry_button: Color::from_rgb(
            0x9A as f32 / 255.0,
            0xA2 as f32 / 255.0,
            0xB6 as f32 / 255.0,
        ),
    };

    pub const DARK: Self = Self {
        primary: Color::from_rgb(
            0x3C as f32 / 255.0,
            0x4F as f32 / 255.0,
            0x76 as f32 / 255.0,
        ),
        secondary: Color::from_rgb(
            0x65 as f32 / 255.0,
            0x7E as f32 / 255.0,
            0xA8 as f32 / 255.0,
        ),
        accent: Color::from_rgb(
            0xF2 as f32 / 255.0,
            0xC1 as f32 / 255.0,
            0x4E as f32 / 255.0,
        ),
        muted_accent: Color::from_rgb(
            0x2D as f32 / 255.0,
            0x2C as f32 / 255.0,
            0x2B as f32 / 255.0,
        ),
        background: Color::from_rgb(
            0xF8 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFB as f32 / 255.0,
        ),
        text: Color::from_rgb(
            0x21 as f32 / 255.0,
            0x25 as f32 / 255.0,
            0x29 as f32 / 255.0,
        ),
        light_text: Color::from_rgb(
            0xF8 as f32 / 255.0 * 0.9 + 0xF2 as f32 / 255.0 * 0.1,
            0xF9 as f32 / 255.0 * 0.9 + 0xC1 as f32 / 255.0 * 0.1,
            0xFB as f32 / 255.0 * 0.9 + 0x4E as f32 / 255.0 * 0.1,
        ),
        subdued_text: Color::from_rgb(
            0x6C as f32 / 255.0,
            0x75 as f32 / 255.0,
            0x7D as f32 / 255.0,
        ),
        active_button: Color::from_rgb(
            0x9A as f32 / 255.0,
            0xA2 as f32 / 255.0,
            0xB6 as f32 / 255.0,
        ),
        active_entry_button: Color::from_rgb(
            0x63 as f32 / 255.0,
            0x68 as f32 / 255.0,
            0x76 as f32 / 255.0,
        ),
    };

    pub const VIBRANT: Self = Self {
        primary: Color::from_rgb(0.0, 0.5, 0.8),
        secondary: Color::from_rgb(0.2, 0.2, 0.2),
        accent: Color::from_rgb(1.0, 0.3, 0.3),
        muted_accent: Color::from_rgb(0.8, 0.5, 0.5),
        background: Color::from_rgb(0.9, 0.9, 0.9),
        text: Color::from_rgb(0.1, 0.1, 0.1),
        light_text: Color::from_rgb(0.3, 0.3, 0.3),
        subdued_text: Color::from_rgb(0.5, 0.5, 0.5),
        active_button: Color::from_rgb(0.2, 0.2, 0.2),
        active_entry_button: Color::from_rgb(0.2, 0.2, 0.2),
    };

    pub const FJORD: Self = Self {
        primary: Color::from_rgb(0.0, 0.4, 0.6),
        secondary: Color::from_rgb(0.2, 0.3, 0.4),
        accent: Color::from_rgb(0.8, 0.4, 0.2),
        muted_accent: Color::from_rgb(0.6, 0.6, 0.5),
        background: Color::from_rgb(0.9, 0.9, 0.9),
        text: Color::from_rgb(0.1, 0.1, 0.1),
        light_text: Color::from_rgb(0.3, 0.3, 0.3),
        subdued_text: Color::from_rgb(0.5, 0.5, 0.5),
        active_button: Color::from_rgb(0.2, 0.2, 0.2),
        active_entry_button: Color::from_rgb(0.2, 0.2, 0.2),
    };    
}