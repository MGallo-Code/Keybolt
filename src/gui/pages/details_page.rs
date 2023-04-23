use iced::{widget::{Container, Column, Text, Space, Button, Row, Scrollable, TextInput}, Element, Length};
use serde_json::Value;

use crate::gui::{styles::types::{element_type::ElementType, style_tuple::StyleTuple, style_type::StyleType}, core::{message::Message}};

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
pub fn view_page(style: StyleType, current_page_mode: PageMode, entry_type: EntryType, entry_data_edits: &Value) -> Element<'static, Message> {
    match current_page_mode {
        PageMode::Closed => {
            Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
        },
        _ => {
            // Nav column
            let keybolt_title = Container::new(
                Text::new("Details Window")
                )
                .padding(15)
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::NavHeader),
                ));
            
            // HEADER
            let close_btn = Button::new("Close")
                .on_press(Message::ChangeEntryMode(PageMode::Closed));

            let edit_toggle_btn_str = if current_page_mode == PageMode::Edit { "Save" } else { "Edit" };
            let mut edit_toggle_btn = Button::new(edit_toggle_btn_str);
            if current_page_mode == PageMode::Edit {
                edit_toggle_btn = edit_toggle_btn
                    .on_press(Message::SaveEntryEdits)
            } else {
                edit_toggle_btn = edit_toggle_btn.on_press(Message::ChangeEntryMode(PageMode::Edit));
            }

            

            // HEADER
            let header_row = Row::new()
                .push(close_btn)
                .push(keybolt_title)
                .push(edit_toggle_btn);

            // BODY
            let details_body =
                match current_page_mode {
                    PageMode::View => {
                        match entry_type {
                            EntryType::Passwords => {
                                let title_label = Text::new("Title: ").size(16);
                                let title_value = Text::new(entry_data_edits["title"].to_string()).size(16);

                                let url_label = Text::new("URL: ").size(16);
                                let url_value = Text::new(entry_data_edits["url"].to_string()).size(16);

                                let username_label = Text::new("Username: ").size(16);
                                let username_value = Text::new(entry_data_edits["username"].to_string()).size(16);

                                let password_label = Text::new("Password: ").size(16);
                                let password_value = Text::new(entry_data_edits["password"].to_string()).size(16);

                                let otpauth_label = Text::new("OTP Auth: ").size(16);
                                let otpauth_value = Text::new(entry_data_edits["otpauth"].to_string()).size(16);

                                // let favorite_label = Text::new("Favorite: ").size(16);
                                // let favorite_value = Text::new(if entry_data_edits.favorite { "Yes" } else { "No" }).size(16);

                                let tags_label = Text::new("Tags: ").size(16);
                                let tags_value = Text::new(entry_data_edits["tags"].to_string());

                                let notes_label = Text::new("Notes: ").size(16);
                                let notes_value = Text::new(entry_data_edits["notes"].to_string()).size(16);

                                let content = Column::new()
                                    .spacing(10)
                                    .push(title_label)
                                    .push(title_value)
                                    .push(url_label)
                                    .push(url_value)
                                    .push(username_label)
                                    .push(username_value)
                                    .push(password_label)
                                    .push(password_value)
                                    .push(otpauth_label)
                                    .push(otpauth_value)
                                    // .push(favorite_label)
                                    // .push(favorite_value)
                                    .push(tags_label)
                                    .push(tags_value)
                                    .push(notes_label)
                                    .push(notes_value)
                                    .width(iced::Length::Fill);

                                Scrollable::new(content)
                            },
                            EntryType::Identities => {
                                let title_label = Text::new("Title: ").size(16);
                                let title_value = Text::new(entry_data_edits["title"].to_string()).size(16);

                                let first_name_label = Text::new("First Name: ").size(16);
                                let first_name_value = Text::new(entry_data_edits["first_name"].to_string()).size(16);

                                let middle_initial_label = Text::new("Middle Initial: ").size(16);
                                let middle_initial_value = Text::new(entry_data_edits["middle_initial"].to_string()).size(16);

                                let last_name_label = Text::new("Last Name: ").size(16);
                                let last_name_value = Text::new(entry_data_edits["last_name"].to_string()).size(16);

                                let address_label = Text::new("Address: ").size(16);
                                let address_value = Text::new(entry_data_edits["address"].to_string()).size(16);

                                let city_label = Text::new("City: ").size(16);
                                let city_value = Text::new(entry_data_edits["city"].to_string()).size(16);

                                let country_label = Text::new("Country: ").size(16);
                                let country_value = Text::new(entry_data_edits["country"].to_string()).size(16);

                                let state_label = Text::new("State: ").size(16);
                                let state_value = Text::new(entry_data_edits["state"].to_string()).size(16);

                                let zipcode_label = Text::new("Zipcode: ").size(16);
                                let zipcode_value = Text::new(entry_data_edits["zipcode"].to_string()).size(16);

                                let phone_label = Text::new("Phone: ").size(16);
                                let phone_value = Text::new(entry_data_edits["phone"].to_string()).size(16);

                                let email_label = Text::new("Email: ").size(16);
                                let email_value = Text::new(entry_data_edits["email"].to_string()).size(16);

                                let apt_number_label = Text::new("Apt Number: ").size(16);
                                let apt_number_value = Text::new(entry_data_edits["apt_number"].to_string()).size(16);

                                let content = Column::new()
                                    .spacing(10)
                                    .push(title_label)
                                    .push(title_value)
                                    .push(first_name_label)
                                    .push(first_name_value)
                                    .push(middle_initial_label)
                                    .push(middle_initial_value)
                                    .push(last_name_label)
                                    .push(last_name_value)
                                    .push(address_label)
                                    .push(address_value)
                                    .push(city_label)
                                    .push(city_value)
                                    .push(country_label)
                                    .push(country_value)
                                    .push(state_label)
                                    .push(state_value)
                                    .push(zipcode_label)
                                    .push(zipcode_value)
                                    .push(phone_label)
                                    .push(phone_value)
                                    .push(email_label)
                                    .push(email_value)
                                    .push(apt_number_label)
                                    .push(apt_number_value)
                                    .width(iced::Length::Fill);

                                Scrollable::new(content)
                            },
                            EntryType::Cards => {
                                let title_label = Text::new("Title: ").size(16);
                                let title_value = Text::new(entry_data_edits["title"].to_string()).size(16);

                                let card_number_label = Text::new("Card Number: ").size(16);
                                let card_number_value = Text::new(entry_data_edits["card_number"].to_string()).size(16);

                                let cardholder_name_label = Text::new("Cardholder Name: ").size(16);
                                let cardholder_name_value = Text::new(entry_data_edits["cardholder_name"].to_string()).size(16);

                                let expiration_date_label = Text::new("Expiration Date: ").size(16);
                                let expiration_date_value = Text::new(entry_data_edits["expiration_date"].to_string()).size(16);

                                let security_code_label = Text::new("Security Code: ").size(16);
                                let security_code_value = Text::new(entry_data_edits["security_code"].to_string()).size(16);

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
                                let title = TextInput::new("Title", &entry_data_edits["title"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdatePasswordTitle);
                        
                                let url = TextInput::new("URL", &entry_data_edits["url"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdatePasswordUrl);
                        
                                let username = TextInput::new("Username", &entry_data_edits["username"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdatePasswordUsername);
                        
                                let password = TextInput::new("Password", &entry_data_edits["password"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdatePasswordPassword);
                        
                                let otpauth = TextInput::new("OTPAuth", &entry_data_edits["otpauth"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdatePasswordOtpAuth);
                        
                                //TODO bool
                                // let favorite = TextInput::new("Favorite", &entry_data["favorite"].as_str().unwrap_or(""))
                                //     .padding(8)
                                //     .on_input(Message::UpdatePasswordFavorite);
                        
                                let tags = TextInput::new("Tags", &entry_data_edits["tags"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdatePasswordTags);
                        
                                let notes = TextInput::new("Notes", &entry_data_edits["notes"].as_str().unwrap_or(""))
                                    .padding(8)
                                    .on_input(Message::UpdatePasswordNotes);
                        
                                Scrollable::new(
                                    Column::new()
                                        .push(title)
                                        .push(url)
                                        .push(username)
                                        .push(password)
                                        .push(otpauth)
                                        //TODO bool
                                        // .push(favorite)
                                        .push(tags)
                                        .push(notes)
                                        .width(iced::Length::Fill)
                                )
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
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::NavColumn),
                )).into()
        },
    }
}