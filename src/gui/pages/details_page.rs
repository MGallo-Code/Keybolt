use iced::widget::{Button, Column, Container, Row, Scrollable, Space, Text, TextInput, Toggler};
use iced::{Element, Length, Padding, Renderer};
use serde_json::Value;

use crate::gui::core::message::Message;
use crate::gui::styles::elements::button::ButtonStyle;
use crate::gui::styles::elements::container::ContainerStyle;
use crate::gui::styles::elements::text::TextStyle;
use crate::gui::styles::elements::text_input::TextInputStyle;
use crate::gui::styles::keybolt_theme::KeyboltTheme;

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
pub fn view_page(
    current_page_mode: PageMode,
    entry_type: EntryType,
    entry_data_edits: &Value,
) -> Element<'static, Message, Renderer<KeyboltTheme>> {
    match current_page_mode {
        PageMode::Closed => Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into(),
        _ => {
            // HEADER
            let close_btn = Button::new("Close")
                .on_press(Message::ChangeEntryMode(PageMode::Closed))
                .style(ButtonStyle::Secondary);

            let edit_toggle_btn_str = if current_page_mode == PageMode::Edit {
                "Save"
            } else {
                "Edit"
            };
            let mut edit_toggle_btn =
                Button::new(edit_toggle_btn_str).style(ButtonStyle::Secondary);
            if current_page_mode == PageMode::Edit {
                edit_toggle_btn = edit_toggle_btn.on_press(Message::SaveEntryEdits)
            } else {
                edit_toggle_btn =
                    edit_toggle_btn.on_press(Message::ChangeEntryMode(PageMode::Edit));
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
            let details_body = {
                let get_entry_str = |entry_name: &str| -> String {
                    match entry_data_edits[entry_name] {
                        Value::String(ref s) => s.to_string(),
                        _ => "".to_string(),
                    }
                };
                let create_text_input =
                    |placeholder: &str,
                     entry: &str,
                     msg: fn(String) -> Message|
                     -> TextInput<'static, Message, Renderer<KeyboltTheme>> {
                        match current_page_mode {
                            PageMode::View => TextInput::new(
                                placeholder,
                                &entry_data_edits[entry].as_str().unwrap_or(""),
                            )
                            .padding(8)
                            .style(TextInputStyle::Primary),
                            PageMode::Edit => TextInput::new(
                                placeholder,
                                &entry_data_edits[entry].as_str().unwrap_or(""),
                            )
                            .padding(8)
                            .on_input(msg)
                            .style(TextInputStyle::Primary),
                            _ => TextInput::new("", ""),
                        }
                    };

                match entry_type {
                    EntryType::Passwords => {
                        let mut title = Column::new();
                        if current_page_mode == PageMode::Edit {
                            title = title.push(
                                create_text_input("Title", "title", Message::UpdatePasswordTitle)
                                    .size(35)
                                    .width(iced::Length::Fill),
                            )
                        } else {
                            title = title.push(
                                Text::new(get_entry_str("title"))
                                    .size(35)
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .width(iced::Length::Fill)
                                    .style(TextStyle::DetailsTitle),
                            )
                        }

                        let favorite_value = Toggler::new(
                            String::from("Favorite"),
                            entry_data_edits["favorite"] == Value::Bool(true),
                            Message::UpdatePasswordFavorite,
                        );

                        let login_info = Container::new(
                            Column::new()
                                .push(
                                    Text::new("Username: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(create_text_input(
                                    "Username",
                                    "username",
                                    Message::UpdatePasswordUsername,
                                ))
                                .push(
                                    Text::new("Password: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(if current_page_mode == PageMode::Edit {
                                    create_text_input(
                                        "12345",
                                        "password",
                                        Message::UpdatePasswordPassword,
                                    )
                                } else {
                                    create_text_input(
                                        "12345",
                                        "password",
                                        Message::UpdatePasswordPassword,
                                    )
                                    .password()
                                }),
                        )
                        .padding(8)
                        .style(ContainerStyle::EntryInputContainer);

                        let mut otpauth_col = Column::new();
                        let otp_auth_exists = entry_data_edits["otpauth"].to_string().is_empty();
                        if otp_auth_exists {
                            otpauth_col = otpauth_col.push(
                                Text::new("OTP Authentication: ")
                                    .size(16)
                                    .style(TextStyle::EntryInputTitle),
                            );
                            otpauth_col = otpauth_col.push(create_text_input(
                                "OtpAuth",
                                "otpauth",
                                Message::UpdatePasswordOtpAuth,
                            ));
                        };
                        let mut otpauth_container =
                            Container::new(otpauth_col).style(ContainerStyle::EntryInputContainer);
                        if otp_auth_exists {
                            otpauth_container = otpauth_container.padding(8);
                        }

                        let website_info = Container::new(
                            Column::new()
                                .push(
                                    Text::new("Site URL: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(create_text_input(
                                    "https://www.example.com",
                                    "url",
                                    Message::UpdatePasswordUrl,
                                )),
                        )
                        .padding(8)
                        .style(ContainerStyle::EntryInputContainer);

                        let extra_info = Container::new(
                            Column::new()
                                .push(
                                    Text::new("Tags: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(create_text_input(
                                    "Tags here...",
                                    "tags",
                                    Message::UpdatePasswordTags,
                                ))
                                .push(
                                    Text::new("Notes: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(create_text_input(
                                    "Notes here...",
                                    "notes",
                                    Message::UpdatePasswordNotes,
                                )),
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
                    }
                    EntryType::Identities => {
                        let mut title = Column::new();
                        if current_page_mode == PageMode::Edit {
                            title = title.push(
                                create_text_input("Title", "title", Message::UpdateIdentityTitle)
                                    .size(35)
                                    .width(iced::Length::Fill),
                            )
                        } else {
                            title = title.push(
                                Text::new(get_entry_str("title"))
                                    .size(35)
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .width(iced::Length::Fill)
                                    .style(TextStyle::DetailsTitle),
                            )
                        }

                        let favorite_value = Toggler::new(
                            String::from("Favorite"),
                            entry_data_edits["favorite"] == Value::Bool(true),
                            Message::UpdateIdentityFavorite,
                        );

                        let name_container = Container::new(
                            Column::new()
                                .push(
                                    Text::new("Full Name: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            create_text_input(
                                                "First Name",
                                                "first_name",
                                                Message::UpdateIdentityFirstName,
                                            )
                                            .width(Length::FillPortion(3)),
                                        )
                                        .push(
                                            create_text_input(
                                                "M.I.",
                                                "middle_initial",
                                                Message::UpdateIdentityMiddleInitial,
                                            )
                                            .width(Length::Shrink)
                                            .width(Length::FillPortion(1)),
                                        )
                                        .push(
                                            create_text_input(
                                                "Last Name",
                                                "last_name",
                                                Message::UpdateIdentityLastName,
                                            )
                                            .width(Length::FillPortion(3)),
                                        ),
                                ),
                        )
                        .padding(8)
                        .style(ContainerStyle::EntryInputContainer);

                        let address_container = Container::new(
                            Column::new()
                                .push(
                                    Row::new()
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Address: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "123 Main St.",
                                                    "address",
                                                    Message::UpdateIdentityAddress,
                                                )),
                                        )
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Apt. Number: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "#",
                                                    "apt_number",
                                                    Message::UpdateIdentityAptNumber,
                                                )),
                                        ),
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("City: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "City",
                                                    "city",
                                                    Message::UpdateIdentityCity,
                                                ))
                                                .width(Length::FillPortion(6)),
                                        )
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Zipcode: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "#####",
                                                    "zipcode",
                                                    Message::UpdateIdentityZipcode,
                                                ))
                                                .width(Length::FillPortion(2)),
                                        )
                                        .width(Length::Fill),
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Country: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "Country",
                                                    "country",
                                                    Message::UpdateIdentityCountry,
                                                ))
                                                .width(Length::FillPortion(5)),
                                        )
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("State: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "State",
                                                    "state",
                                                    Message::UpdateIdentityState,
                                                ))
                                                .width(Length::Fill),
                                        )
                                        .width(Length::Fill),
                                ),
                        )
                        .style(ContainerStyle::EntryInputContainer)
                        .padding(8);

                        let contact_container = Container::new(
                            Column::new()
                                .push(
                                    Text::new("Phone: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(create_text_input(
                                    "xxxxxxxxxx",
                                    "phone",
                                    Message::UpdateIdentityPhone,
                                ))
                                .push(
                                    Text::new("Email: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(create_text_input(
                                    "example@email.com",
                                    "email",
                                    Message::UpdateIdentityEmail,
                                ))
                                .push(
                                    Text::new("Site: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(create_text_input(
                                    "https://www.example.com",
                                    "website",
                                    Message::UpdateIdentityWebsite,
                                )),
                        )
                        .style(ContainerStyle::EntryInputContainer)
                        .padding(8);

                        let birthday_container = Container::new(
                            Column::new()
                                .push(
                                    Text::new("Birthday: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            create_text_input(
                                                "Mon",
                                                "birth_month",
                                                Message::UpdateIdentityBirthMonth,
                                            )
                                            .width(Length::Fill),
                                        )
                                        .push(
                                            create_text_input(
                                                "Day",
                                                "birth_day",
                                                Message::UpdateIdentityBirthDay,
                                            )
                                            .width(Length::Fill),
                                        )
                                        .push(
                                            create_text_input(
                                                "Year",
                                                "birth_year",
                                                Message::UpdateIdentityBirthYear,
                                            )
                                            .width(Length::FillPortion(3)),
                                        ),
                                ),
                        )
                        .style(ContainerStyle::EntryInputContainer)
                        .padding(8);

                        let content = Column::new()
                            .spacing(10)
                            .push(title)
                            .push(favorite_value)
                            .push(name_container)
                            .push(address_container)
                            .push(contact_container)
                            .push(birthday_container)
                            .padding(15.0)
                            .width(iced::Length::Fill);

                        Scrollable::new(content)
                    }
                    EntryType::Cards => {
                        let mut title = Column::new();
                        if current_page_mode == PageMode::Edit {
                            title = title.push(
                                create_text_input("Title", "title", Message::UpdateCardTitle)
                                    .size(35)
                                    .width(iced::Length::Fill),
                            )
                        } else {
                            title = title.push(
                                Text::new(get_entry_str("title"))
                                    .size(35)
                                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                                    .width(iced::Length::Fill)
                                    .style(TextStyle::DetailsTitle),
                            )
                        }

                        let favorite_value = Toggler::new(
                            String::from("Favorite"),
                            entry_data_edits["favorite"] == Value::Bool(true),
                            Message::UpdateCardFavorite,
                        );

                        let mut card_number = Column::new();
                        if current_page_mode == PageMode::Edit {
                            card_number = card_number.push(
                                Text::new("Card Number: ")
                                    .size(16)
                                    .style(TextStyle::EntryInputTitle),
                            );
                            card_number = card_number.push(
                                create_text_input(
                                    "xxxxxxxxxxxxxxxx",
                                    "card_number",
                                    Message::UpdateCardNumber,
                                )
                                .size(16)
                                .width(iced::Length::Fill),
                            );
                        } else {
                            card_number = card_number.push(
                                Text::new("Card Number: ")
                                    .size(16)
                                    .style(TextStyle::EntryInputTitle),
                            );
                            card_number = card_number.push(
                                TextInput::new(
                                    "xxxxxxxxxxxxxxxx",
                                    &(String::from("****-****-****-")
                                        + &entry_data_edits["card_last_four"]
                                            .as_str()
                                            .unwrap_or("")
                                            .clone()
                                            .to_owned()),
                                )
                                .padding(8)
                                .style(TextInputStyle::Primary)
                                .width(iced::Length::Fill),
                            );
                        }

                        let card_container = Container::new(
                            Column::new()
                                .push(
                                    Text::new("Cardholder Name: ")
                                        .size(16)
                                        .style(TextStyle::EntryInputTitle),
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            create_text_input(
                                                "First Name",
                                                "first_name",
                                                Message::UpdateCardFirstName,
                                            )
                                            .width(Length::FillPortion(3)),
                                        )
                                        .push(
                                            create_text_input(
                                                "M.I.",
                                                "middle_initial",
                                                Message::UpdateCardMiddleInitial,
                                            )
                                            .width(Length::Shrink)
                                            .width(Length::FillPortion(1)),
                                        )
                                        .push(
                                            create_text_input(
                                                "Last Name",
                                                "last_name",
                                                Message::UpdateCardLastName,
                                            )
                                            .width(Length::FillPortion(3)),
                                        ),
                                )
                                .push(card_number)
                                .push(
                                    Column::new()
                                        .push(
                                            Row::new()
                                            .push(
                                                Text::new("Expiration Date: ")
                                                    .size(16)
                                                    .style(TextStyle::EntryInputTitle)
                                                    .width(Length::FillPortion(4))
                                            )
                                            .push(
                                                Text::new("Security Code: ")
                                                    .size(16)
                                                    .style(TextStyle::EntryInputTitle)
                                                    .width(Length::FillPortion(3))
                                            )
                                        )
                                        .push(
                                    Row::new()
                                        .push(
                                            create_text_input(
                                                "Month",
                                                "expiration_month",
                                                Message::UpdateCardExpirationMonth,
                                            )
                                            .width(Length::FillPortion(2)),
                                        )
                                        .push(
                                            create_text_input(
                                                "Year",
                                                "expiration_year",
                                                Message::UpdateCardExpirationYear,
                                            )
                                            .width(Length::Shrink)
                                            .width(Length::FillPortion(2)),
                                        )
                                        .push(
                                            create_text_input(
                                                "",
                                                "security_code",
                                                Message::UpdateCardSecurityCode,
                                            )
                                            .width(Length::FillPortion(3)),
                                        )),
                                ),
                        )
                        .padding(8)
                        .style(ContainerStyle::EntryInputContainer);

                        let address_container = Container::new(
                            Column::new()
                                .push(
                                    Row::new()
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Address: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "123 Main St.",
                                                    "address",
                                                    Message::UpdateCardAddress,
                                                )),
                                        )
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Apt. Number: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "#",
                                                    "apt_number",
                                                    Message::UpdateCardAptNumber,
                                                )),
                                        ),
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("City: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "City",
                                                    "city",
                                                    Message::UpdateCardCity,
                                                ))
                                                .width(Length::FillPortion(6)),
                                        )
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Zipcode: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "#####",
                                                    "zipcode",
                                                    Message::UpdateCardZipcode,
                                                ))
                                                .width(Length::FillPortion(2)),
                                        )
                                        .width(Length::Fill),
                                )
                                .push(
                                    Row::new()
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("Country: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "Country",
                                                    "country",
                                                    Message::UpdateCardCountry,
                                                ))
                                                .width(Length::FillPortion(5)),
                                        )
                                        .push(
                                            Column::new()
                                                .push(
                                                    Text::new("State: ")
                                                        .size(16)
                                                        .style(TextStyle::EntryInputTitle),
                                                )
                                                .push(create_text_input(
                                                    "State",
                                                    "state",
                                                    Message::UpdateCardState,
                                                ))
                                                .width(Length::Fill),
                                        )
                                        .width(Length::Fill),
                                ),
                        )
                        .style(ContainerStyle::EntryInputContainer)
                        .padding(8);

                        let content = Column::new()
                            .spacing(10)
                            .push(title)
                            .push(favorite_value)
                            .push(card_container)
                            .push(address_container)
                            .padding(15.0)
                            .width(iced::Length::Fill);
                        
                        Scrollable::new(content)
                    }
                }
            };

            // Create nav container
            Container::new(Column::new().push(header_row).push(details_body))
                .width(iced::Length::Fixed(300.0))
                .height(iced::Length::Fill)
                .into()
        }
    }
}
