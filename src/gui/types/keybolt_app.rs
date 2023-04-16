//! Module defining the `KeyboltApp` struct, which trace gui's component statuses and permits
//! to share data among the different threads.
use iced::Command;

use crate::gui::styles::types::style_type;
use crate::gui::types::message::Message;

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

impl KeyboltApp {
    pub fn new() -> Self {
        Self {
            current_page: Pages::ProfilePage,
            current_style: style_type::StyleType::Day,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => self.current_page = page,
            Message::ChangeStyle(style) => self.current_style = style,
        }
        Command::none()
    }
}
