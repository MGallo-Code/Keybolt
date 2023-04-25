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
            KeyboltTheme::Dark => KeyboltPalette::LIGHT,
            KeyboltTheme::Vibrant => KeyboltPalette::LIGHT,
            KeyboltTheme::Fjord => KeyboltPalette::LIGHT,
        }
    }
}

pub struct KeyboltPalette {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub text: Color,
    pub subdued_text: Color,
    pub active_button: Color,
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
    };
}