use iced::{Length, Renderer, Element, Padding};
use iced::widget::{Column, Container, Text, Button, Space, Row, Scrollable, TextInput, Toggler};
use serde_json::Value;

use crate::gui::styles::elements::button::ButtonStyle;
use crate::gui::styles::elements::container::ContainerStyle;
use crate::gui::styles::elements::text::TextStyle;
use crate::gui::styles::elements::text_input::TextInputStyle;
use crate::gui::styles::keybolt_theme::KeyboltTheme;
use crate::gui::core::message::Message;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PageMode {
    View,
    Edit,
    Closed,
}

#[derive(Clone, Copy, Debug)]
pub enum EntryType {
    Passwords,
    Identities,
    Cards,
}

impl EntryType {
    pub fn as_str(&self) -> &str {
        match self {
            EntryType::Passwords => "passwords",
            EntryType::Identities => "identities",
            EntryType::Cards => "cards",
        }
    }
}

// Define the user interface layout for the ProfilePage
pub fn view_page(current_page_mode: PageMode, entry_type: EntryType, entry_data_edits: &Value) -> Element<'static, Message, Renderer<KeyboltTheme>> {
    match current_page_mode {
        PageMode::Closed => {
            Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
        },
        _ => {
            // HEADER
            let close_btn = Button::new("Close")
                .on_press(Message::ChangeEntryMode(PageMode::Closed))
                .style(ButtonStyle::Secondary);

            let edit_toggle_btn_str = if current_page_mode == PageMode::Edit { "Save" } else { "Edit" };
            let mut edit_toggle_btn = Button::new(edit_toggle_btn_str)
                .style(ButtonStyle::Secondary);
            if current_page_mode == PageMode::Edit {
                edit_toggle_btn = edit_toggle_btn
                    .on_press(Message::SaveEntryEdits)
            } else {
                edit_toggle_btn = edit_toggle_btn.on_press(Message::ChangeEntryMode(PageMode::Edit));
            }

            let left_btn_container = Column::new()
                .push(close_btn)
                .width(Length::Fill)
                .align_items(iced::Alignment::Start);

            let right_btn_container = Column::new()
                .push(edit_toggle_btn)
                .width(Length::Fill)
                .align_items(iced::Alignment::End);

            // HEADER
            let header_row = Row::new()
                .push(left_btn_container)
                .push(right_btn_container);

            // BODY
            let details_body =
                match current_page_mode {
                    PageMode::View => {
                        let get_entry_str = |entry_name: &str| -> String {
                            match entry_data_edits[entry_name] {
                                Value::String(ref s) => s.to_string(),
                                _ => "".to_string(),
                            }
                        };
                        match entry_type {
                            EntryType::Passwords => {
                                let title = 
                                    Column::new()
                                    .push(
                                        Text::new(get_entry_str("title"))
                                            .size(35)
                                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                                            .width(iced::Length::Fill)
                                            .style(TextStyle::DetailsTitle)
                                    );

                                let favorite_value = Toggler::new(String::from("Favorite"), entry_data_edits["favorite"] == Value::Bool(true), Message::UpdatePasswordFavorite);

                                let login_info = Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Username: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("Username", &entry_data_edits["username"].as_str().unwrap_or(""))
                                                .padding(8)
                                        )
                                        .push(
                                            Text::new("Password: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)

                                        )
                                        .push(
                                            TextInput::new("Password", &entry_data_edits["password"].as_str().unwrap_or(""))
                                                .padding(8)
                                                .password()
                                        )
                                    )
                                    .padding(8)
                                    .style(ContainerStyle::EntryInputContainer);

                                let mut otpauth_col = Column::new();
                                let otp_auth_exists = entry_data_edits["otpauth"].to_string().is_empty();
                                if otp_auth_exists {
                                    otpauth_col = otpauth_col.push(
                                        Text::new("OTP Authentication: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle));
                                    otpauth_col = otpauth_col.push(
                                        TextInput::new("otpauth", &entry_data_edits["otpauth"].as_str().unwrap_or(""))
                                                .padding(8));
                                };
                                let mut otpauth_container = Container::new(otpauth_col).style(ContainerStyle::EntryInputContainer);
                                if otp_auth_exists {
                                    otpauth_container = otpauth_container.padding(8);
                                }

                                let website_info = Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Site URL: ")
                                                .size(16)
                                                .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("https://www.example.com", &entry_data_edits["url"].as_str().unwrap_or(""))
                                                .padding(8)
                                        )
                                    )
                                    .padding(8)
                                    .style(ContainerStyle::EntryInputContainer);

                                let extra_info = Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Tags: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("Tags here...", &entry_data_edits["tags"].as_str().unwrap_or(""))
                                                .padding(8)
                                        )
                                        .push(
                                            Text::new("Notes: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("Notes here...", &entry_data_edits["notes"].as_str().unwrap_or(""))
                                                .padding(8)
                                        )
                                    )
                                    .padding(8)
                                    .style(ContainerStyle::EntryInputContainer);

                                let content = Column::new()
                                    .spacing(10)
                                    .push(title)
                                    .push(favorite_value)
                                    .push(login_info)
                                    .push(otpauth_container)
                                    .push(website_info)
                                    .push(extra_info)
                                    .padding(Padding::new(15.0))
                                    .width(iced::Length::Fill);

                                Scrollable::new(content)
                            },
                            EntryType::Identities => {
                                let title = 
                                    Column::new()
                                    .push(
                                        Text::new(get_entry_str("title"))
                                            .size(35)
                                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                                            .width(iced::Length::Fill)
                                            .style(TextStyle::DetailsTitle)
                                    );

                                let name_container = Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Full Name: ")
                                                .size(16)
                                                .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            Row::new()
                                                .push(
                                                    TextInput::new("First Name", &entry_data_edits["first_name"].as_str().unwrap_or(""))
                                                        .width(Length::Fill)
                                                )
                                                .push(
                                                    TextInput::new("M.I.", &entry_data_edits["middle_initial"].as_str().unwrap_or(""))
                                                        .width(Length::Shrink)
                                                )
                                                .push(
                                                    TextInput::new("Last Name", &entry_data_edits["last_name"].as_str().unwrap_or(""))
                                                        .width(Length::Fill)
                                                )
                                                .padding(8)
                                            )
                                    )
                                    .padding(8)
                                    .style(ContainerStyle::EntryInputContainer);

                                let address_container = Container::new(Column::new()
                                    .push(Text::new("Address: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("address")).size(16))
                                    .push(Text::new("City: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("city")).size(16))
                                    .push(Text::new("Country: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("country")).size(16))
                                    .push(Text::new("State: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("state")).size(16))
                                    .push(Text::new("Zipcode: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("zipcode")).size(16))
                                    .push(Text::new("Apt Number: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("apt_number")).size(16))
                            ).style(ContainerStyle::EntryInputContainer)
                                    .padding(8);

                                let contact_container = Container::new(Column::new()
                                    .push(Text::new("Phone: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("phone")).size(16))
                                    .push(Text::new("Email: ").size(16).style(TextStyle::EntryInputTitle))
                                    .push(Text::new(get_entry_str("email")).size(16))
                                ).style(ContainerStyle::EntryInputContainer)
                                    .padding(8);

                                let content = Column::new()
                                    .spacing(10)
                                    .push(title)
                                    .push(name_container)
                                    .push(address_container)
                                    .push(contact_container)
                                    .width(iced::Length::Fill);

                                Scrollable::new(content)
                            },
                            EntryType::Cards => {
                                let title_label = Text::new("Title: ").size(16);
                                let title_value = Text::new(get_entry_str("title")).size(16);

                                let card_number_label = Text::new("Card Number: ").size(16);
                                let card_number_value = Text::new(get_entry_str("card_number")).size(16);

                                let cardholder_name_label = Text::new("Cardholder Name: ").size(16);
                                let cardholder_name_value = Text::new(get_entry_str("cardholder_name")).size(16);

                                let expiration_date_label = Text::new("Expiration Date: ").size(16);
                                let expiration_date_value = Text::new(get_entry_str("expiration_date")).size(16);

                                let security_code_label = Text::new("Security Code: ").size(16);
                                let security_code_value = Text::new(get_entry_str("security_code")).size(16);

                                let content = Column::new()
                                    .spacing(10)
                                    .push(title_label)
                                    .push(title_value)
                                    .push(card_number_label)
                                    .push(card_number_value)
                                    .push(cardholder_name_label)
                                    .push(cardholder_name_value)
                                    .push(expiration_date_label)
                                    .push(expiration_date_value)
                                    .push(security_code_label)
                                    .push(security_code_value)
                                    .width(iced::Length::Fill);

                                Scrollable::new(content)
                            }
                        }
                    },
                    PageMode::Edit | _ => {
                        match entry_type {
                            EntryType::Passwords => {
                                let title = 
                                    Column::new()
                                    .push(
                                    TextInput::new("Title", &entry_data_edits["title"].as_str().unwrap_or(""))
                                        .padding(8)
                                        .size(35)
                                        .on_input(Message::UpdatePasswordTitle)
                                    );

                                let favorite_value = Toggler::new(String::from("Favorite"), entry_data_edits["favorite"] == Value::Bool(true), Message::UpdatePasswordFavorite);

                                let login_info = Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Username: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("Username", &entry_data_edits["username"].as_str().unwrap_or(""))
                                                .padding(8)
                                                .on_input(Message::UpdatePasswordUsername)
                                        )
                                        .push(
                                            Text::new("Password: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)

                                        )
                                        .push(
                                            TextInput::new("Password", &entry_data_edits["password"].as_str().unwrap_or(""))
                                                .padding(8)
                                                .on_input(Message::UpdatePasswordPassword)
                                        )
                                    )
                                    .padding(8)
                                    .style(ContainerStyle::EntryInputContainer);

                                let mut otpauth_col = Column::new();
                                let otp_auth_exists = entry_data_edits["otpauth"].to_string().is_empty();
                                if otp_auth_exists {
                                    otpauth_col = otpauth_col.push(
                                        Text::new("OTP Authentication: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle));
                                    otpauth_col = otpauth_col.push(
                                        TextInput::new("otpauth", &entry_data_edits["otpauth"].as_str().unwrap_or(""))
                                                .padding(8));
                                };
                                let mut otpauth_container = Container::new(otpauth_col).style(ContainerStyle::EntryInputContainer);
                                if otp_auth_exists {
                                    otpauth_container = otpauth_container.padding(8);
                                }

                                let website_info = Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Site URL: ")
                                                .size(16)
                                                .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("https://www.example.com", &entry_data_edits["url"].as_str().unwrap_or(""))
                                                .padding(8)
                                                .on_input(Message::UpdatePasswordUrl)
                                        )
                                    )
                                    .padding(8)
                                    .style(ContainerStyle::EntryInputContainer);

                                let extra_info = Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Tags: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("Tags here...", &entry_data_edits["tags"].as_str().unwrap_or(""))
                                                .padding(8)
                                                .on_input(Message::UpdatePasswordTags)
                                        )
                                        .push(
                                            Text::new("Notes: ")
                                            .size(16)
                                            .style(TextStyle::EntryInputTitle)
                                        )
                                        .push(
                                            TextInput::new("Notes here...", &entry_data_edits["notes"].as_str().unwrap_or(""))
                                                .padding(8)
                                                .on_input(Message::UpdatePasswordNotes)
                                        )
                                    )
                                    .padding(8)
                                    .style(ContainerStyle::EntryInputContainer);

                                let content = Column::new()
                                    .spacing(10)
                                    .push(title)
                                    .push(favorite_value)
                                    .push(login_info)
                                    .push(otpauth_container)
                                    .push(website_info)
                                    .push(extra_info)
                                    .padding(Padding::new(15.0))
                                    .width(iced::Length::Fill);

                                Scrollable::new(content)
                            },
                            EntryType::Identities => {
                                let title = TextInput::new("Title", &entry_data_edits["title"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityTitle);
                        
                                let first_name = TextInput::new("First Name", &entry_data_edits["first_name"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityFirstName);
                        
                                let middle_initial = TextInput::new("Middle Initial", &entry_data_edits["middle_initial"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityMiddleInitial);
                        
                                let last_name = TextInput::new("Last Name", &entry_data_edits["last_name"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityLastName);
                        
                                let address = TextInput::new("Address", &entry_data_edits["address"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityAddress);
                        
                                let city = TextInput::new("City", &entry_data_edits["city"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityCity);
                        
                                let country = TextInput::new("Country", &entry_data_edits["country"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityCountry);
                        
                                let state = TextInput::new("State", &entry_data_edits["state"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityState);
                        
                                let zipcode = TextInput::new("Zipcode", &entry_data_edits["zipcode"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityZipcode);
                        
                                let phone = TextInput::new("Phone", &entry_data_edits["phone"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityPhone);
                        
                                let email = TextInput::new("Email", &entry_data_edits["email"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityEmail);
                        
                                let apt_number = TextInput::new("Apt Number", &entry_data_edits["apt_number"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateIdentityAptNumber);
                        
                                Scrollable::new(
                                    Column::new()
                                        .push(title)
                                        .push(first_name)
                                        .push(middle_initial)
                                        .push(last_name)
                                        .push(address)
                                        .push(city)
                                        .push(country)
                                        .push(state)
                                        .push(zipcode)
                                        .push(phone)
                                        .push(email)
                                        .push(apt_number)
                                        .width(iced::Length::Fill)
                                    )
                            },
                            EntryType::Cards => {
                                let title = TextInput::new("Title", &entry_data_edits["title"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateCardTitle);
                        
                                let name = TextInput::new("Name", &entry_data_edits["name"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateCardName);
                        
                                let card_number = TextInput::new("Card Number", &entry_data_edits["card_number"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateCardNumber);
                        
                                let card_last_four = TextInput::new("Card Last Four", &entry_data_edits["card_last_four"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateCardLastFour);
                        
                                let expiration_date = TextInput::new("Expiration Date", &entry_data_edits["expiration_date"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateCardExpirationDate);
                        
                                let security_code = TextInput::new("Security Code", &entry_data_edits["security_code"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdateCardSecurityCode);
                        
                                Scrollable::new(
                                    Column::new()
                                        .push(title)
                                        .push(name)
                                        .push(card_number)
                                        .push(card_last_four)
                                        .push(expiration_date)
                                        .push(security_code)
                                        .width(iced::Length::Fill)
                                )
                            }
                        }
                    }
                };

            // Create nav container
            Container::new(
                Column::new()
                    .push(header_row)
                    .push(details_body)
                )
                .width(iced::Length::Fixed(300.0))
                .height(iced::Length::Fill)
                .into()
        },
    }
}