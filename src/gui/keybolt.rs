use iced::{Command, Length, Application, Theme, executor};
use iced::widget::{
    Button,
    Column,
    Container,
};
use crate::gui::styles::types::{
    style_tuple::StyleTuple,
    style_type::StyleType,
};

// Page View Imports
use crate::gui::pages::{
    cards_page,
    identities_page,
    passwords_page,
    profile_page,
};

// The main struct representing the application's state and its views
pub struct KeyboltApp {
    pub current_page: Pages,
    pub current_theme: StyleType,
}

// An enumeration of the different views in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pages {
    CardsPage,
    IdentitiesPage,
    PasswordsPage,
    ProfilePage,
}

// An enumeration of the messages the application can receive
#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Pages),
    ChangeTheme(StyleType),
}

// Implement the Application trait for the AppView struct
impl Application for KeyboltApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    // Initialize a new AppView struct with default values
    fn new(_flags: ()) -> (KeyboltApp, Command<Message>) {
        (KeyboltApp {
            current_page: Pages::ProfilePage,
            current_theme: StyleType::Day,
        }, Command::none())
    }

    // Return the title of the application
    fn title(&self) -> String {
        String::from("Keybolt")
    }

    // Update the application's state based on received messages
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => self.current_page = page,
            Message::ChangeTheme(theme) => self.current_theme = theme,
        }
        Command::none()
    }

    // Define the application's user interface layout based on its state
    fn view(&self) -> iced::Element<Message> {
        let style = self.current_theme;

        // let nav_btn = |label, page| {
        //     Button::new(label)
        //         .width(Length::Fixed(300.0))
        //         .padding(10)
        //         .on_press(Message::ChangePage(page))
        //         // .style(Box::new(styling::CustomButtonStyle))
        // };

        // // Nav column
        // let profile_page_btn = nav_btn("Profile", Pages::ProfilePage);
        // let passwords_page_btn = nav_btn("Passwords", Pages::PasswordsPage);
        // let identities_page_btn = nav_btn("Identities", Pages::IdentitiesPage);
        // let cards_page_btn = nav_btn("Cards", Pages::CardsPage);

        // // Create nav container
        // let nav = Container::new(
        //     Column::new()
        //         .push(profile_page_btn)
        //         .push(passwords_page_btn)
        //         .push(identities_page_btn)
        //         .push(cards_page_btn)
        // ).height(iced::Length::Fill);
        

        let window_view;
        // Set appropriate window view based on the current_view value
        match self.current_page {
            Pages::ProfilePage => {
                window_view = profile_page::view_page(style)
            },
            Pages::PasswordsPage => {
                window_view = passwords_page::view_page(style)
            },
            Pages::IdentitiesPage => {
                window_view = identities_page::view_page(style)
            },
            Pages::CardsPage => {
                window_view = cards_page::view_page(style)
            },
        }
        window_view.into()
        // Add nav and window view together, display()
        // Container::new(
        //     Row::new()
        //         .push(nav)
        //         .push(window_view),
        // )
        // .style(
        //     <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
        //         self.current_theme,
        //         ElementType::Default,
        //     )),
        // )
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}