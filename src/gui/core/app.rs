//! Module defining the application structure: messages, updates, subscriptions.
use iced::widget::{Container, Row, Button, Column};
use iced::{executor, Application, Command, Element, Theme, Length};

use crate::gui::core::{
    message::Message,
};

use crate::gui::pages::{
    cards_page,
    identities_page,
    passwords_page,
    profile_page,
};
use crate::gui::styles::types::{
    element_type::ElementType,
    style_tuple::StyleTuple,
};

use crate::gui::styles::types::style_type;

// An enumeration of the different views in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pages {
    CardsPage,
    IdentitiesPage,
    PasswordsPage,
    ProfilePage,
}

pub struct KeyboltApp {
    pub current_page: Pages,
    pub current_style: style_type::StyleType,
}

impl Application for KeyboltApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (KeyboltApp {
            current_page: Pages::ProfilePage,
            current_style: style_type::StyleType::Dark,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Keybolt")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => self.current_page = page,
            Message::ChangeStyle(style) => self.current_style = style,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let style = self.current_style;

        let nav_btn = |label, page| {
            if self.current_page == page {
                Button::new(label)
                    .width(Length::Fixed(300.0))
                    .padding(10)
                    .on_press(Message::ChangePage(page))
                    .style(<StyleTuple as Into<iced::theme::Button>>::into(
                        StyleTuple(style, ElementType::NavActive),
                    ))
            } else {
                Button::new(label)
                .width(Length::Fixed(300.0))
                .padding(10)
                .on_press(Message::ChangePage(page))
                .style(<StyleTuple as Into<iced::theme::Button>>::into(
                    StyleTuple(style, ElementType::NavInactive),
                ))
            }
        };

        // Nav column
        let profile_page_btn = nav_btn("Profile", Pages::ProfilePage);
        let passwords_page_btn = nav_btn("Passwords", Pages::PasswordsPage);
        let identities_page_btn = nav_btn("Identities", Pages::IdentitiesPage);
        let cards_page_btn = nav_btn("Cards", Pages::CardsPage);

        // Create nav container
        let nav = Container::new(
            Column::new()
                .push(profile_page_btn)
                .push(passwords_page_btn)
                .push(identities_page_btn)
                .push(cards_page_btn)
        ).height(iced::Length::Fill);

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
        // Add nav and window view together, display()
        Container::new(
            Row::new()
                .push(nav)
                .push(window_view),
        )
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Default),
        )).into()
    }
} 