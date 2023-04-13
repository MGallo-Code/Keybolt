#![allow(unused_imports)]
use iced::Settings;
use iced::pure::widget::{Button, Column, Container, Row, Text};
use iced::pure::Sandbox;
use iced::Length::Units;

// Include each page
use profile_page::ProfilePage;
use passwords_page::PasswordsPage;
use identities_page::IdentitiesPage;
use cards_page::CardsPage;
mod profile_page;
mod passwords_page;
mod identities_page;
mod cards_page;

// Run application
fn main() -> Result<(), iced::Error> {
    // Start the application with default settings
    AppView::run(Settings::default())
}

// The main struct representing the application's state and its views
struct AppView {
    current_view: Views,
    current_theme: Theme,
    profile_page: ProfilePage,
    passwords_page: PasswordsPage,
    identities_page: IdentitiesPage,
    cards_page: CardsPage,
}

// An enumeration of the different views in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Views {
    MainPage,
    ProfilePage,
    PasswordsPage,
    IdentitiesPage,
    CardsPage,
}

// An enumeration of the messages the application can receive
#[derive(Debug, Clone, Copy)]
pub enum AppMsg {
    ChangePage(Views),
    ChangeTheme,
}

// An enumeration of the available themes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Custom,
}

impl Theme {
    // Get the next theme in the enumeration
    fn next(&self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Custom,
            Theme::Custom => Theme::Light,
        }
    }
}

// Implement the Sandbox trait for the AppView struct
impl Sandbox for AppView {
    type Message = AppMsg;

    // Initialize a new AppView struct with default values
    fn new() -> Self {
        AppView {
            current_view: Views::MainPage,
            current_theme: Theme::Light,
            profile_page: ProfilePage::new(),
            passwords_page: PasswordsPage::new(),
            identities_page: IdentitiesPage::new(),
            cards_page: CardsPage::new(),
        }
    }

    // Return the title of the application
    fn title(&self) -> String {
        String::from("Keybolt")
    }

    // Update the application's state based on received messages
    fn update(&mut self, message: Self::Message) {
        match message {
            AppMsg::ChangePage(view) => self.current_view = view,
            AppMsg::ChangeTheme => self.current_theme = self.current_theme.next(),
        }
    }

    // Define the application's user interface layout based on its state
    fn view(&self) -> iced::pure::Element<Self::Message> {
        let nav_btn = |label, view| {
            if self.current_view== view {
                Button::new(label)
                    .width(Units(200))
                    .padding(10)
                    .style(style::Button::new(self.current_theme))
                    .on_press(AppMsg::ChangePage(view))
            } else {
                Button::new(label)
                    .width(Units(200))
                    .padding(10)
                    .style(style::Button::new(self.current_theme))
                    .on_press(AppMsg::ChangePage(view))
            }
        };

        // Nav column
        let profile_page_btn = nav_btn("Profile", Views::ProfilePage);
        let main_page_btn = nav_btn("Main Page", Views::MainPage);
        let passwords_page_btn = nav_btn("Passwords", Views::PasswordsPage);
        let identities_page_btn = nav_btn("Identities", Views::IdentitiesPage);
        let cards_page_btn = nav_btn("Cards", Views::CardsPage);

        // Create nav container
        let nav = Container::new(
            Column::new()
                .push(profile_page_btn)
                .push(main_page_btn)
                .push(passwords_page_btn)
                .push(identities_page_btn)
                .push(cards_page_btn)
        ).style(style::Container::new(self.current_theme, style::ContainerType::Nav)).height(iced::Length::Fill);
        
        // Main page layout

        // This button should toggle dark and light mode!
        let toggle_mode_btn = Button::new("Toggle Mode")
            .on_press(AppMsg::ChangeTheme);

        let main_page_layout = Container::new(toggle_mode_btn).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();

        let window_view;
        // Set appropriate window view based on the current_view value
        match self.current_view {
            Views::MainPage => {
                window_view = main_page_layout
            },
            Views::ProfilePage => {
                window_view = self.profile_page.view()
            },
            Views::PasswordsPage => {
                window_view = self.passwords_page.view()
            },
            Views::IdentitiesPage => {
                window_view = self.identities_page.view()
            },
            Views::CardsPage => {
                window_view = self.cards_page.view()
            },
        }
        // Add nav and window view together, display()
        Container::new(Row::new().push(nav).push(window_view)).into()
    }
}

pub mod theme_colors {
    use iced::Color;

    // MAIN COLORS
    // (Background Color, Text Color)
    // light
    pub const LIGHT_PRIMARY_COL: (Color, Color) = (Color::WHITE, Color::BLACK);
    pub const LIGHT_SECONDARY_COL: (Color, Color) = (Color::BLACK, Color::WHITE);
    pub const LIGHT_TERNARY_COL: (Color, Color) = (Color::from_rgb(0.0, 0.5, 0.0), Color::WHITE);
    // dark
    pub const DARK_PRIMARY_COL: (Color, Color) = (Color::from_rgb(0.1, 0.1, 0.1), Color::WHITE);
    pub const DARK_SECONDARY_COL: (Color, Color) = (Color::WHITE, Color::BLACK);
    pub const DARK_TERNARY_COL: (Color, Color) = (Color::from_rgb(0.0, 0.5, 0.0), Color::WHITE);
    // custom
    pub const CUST_PRIMARY_COL: (Color, Color) = (Color::from_rgb(0.2, 0.2, 0.5), Color::WHITE);
    pub const CUST_SECONDARY_COL: (Color, Color) = (Color::WHITE, Color::BLACK);
    pub const CUST_TERNARY_COL: (Color, Color) = (Color::from_rgb(0.0, 0.5, 0.0), Color::WHITE);

    // CONTAINERS
    pub const LIGHT_DEFAULT_CONTAINER: (Color, Color) = LIGHT_PRIMARY_COL;
    pub const DARK_DEFAULT_CONTAINER: (Color, Color) = DARK_PRIMARY_COL;
    pub const CUST_DEFAULT_CONTAINER: (Color, Color) = CUST_PRIMARY_COL;
    // NAV
    pub const LIGHT_NAV_CONTAINER: (Color, Color) = LIGHT_PRIMARY_COL;
    pub const DARK_NAV_CONTAINER: (Color, Color) = DARK_PRIMARY_COL;
    pub const CUST_NAV_CONTAINER: (Color, Color) = CUST_PRIMARY_COL;
}


pub mod style {
    use iced::{button, Background, Color, container};
    use crate::Theme;
    use crate::theme_colors;

    pub struct Button {
        theme: Theme,
    }

    impl Button {
        pub fn new(theme: Theme) -> Self {
            Button { theme }
        }
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let (bg_color, text_color) = match self.theme {
                Theme::Light => (Color::from_rgb(0.0, 0.0, 0.0), Color::WHITE),
                Theme::Dark => (Color::from_rgb(0.5, 0.5, 0.5), Color::BLACK),
                Theme::Custom => (Color::from_rgb(0.0, 0.5, 0.0), Color::WHITE),
            };

            button::Style {
                background: Some(Background::Color(bg_color)),
                border_radius: 15.0,
                text_color,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let (bg_color, text_color) = match self.theme {
                Theme::Light => (Color::from_rgb(0.2, 0.2,0.2), Color::WHITE),
                Theme::Dark => (Color::from_rgb(0.7, 0.7, 0.7), Color::BLACK),
                Theme::Custom => (Color::from_rgb(0.2, 0.5, 0.2), Color::WHITE),
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
                    Theme::Light => theme_colors::LIGHT_DEFAULT_CONTAINER,
                    Theme::Dark => theme_colors::DARK_DEFAULT_CONTAINER,
                    Theme::Custom => theme_colors::CUST_DEFAULT_CONTAINER,
                },
                ContainerType::Nav => match self.theme {
                    Theme::Light => theme_colors::LIGHT_NAV_CONTAINER,
                    Theme::Dark => theme_colors::DARK_NAV_CONTAINER,
                    Theme::Custom => theme_colors::CUST_NAV_CONTAINER,
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
