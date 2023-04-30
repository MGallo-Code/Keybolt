//! Module defining the application structure: messages, updates, subscriptions.
use iced::widget::{TextInput, Button, Text, Row, Container};
use iced::{executor, Application, Command, Element, Length, Renderer};
use serde_json::Value;

use crate::gui::core::{
    message::Message,
};

use crate::gui::pages::details_page::{PageMode, EntryType};
use crate::gui::pages::{
    details_page,
    nav_page,
    profile_page, entries_page,
};

use crate::gui::styles::elements::button::ButtonStyle;
use crate::gui::styles::elements::text_input::TextInputStyle;
use crate::gui::styles::font_constants::RALEWAY_BOLD;
use crate::gui::styles::keybolt_theme::KeyboltTheme;
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
    pub current_theme: KeyboltTheme,
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
    type Theme = KeyboltTheme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (KeyboltApp {
            login_state: LoginState::LoggedOut,
            current_page: Pages::PasswordsPage,
            current_theme: KeyboltTheme::Light,
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
            Message::ChangeStyle(theme) => self.current_theme = theme,
            Message::PasswordInputChanged(passphrase) => self.passphrase = passphrase,
            Message::PasswordInputSubmit => {
                if self.login_state != LoginState::LoggingIn {
                    self.login_state = LoginState::LoggingIn;

                    match read_data(&self.passphrase) {
                        Ok(data) => {
                            self.entries = data;
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
                    decrypt_sensitive_fields(&self.passphrase, &mut self.current_entry_edits, self.current_entry_type).unwrap();
                } else if self.current_entry_mode == PageMode::Edit {
                    encrypt_sensitive_fields(&self.passphrase, &mut self.current_entry_edits, self.current_entry_type).unwrap();
                }
                // Reset selected item if details window closed
                if mode == PageMode::Closed {
                    self.selected_entry_id = -1;
                    self.current_entry_edits = Value::Null;
                }
                self.current_entry_mode = mode;
            },
            Message::SaveEntryEdits => {
                self.current_entry_mode = PageMode::View;
                encrypt_sensitive_fields(&self.passphrase, &mut self.current_entry_edits, self.current_entry_type).unwrap();
                self.entries[self.current_entry_type.as_str()][self.selected_entry_id as usize] = self.current_entry_edits.clone();
                write_data(&self.passphrase, self.entries.clone()).unwrap();
            },
            Message::SelectEntry(entry_id) => {
                self.selected_entry_id = entry_id;
                self.current_entry_edits = self.entries[self.current_entry_type.as_str()][entry_id as usize].clone();
                self.current_entry_mode = PageMode::View;
            },
            // Messages for updating password entries
            Message::UpdatePasswordTitle(input) => self.current_entry_edits["title"] = Value::String(input),
            Message::UpdatePasswordUrl(input) => self.current_entry_edits["url"] = Value::String(input),
            Message::UpdatePasswordUsername(input) => self.current_entry_edits["username"] = Value::String(input),
            Message::UpdatePasswordPassword(input) => self.current_entry_edits["password"] = Value::String(input),
            Message::UpdatePasswordOtpAuth(input) => self.current_entry_edits["otpauth"] = Value::String(input),
            Message::UpdatePasswordFavorite(input) | Message::UpdateCardFavorite(input) | Message::UpdateIdentityFavorite(input) => {
                self.current_entry_edits["favorite"] = Value::Bool(input);
                self.entries[self.current_entry_type.as_str()][self.selected_entry_id as usize]["favorite"] = Value::Bool(input);
                write_data(&self.passphrase, self.entries.clone()).unwrap()
            },
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
            Message::UpdateIdentityBirthMonth(input) => self.current_entry_edits["birth_month"] = Value::String(input),
            Message::UpdateIdentityBirthDay(input) => self.current_entry_edits["birth_day"] = Value::String(input),
            Message::UpdateIdentityBirthYear(input) => self.current_entry_edits["birth_year"] = Value::String(input),
            Message::UpdateIdentityWebsite(input) => self.current_entry_edits["website"] = Value::String(input),

            // Messages for updating card entries
            Message::UpdateCardTitle(input) => self.current_entry_edits["title"] = Value::String(input),
            Message::UpdateCardFirstName(input) => self.current_entry_edits["first_name"] = Value::String(input),
            Message::UpdateCardMiddleInitial(input) => self.current_entry_edits["middle_initial"] = Value::String(input),
            Message::UpdateCardLastName(input) => self.current_entry_edits["last_name"] = Value::String(input),
            Message::UpdateCardNumber(input) => {
                self.current_entry_edits["card_number"] = Value::String(input.clone());
                self.current_entry_edits["card_last_four"] = Value::String(String::from(&input.clone()[input.len() - 4..]));
            },
            Message::UpdateCardExpirationMonth(input) => self.current_entry_edits["expiration_month"] = Value::String(input),
            Message::UpdateCardExpirationYear(input) => self.current_entry_edits["expiration_year"] = Value::String(input),
            Message::UpdateCardSecurityCode(input) => self.current_entry_edits["security_code"] = Value::String(input),
            Message::UpdateCardAddress(input) => self.current_entry_edits["address"] = Value::String(input),
            Message::UpdateCardCity(input) => self.current_entry_edits["city"] = Value::String(input),
            Message::UpdateCardCountry(input) => self.current_entry_edits["country"] = Value::String(input),
            Message::UpdateCardState(input) => self.current_entry_edits["state"] = Value::String(input),
            Message::UpdateCardZipcode(input) => self.current_entry_edits["zipcode"] = Value::String(input),
            Message::UpdateCardAptNumber(input) => self.current_entry_edits["apt_number"] = Value::String(input), 
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer<KeyboltTheme>> {
        // Add nav and window view together, display()
        let combine_views = |view| -> Element<'static, Message, Renderer<KeyboltTheme>> {
            Row::new()
                .push(nav_page::view_page(self.current_page))
                .push(view)
                .push(details_page::view_page(self.current_entry_mode, self.current_entry_type, &self.current_entry_edits))
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
                    .password()
                    .style(TextInputStyle::Primary);

                let submit_button =
                    Button::new(
                        Text::new("Unlock")
                            .font(RALEWAY_BOLD)
                        )
                        .padding(8)
                        .on_press(Message::PasswordInputSubmit)
                        .style(ButtonStyle::Secondary);

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
                    .into()
            },
            // User is logged in
            (_, Pages::ProfilePage) => combine_views(profile_page::view_page()),
            (_, Pages::PasswordsPage) => combine_views(entries_page::view_page(&self.entries["passwords"], EntryType::Passwords, self.selected_entry_id)),
            (_, Pages::IdentitiesPage) => combine_views(entries_page::view_page(&self.entries["identities"], EntryType::Identities, self.selected_entry_id)),
            (_, Pages::CardsPage) => combine_views(entries_page::view_page(&self.entries["cards"], EntryType::Cards, self.selected_entry_id)),
        }
    }

    fn theme(&self) -> Self::Theme {
        self.current_theme
    }
} 