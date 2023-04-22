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
pub fn view_page(style: StyleType, current_details_page_mode: PageMode, entry_type: EntryType, entry_id: i32, entries: &Value) -> Element<'static, Message> {
    let entry_data = &entries[entry_type.as_str()][entry_id as usize];
    match current_details_page_mode {
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
            let updated_entry_data = entry_data.clone(); // Clone the entry_data to make it mutable and independent from the original entries data

            let edit_toggle_btn_str = if current_details_page_mode == PageMode::Edit { "Save" } else { "Edit" };
            let mut edit_toggle_btn = Button::new(edit_toggle_btn_str);
            if current_details_page_mode == PageMode::Edit {
                edit_toggle_btn = edit_toggle_btn
                    .on_press(Message::UpdateEntry(entry_type, entry_id, updated_entry_data))
                    .on_press(Message::ChangeEntryMode(PageMode::View));
            } else {
                edit_toggle_btn = edit_toggle_btn.on_press(Message::ChangeEntryMode(PageMode::Edit));
            }

            

            // HEADER
            let header_row = Row::new()
                .push(close_btn)
                .push(keybolt_title)
                .push(edit_toggle_btn);

            // BODY
            let details_body = match entry_type {
                EntryType::Passwords => {
                    let title = TextInput::new("Title", &entry_data["title"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordTitle);
            
                    let url = TextInput::new("URL", &entry_data["url"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordUrl);
            
                    let username = TextInput::new("Username", &entry_data["username"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordUsername);
            
                    let password = TextInput::new("Password", &entry_data["password"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordPassword);
            
                    let otpauth = TextInput::new("OTPAuth", &entry_data["otpauth"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordOtpAuth);
            
                    let favorite = TextInput::new("Favorite", &entry_data["favorite"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordFavorite);
            
                    let tags = TextInput::new("Tags", &entry_data["tags"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordTags);
            
                    let notes = TextInput::new("Notes", &entry_data["notes"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdatePasswordNotes);
            
                    Scrollable::new(
                        Column::new()
                            .push(title)
                            .push(url)
                            .push(username)
                            .push(password)
                            .push(otpauth)
                            .push(favorite)
                            .push(tags)
                            .push(notes)
                            .width(iced::Length::Fill)
                    )
                },
                EntryType::Identities => {
                    let title = TextInput::new("Title", &entry_data["title"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityTitle);
            
                    let first_name = TextInput::new("First Name", &entry_data["first_name"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityFirstName);
            
                    let middle_initial = TextInput::new("Middle Initial", &entry_data["middle_initial"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityMiddleInitial);
            
                    let last_name = TextInput::new("Last Name", &entry_data["last_name"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityLastName);
            
                    let address = TextInput::new("Address", &entry_data["address"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityAddress);
            
                    let city = TextInput::new("City", &entry_data["city"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityCity);
            
                    let country = TextInput::new("Country", &entry_data["country"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityCountry);
            
                    let state = TextInput::new("State", &entry_data["state"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityState);
            
                    let zipcode = TextInput::new("Zipcode", &entry_data["zipcode"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityZipcode);
            
                    let phone = TextInput::new("Phone", &entry_data["phone"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityPhone);
            
                    let email = TextInput::new("Email", &entry_data["email"].as_str().unwrap_or(""))
                        .padding(8)
                        .on_input(Message::UpdateIdentityEmail);
            
                    let apt_number = TextInput::new("Apt Number", &entry_data["apt_number"].as_str().unwrap_or(""))
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
                let title = TextInput::new("Title", &entry_data["title"].as_str().unwrap_or(""))
                    .padding(8)
                    .on_input(Message::UpdateCardTitle);
        
                let name = TextInput::new("Name", &entry_data["name"].as_str().unwrap_or(""))
                    .padding(8)
                    .on_input(Message::UpdateCardName);
        
                let card_number = TextInput::new("Card Number", &entry_data["card_number"].as_str().unwrap_or(""))
                    .padding(8)
                    .on_input(Message::UpdateCardNumber);
        
                let card_last_four = TextInput::new("Card Last Four", &entry_data["card_last_four"].as_str().unwrap_or(""))
                    .padding(8)
                    .on_input(Message::UpdateCardLastFour);
        
                let expiration_date = TextInput::new("Expiration Date", &entry_data["expiration_date"].as_str().unwrap_or(""))
                    .padding(8)
                    .on_input(Message::UpdateCardExpirationDate);
        
                let security_code = TextInput::new("Security Code", &entry_data["security_code"].as_str().unwrap_or(""))
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
            },
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