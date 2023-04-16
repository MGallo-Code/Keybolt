use iced::{Application, Command, executor, Settings, Theme};
use iced::widget::{Button, Column, Container, Row};
use iced::Length;

// Page View Imports
mod gui;
use gui::pages::{
    cards_page::CardsPage,
    identities_page::IdentitiesPage,
    passwords_page::PasswordsPage,
    profile_page::ProfilePage,
};

// Run application
fn main() -> Result<(), iced::Error> {
    // Start the application with default settings
    KeyboltApp::run(Settings::default())
}

// The main struct representing the application's state and its views
struct KeyboltApp {
    current_page: Pages,
    current_theme: Theme,
    profile_page: ProfilePage,
    passwords_page: PasswordsPage,
    identities_page: IdentitiesPage,
    cards_page: CardsPage,
}

// An enumeration of the different views in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pages {
    MainPage,
    ProfilePage,
    PasswordsPage,
    IdentitiesPage,
    CardsPage,
}

// An enumeration of the messages the application can receive
#[derive(Debug, Clone)]
pub enum AppMsg {
    ChangePage(Pages),
    ChangeTheme(Theme),
}

// Implement the Application trait for the AppView struct
impl Application for KeyboltApp {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = Theme;
    type Flags = ();

    // Initialize a new AppView struct with default values
    fn new(_flags: ()) -> (KeyboltApp, Command<Self::Message>) {
        (KeyboltApp {
            current_page: Pages::MainPage,
            current_theme: Theme::Light,
            profile_page: ProfilePage::new(),
            passwords_page: PasswordsPage::new(),
            identities_page: IdentitiesPage::new(),
            cards_page: CardsPage::new(),
        }, Command::none())
    }

    // Return the title of the application
    fn title(&self) -> String {
        String::from("Keybolt")
    }

    // Update the application's state based on received messages
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMsg::ChangePage(page) => self.current_page = page,
            AppMsg::ChangeTheme(theme) => self.current_theme = theme,
        }
        Command::none()
    }

    // Define the application's user interface layout based on its state
    fn view(&self) -> iced::Element<Self::Message> {
        let nav_btn = |label, page| {
            Button::new(label)
            .width(Length::Fixed(300.0))
            .padding(10)
            .on_press(AppMsg::ChangePage(page))
            // .style(Box::new(styling::CustomButtonStyle))
        };

        // Nav column
        let profile_page_btn = nav_btn("Profile", Pages::ProfilePage);
        let main_page_btn = nav_btn("Main Page", Pages::MainPage);
        let passwords_page_btn = nav_btn("Passwords", Pages::PasswordsPage);
        let identities_page_btn = nav_btn("Identities", Pages::IdentitiesPage);
        let cards_page_btn = nav_btn("Cards", Pages::CardsPage);

        // Create nav container
        let nav = Container::new(
            Column::new()
                .push(profile_page_btn)
                .push(main_page_btn)
                .push(passwords_page_btn)
                .push(identities_page_btn)
                .push(cards_page_btn)
        ).height(iced::Length::Fill);
        
        // Main page layout

        let light_mode_btn = Button::new("Light Mode")
            .width(Length::Fixed(300.0))
            .padding(10)
            .on_press(AppMsg::ChangeTheme(Theme::Light));
        let dark_mode_btn = Button::new("Dark Mode")
            .width(Length::Fixed(300.0))
            .padding(10)
            .on_press(AppMsg::ChangeTheme(Theme::Dark));

        let main_page_layout = Container::new(
            Column::new()
                .push(light_mode_btn)
                .push(dark_mode_btn)
        ).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into();

        let window_view;
        // Set appropriate window view based on the current_view value
        match self.current_page {
            Pages::MainPage => {
                window_view = main_page_layout
            },
            Pages::ProfilePage => {
                window_view = self.profile_page.view()
            },
            Pages::PasswordsPage => {
                window_view = self.passwords_page.view()
            },
            Pages::IdentitiesPage => {
                window_view = self.identities_page.view()
            },
            Pages::CardsPage => {
                window_view = self.cards_page.view()
            },
        }
        // Add nav and window view together, display()
        Container::new(Row::new().push(nav).push(window_view)).into()
    }

    fn theme(&self) -> Self::Theme {
        match self.current_theme {
            Theme::Light => Theme::Light,
            Theme::Dark => Theme::Dark,
            _ => Theme::Light,
        }
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}