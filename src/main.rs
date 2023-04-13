#![allow(unused_imports)]
use iced::Settings;
use iced::pure::widget::{Button, Column, Container, Row, Text};
use iced::pure::Sandbox;
use iced::Length::Units;

// Style imports
use crate::themes::{ Theme, style };
pub mod themes;

// Page View Imports
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
    ChangeTheme(Theme),
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
            AppMsg::ChangeTheme(theme) => self.current_theme = theme,
        }
    }

    // Define the application's user interface layout based on its state
    fn view(&self) -> iced::pure::Element<Self::Message> {
        let nav_btn = |label, view| {
            if self.current_view == view {
                Button::new(label)
                    .width(Units(200))
                    .padding(10)
                    .style(style::Button::new(self.current_theme, style::ButtonType::Nav))
                    .on_press(AppMsg::ChangePage(view))
            } else {
                Button::new(label)
                    .width(Units(200))
                    .padding(10)
                    .style(style::Button::new(self.current_theme, style::ButtonType::Nav))
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

        let light_mode_btn = Button::new("Light Mode")
            .on_press(AppMsg::ChangeTheme(Theme::Light));
        let dark_mode_btn = Button::new("Dark Mode")
            .on_press(AppMsg::ChangeTheme(Theme::Dark));
        let default_mode_btn = Button::new("Default Mode")
            .on_press(AppMsg::ChangeTheme(Theme::Default));

        let main_page_layout = Container::new(
            Column::new()
                .push(light_mode_btn)
                .push(dark_mode_btn)
                .push(default_mode_btn)
        ).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();

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