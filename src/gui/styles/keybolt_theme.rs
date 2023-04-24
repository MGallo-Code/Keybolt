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
            KeyboltTheme::Light => KeyboltPalette::GREEN,
            KeyboltTheme::Dark => KeyboltPalette::BLUE,
            KeyboltTheme::Vibrant => KeyboltPalette::BLUE,
            KeyboltTheme::Fjord => KeyboltPalette::BLUE,
        }
    }
}

pub struct KeyboltPalette {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub text_light: Color,
    pub text_dark: Color,
}

impl KeyboltPalette {
    pub const GREEN: Self = Self {
        primary: Color::from_rgb(
            0x47 as f32 / 255.0,
            0x7c as f32 / 255.0,
            0x47 as f32 / 255.0,
        ),
        secondary: Color::from_rgb(
            0x32 as f32 / 255.0,
            0x47 as f32 / 255.0,
            0x31 as f32 / 255.0,
        ),
        tertiary: Color::WHITE,
        text_dark: Color::BLACK,
        text_light: Color::WHITE,
    };

    pub const BLUE: Self = Self {
        primary: Color::from_rgb(
            0xFF as f32 / 255.0,
            0xFF as f32 / 255.0,
            0xFF as f32 / 255.0,
        ),
        secondary: Color::from_rgb(
            0x00 as f32 / 255.0,
            0x00 as f32 / 255.0,
            0x00 as f32 / 255.0,
        ),
        tertiary: Color::BLACK,
        text_dark: Color::WHITE,
        text_light: Color::BLACK,
    };
}