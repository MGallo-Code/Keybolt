use iced::{button, Background, Color, container};

// An enumeration of the available themes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Default,
}

pub mod theme_colors {
    use iced::Color;

    // MAIN COLORS
    // (Background Color, Text Color)
    // light
    pub const LIGHT_PRIMARY: (Color, Color) = (Color::WHITE, Color::BLACK);
    pub const LIGHT_SECONDARY: (Color, Color) = (Color::BLACK, Color::WHITE);
    pub const LIGHT_TERNARY: (Color, Color) = (Color::from_rgb(0.0, 0.5, 0.0), Color::WHITE);
    // dark
    pub const DARK_PRIMARY: (Color, Color) = (Color::from_rgb(0.1, 0.1, 0.1), Color::WHITE);
    pub const DARK_SECONDARY: (Color, Color) = (Color::WHITE, Color::BLACK);
    pub const DARK_TERNARY: (Color, Color) = (Color::from_rgb(0.0, 0.5, 0.0), Color::WHITE);
    // default
    pub const DEF_PRIMARY: (Color, Color) = (Color::from_rgb(0.2, 0.2, 0.5), Color::WHITE);
    pub const DEF_SECONDARY: (Color, Color) = (Color::WHITE, Color::BLACK);
    pub const DEF_TERNARY: (Color, Color) = (Color::from_rgb(0.0, 0.5, 0.0), Color::WHITE);

    pub mod container_colors {
        use iced::Color;
        use crate::themes::theme_colors;

        pub const LIGHT_DEFAULT: (Color, Color) = theme_colors::LIGHT_PRIMARY;
        pub const DARK_DEFAULT: (Color, Color) = theme_colors::DARK_PRIMARY;
        pub const DEF_DEFAULT: (Color, Color) = theme_colors::DEF_PRIMARY;
        // NAV
        pub const LIGHT_NAV: (Color, Color) = theme_colors::LIGHT_PRIMARY;
        pub const DARK_NAV: (Color, Color) = theme_colors::DARK_PRIMARY;
        pub const DEF_NAV: (Color, Color) = theme_colors::DEF_PRIMARY;
    }

    pub mod button_colors {
        use iced::Color;
        use crate::themes::theme_colors;

        pub const LIGHT_DEFAULT: (Color, Color) = theme_colors::LIGHT_SECONDARY;
        pub const DARK_DEFAULT: (Color, Color) = theme_colors::DARK_SECONDARY;
        pub const DEF_DEFAULT: (Color, Color) = theme_colors::DEF_SECONDARY;
        pub const LIGHT_DEFAULT_HOVERED: (Color, Color) = (Color::from_rgb(0.1, 0.1, 0.1), Color::WHITE);
        pub const DARK_DEFAULT_HOVERED: (Color, Color) = (Color::from_rgb(0.9, 0.9, 0.9), Color::BLACK);
        pub const DEF_DEFAULT_HOVERED: (Color, Color) = (Color::from_rgb(0.9, 0.9, 0.9), Color::WHITE);
        // NAV
        pub const LIGHT_NAV: (Color, Color) = theme_colors::LIGHT_SECONDARY;
        pub const DARK_NAV: (Color, Color) = theme_colors::DARK_SECONDARY;
        pub const DEF_NAV: (Color, Color) = theme_colors::DEF_SECONDARY;
        pub const LIGHT_NAV_HOVERED: (Color, Color) = (Color::from_rgb(0.1, 0.1, 0.1), Color::WHITE);
        pub const DARK_NAV_HOVERED: (Color, Color) = (Color::from_rgb(0.9, 0.9, 0.9), Color::BLACK);
        pub const DEF_NAV_HOVERED: (Color, Color) = (Color::from_rgb(0.9, 0.9, 0.9), Color::WHITE);
    }
}


pub mod style {
    use iced::{button, Background, Color, container};
    use crate::themes::Theme;
    use crate::themes::theme_colors::{button_colors, container_colors};

    pub enum ButtonType {
        Default,
        Nav,
    }

    pub struct Button {
        theme: Theme,
        button_type: ButtonType,
    }

    impl Button {
        pub fn new(theme: Theme, button_type: ButtonType) -> Self {
            Button { theme, button_type }
        }
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let (bg_color, text_color) = match self.button_type {
                ButtonType::Default => match self.theme {
                    Theme::Light => button_colors::LIGHT_DEFAULT,
                    Theme::Dark => button_colors::DARK_DEFAULT,
                    Theme::Default => button_colors::DEF_DEFAULT,
                },
                ButtonType::Nav => match self.theme {
                    Theme::Light => button_colors::LIGHT_NAV,
                    Theme::Dark => button_colors::DARK_NAV,
                    Theme::Default => button_colors::DEF_NAV,
                },
            };

            button::Style {
                background: Some(Background::Color(bg_color)),
                border_radius: 15.0,
                text_color,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let (bg_color, text_color) = match self.button_type {
                ButtonType::Default => match self.theme {
                    Theme::Light => button_colors::LIGHT_DEFAULT_HOVERED,
                    Theme::Dark => button_colors::DARK_DEFAULT_HOVERED,
                    Theme::Default => button_colors::DEF_DEFAULT_HOVERED,
                },
                ButtonType::Nav => match self.theme {
                    Theme::Light => button_colors::LIGHT_NAV_HOVERED,
                    Theme::Dark => button_colors::DARK_NAV_HOVERED,
                    Theme::Default => button_colors::DEF_NAV_HOVERED,
                },
            };

            button::Style {
                background: Some(Background::Color(bg_color)),
                text_color,
                ..self.active()
            }
        }
    }

    pub struct Container {
        theme: Theme,
        container_type: ContainerType,
    }

    pub enum ContainerType {
        Default,
        Nav,
    }

    impl Container {
        pub fn new(theme: Theme, container_type: ContainerType) -> Self {
            Container { theme, container_type }
        }
    }

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            let (bg_color, text_color) = match self.container_type {
                ContainerType::Default => match self.theme {
                    Theme::Light => container_colors::LIGHT_DEFAULT,
                    Theme::Dark => container_colors::DARK_DEFAULT,
                    Theme::Default => container_colors::DEF_DEFAULT,
                },
                ContainerType::Nav => match self.theme {
                    Theme::Light => container_colors::LIGHT_NAV,
                    Theme::Dark => container_colors::DARK_NAV,
                    Theme::Default => container_colors::DEF_NAV,
                },
            };
            

            container::Style {
                background: Some(Background::Color(bg_color)),
                text_color: Some(text_color),
                ..container::Style::default()
            }
        }
    }
}
