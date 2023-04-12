#![allow(unused_imports)]
use iced::Settings;
use iced::pure::widget::{Button, Column, Container, Row, Text};
use iced::pure::Sandbox;

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
    profile_page: ProfilePage,
    passwords_page: PasswordsPage,
    identities_page: IdentitiesPage,
    cards_page: CardsPage,
}

// An enumeration of the different views in the application
#[derive(Debug, Clone, Copy)]
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
}

// Implement the Sandbox trait for the AppView struct
impl Sandbox for AppView {
    type Message = AppMsg;

    // Initialize a new AppView struct with default values
    fn new() -> Self {
        AppView {
            current_view: Views::MainPage,
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
        }
    }

    // Define the application's user interface layout based on its state
    fn view(&self) -> iced::pure::Element<Self::Message> {
        // Nav column
        let profile_btn = Button::new("Profile").on_press(AppMsg::ChangePage(Views::ProfilePage));
        let main_page_btn = Button::new("Main Page").on_press(AppMsg::ChangePage(Views::MainPage));
        let pwds_page_btn = Button::new("Passwords").on_press(AppMsg::ChangePage(Views::PasswordsPage));
        let identities_page_btn = Button::new("Identities").on_press(AppMsg::ChangePage(Views::IdentitiesPage));
        let cards_page_btn = Button::new("Cards").on_press(AppMsg::ChangePage(Views::CardsPage));
        let nav_col = Column::new()
            .push(profile_btn)
            .push(main_page_btn)
            .push(pwds_page_btn)
            .push(identities_page_btn)
            .push(cards_page_btn);
        
        // Main page layout
        let label = Text::new(format!("View: Main"));
        let main_page_layout = Container::new(label).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();

        let window_view;
        // Set appropriate window view based on the current_view value
        match self.current_view {
            Views::MainPage => window_view = main_page_layout,
            Views::ProfilePage => window_view = self.profile_page.view(),
            Views::PasswordsPage => window_view = self.passwords_page.view(),
            Views::IdentitiesPage => window_view = self.identities_page.view(),
            Views::CardsPage => window_view = self.cards_page.view(),
        }
        // Add nav and window view together, display()
        Container::new(Row::new().push(nav_col).push(window_view)).into()
    }
}
