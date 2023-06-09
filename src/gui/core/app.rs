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
use crate::secure::encrypt::{read_data, encrypt_sensitive_fields, decrypt_sensitive_fields, write_data};

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
    pub passphrase: String,
    pub entries: Value,
    pub selected_entry_id: i32,
    pub current_entry_edits: Value,
    pub current_entry_mode: details_page::PageMode,
    pub current_entry_type: EntryType,
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
            passphrase: String::new(),
            entries: Value::Null,
            selected_entry_id: -1,
            current_entry_edits: Value::Null,
            current_entry_mode: PageMode::Closed,
            current_entry_type: EntryType::Passwords,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Keybolt")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => {
                self.current_page = page;
                self.current_entry_type = match page {
                    Pages::CardsPage => EntryType::Cards,
                    Pages::IdentitiesPage => EntryType::Identities,
                    _ => EntryType::Passwords,
                };
                self.selected_entry_id = -1;
                self.current_entry_mode = PageMode::Closed;
            },
            Message::ChangeStyle(style) => self.current_style = style,
            Message::PasswordInputChanged(passphrase) => self.passphrase = passphrase,
            Message::PasswordInputSubmit => {
                if self.login_state != LoginState::LoggingIn {
                    self.login_state = LoginState::LoggingIn;

                    match read_data(&self.passphrase) {
                        Ok(data) => {
                            self.entries = data;
                            // println!("Data: {:?}", self.entries);
                            self.login_state = LoginState::LoggedIn;
                        },
                        Err(e) => {
                            println!("Error reading data: {:?}", e);
                            self.login_state = LoginState::LoggedOut;
                        },
                    }
                }
            },
            Message::ChangeEntryMode(mode) => {
                if mode == PageMode::Edit {
                    decrypt_sensitive_fields(&self.passphrase, &mut self.current_entry_edits, self.current_entry_type);
                } else if self.current_entry_mode == PageMode::Edit {
                    encrypt_sensitive_fields(&self.passphrase, &mut self.current_entry_edits, self.current_entry_type);
                }
                self.current_entry_mode = mode;
            },
            Message::SaveEntryEdits => {
                self.current_entry_mode = PageMode::View;
                encrypt_sensitive_fields(&self.passphrase, &mut self.current_entry_edits, self.current_entry_type);
                self.entries[self.current_entry_type.as_str()][self.selected_entry_id as usize] = self.current_entry_edits.clone();
                //TODO UNCOMMENT TO SAVE CHANGES TO FILE
                // write_data(&self.passphrase, self.entries.clone());
            },
            Message::SelectEntry(entry_id) => {
                self.selected_entry_id = entry_id;
                self.current_entry_edits = self.entries[self.current_entry_type.as_str()][entry_id as usize].clone();
                println!("{:?}", entry_id);
                println!("{:?}", self.entries[self.current_entry_type.as_str()][entry_id as usize]);
                self.current_entry_mode = PageMode::View;
            },
            // Messages for updating password entries
            Message::UpdatePasswordTitle(input) => self.current_entry_edits["title"] = Value::String(input),
            Message::UpdatePasswordUrl(input) => self.current_entry_edits["url"] = Value::String(input),
            Message::UpdatePasswordUsername(input) => self.current_entry_edits["username"] = Value::String(input),
            Message::UpdatePasswordPassword(input) => self.current_entry_edits["password"] = Value::String(input),
            Message::UpdatePasswordOtpAuth(input) => self.current_entry_edits["otpauth"] = Value::String(input),
            Message::UpdatePasswordFavorite(input) => self.current_entry_edits["favorite"] = Value::Bool(input),
            Message::UpdatePasswordTags(input) => self.current_entry_edits["tags"] = Value::String(input),
            Message::UpdatePasswordNotes(input) => self.current_entry_edits["notes"] = Value::String(input),

            // Messages for updating identity entries
            Message::UpdateIdentityTitle(input) => self.current_entry_edits["title"] = Value::String(input),
            Message::UpdateIdentityFirstName(input) => self.current_entry_edits["first_name"] = Value::String(input),
            Message::UpdateIdentityMiddleInitial(input) => self.current_entry_edits["middle_initial"] = Value::String(input),
            Message::UpdateIdentityLastName(input) => self.current_entry_edits["last_name"] = Value::String(input),
            Message::UpdateIdentityAddress(input) => self.current_entry_edits["address"] = Value::String(input),
            Message::UpdateIdentityCity(input) => self.current_entry_edits["city"] = Value::String(input),
            Message::UpdateIdentityCountry(input) => self.current_entry_edits["country"] = Value::String(input),
            Message::UpdateIdentityState(input) => self.current_entry_edits["state"] = Value::String(input),
            Message::UpdateIdentityZipcode(input) => self.current_entry_edits["zipcode"] = Value::String(input),
            Message::UpdateIdentityPhone(input) => self.current_entry_edits["phone"] = Value::String(input),
            Message::UpdateIdentityEmail(input) => self.current_entry_edits["email"] = Value::String(input),
            Message::UpdateIdentityAptNumber(input) => self.current_entry_edits["apt_number"] = Value::String(input),

            // Messages for updating card entries
            Message::UpdateCardTitle(input) => self.current_entry_edits["title"] = Value::String(input),
            Message::UpdateCardName(input) => self.current_entry_edits["name"] = Value::String(input),
            Message::UpdateCardNumber(input) => self.current_entry_edits["card_number"] = Value::String(input),
            Message::UpdateCardLastFour(input) => self.current_entry_edits["card_last_four"] = Value::String(input),
            Message::UpdateCardExpirationDate(input) => self.current_entry_edits["expiration_date"] = Value::String(input),
            Message::UpdateCardSecurityCode(input) => self.current_entry_edits["security_code"] = Value::String(input),
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
                .push(details_page::view_page(self.current_style, self.current_entry_mode, self.current_entry_type, &self.current_entry_edits))
                .into()
        };

        // Set appropriate window view based on the current_view value
        match (self.login_state, self.current_page) {
            // User not logged in
            (LoginState::LoggedOut | LoginState::LoggingIn, _) => {
                let input =
                    TextInput::new(
                        "Enter password...",
                        &self.passphrase,
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