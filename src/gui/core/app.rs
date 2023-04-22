//! Module defining the application structure: messages, updates, subscriptions.
use iced::widget::{TextInput, Button, Text, Row, Container};
use iced::{executor, Application, Command, Element, Theme, Length};
use serde_json::Value;
use zeroize::Zeroize;

use crate::gui::core::{
    message::Message,
};

use crate::gui::pages::details_page::{PageMode, EntryType};
use crate::gui::pages::{
    cards_page,
    details_page,
    identities_page,
    nav_page,
    passwords_page,
    profile_page,
};

use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::styles::types::style_type;
use crate::secure::encrypt::read_data;

// An enumeration of the different views in the application
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pages {
    CardsPage,
    IdentitiesPage,
    PasswordsPage,
    ProfilePage,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LoginState {
    LoggedOut,
    LoggingIn,
    LoggedIn,
}

pub struct KeyboltApp {
    pub login_state: LoginState,
    pub current_page: Pages,
    pub current_style: style_type::StyleType,
    pub password: String,
    pub entries: Value,
    pub selected_entry_id: i32,
    pub current_entry_mode: details_page::PageMode,
    pub current_entry_type: EntryType,
}

impl KeyboltApp {
    fn update_entry(&mut self, entry_type: EntryType, entry_id: i32, updated_entry: Value) {
        if let Some(entries) = self.entries.get_mut(entry_type.as_str()) {
            if let Some(entry) = entries.get_mut(entry_id as usize) {
                *entry = updated_entry;
            }
        }
    }
}

impl Application for KeyboltApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (KeyboltApp {
            login_state: LoginState::LoggedOut,
            current_page: Pages::PasswordsPage,
            current_style: style_type::StyleType::Default,
            password: String::new(),
            entries: Value::Null,
            selected_entry_id: -1,
            current_entry_mode: PageMode::Closed,
            current_entry_type: EntryType::Passwords,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Keybolt")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => self.current_page = page,
            Message::ChangeStyle(style) => self.current_style = style,
            Message::PasswordInputChanged(password) => self.password = password,
            Message::PasswordInputSubmit => {
                if self.login_state != LoginState::LoggingIn {
                    self.login_state = LoginState::LoggingIn;

                    match read_data(&self.password) {
                        Ok(data) => {
                            self.entries = data;
                            println!("Data: {:?}", self.entries);
                            self.password.zeroize();
                            self.login_state = LoginState::LoggedIn;
                        },
                        Err(e) => {
                            println!("Error reading data: {:?}", e);
                            self.password.zeroize();
                            self.login_state = LoginState::LoggedOut;
                        },
                    }
                }
            },
            Message::ChangeEntryMode(mode) => {
                self.current_entry_mode = mode;
            },
            Message::UpdateEntry(entry_type, entry_id, updated_entry) => {
                self.update_entry(entry_type, entry_id, updated_entry);
            },
            Message::SelectEntry(entry_id) => {
                self.selected_entry_id = entry_id;
                self.current_entry_mode = PageMode::View;
            },
            _ => ()
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Add nav and window view together, display()
        let combine_views = |view| {
            Row::new()
                .push(nav_page::view_page(self.current_style, self.current_page))
                .push(view)
                .push(details_page::view_page(self.current_style, self.current_entry_mode, self.current_entry_type, self.selected_entry_id, &self.entries))
                .into()
        };

        // Set appropriate window view based on the current_view value
        match (self.login_state, self.current_page) {
            // User not logged in
            (LoginState::LoggedOut | LoginState::LoggingIn, _) => {
                let input =
                    TextInput::new(
                        "Enter password...",
                        &self.password,
                    )
                    .padding(8)
                    .on_input(Message::PasswordInputChanged)
                    .on_submit(Message::PasswordInputSubmit)
                    .password();
                
                let submit_button =
                    Button::new(Text::new("Unlock"))
                        .padding(8)
                        .on_press(Message::PasswordInputSubmit);
            
                let content = Row::new()
                    .width(Length::Fixed(300.0))
                    .spacing(5)
                    .push(input)
                    .push(submit_button);
            
                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                        StyleTuple(self.current_style, ElementType::NavColumn),
                    )).into()
            },
            // User is logged in
            (_, Pages::ProfilePage) => combine_views(profile_page::view_page(self.current_style)),
            (_, Pages::PasswordsPage) => combine_views(passwords_page::view_page(self.current_style, &self.entries["passwords"], self.selected_entry_id)),
            (_, Pages::IdentitiesPage) => combine_views(identities_page::view_page(self.current_style, &self.entries["identities"], self.selected_entry_id)),
            (_, Pages::CardsPage) => combine_views(cards_page::view_page(self.current_style, &self.entries["cards"], self.selected_entry_id)),
        }
    }
} 