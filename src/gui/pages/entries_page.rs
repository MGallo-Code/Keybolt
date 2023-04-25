use iced::{Length, Renderer, Element};
use iced::widget::{Column, Container, Text, TextInput, Button, Scrollable};
use serde_json::Value;

// Import Message enum from the main application module
use crate::gui::core::message::Message;
use crate::gui::styles::elements::button::ButtonStyle;
use crate::gui::styles::elements::container::ContainerStyle;
use crate::gui::styles::keybolt_theme::KeyboltTheme;

use super::details_page::EntryType;

// Define the user interface layout for the PasswordsPage
pub fn view_page(entries: &Value, entry_type: EntryType, selected_entry_id: i32) -> Element<'static, Message, Renderer<KeyboltTheme>> {

    let search_bar = TextInput::new("Search", "");
    // Create a column layout, add the label and button to it
    let mut col = Column::new();
    let get_val = |value: &Value, label: &str| {
        let s = serde_json::to_string(&value[label]).unwrap();
        s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).map(|s| s.to_owned()).unwrap_or_else(|| s.to_owned())
    };
    let mut counter = 0;
    match entry_type {
        EntryType::Passwords => {
            // Create a text label for the PasswordsPage
            let password_entry = |entry_id: i32, label: String, username: String| {
                Button::new(
                    Column::new()
                        .push(Text::new(label))
                        .push(Text::new(username))
                )
                    .padding(25)
                    .width(Length::Fill)
                    .on_press(Message::SelectEntry(entry_id))
                    .style(ButtonStyle::EntryListButton(entry_id == selected_entry_id))
            };
            if let Some(obj) = entries.as_array() {
                for value in obj {
                    col = col.push(
                        password_entry(counter,
                            get_val(value, "title"),
                            get_val(value, "username"),
                        ),
                    );
                    counter += 1;
                }
            }
        },
        EntryType::Identities => {
            let identity_entry = |entry_id: i32, title: String, first_name: String, last_name: String, email: String| {
                Button::new(
                    Column::new()
                        .push(Text::new(title))
                        .push(Text::new(format!("{} {}", first_name, last_name)))
                        .push(Text::new(email))
                )
                    .padding(25)
                    .width(Length::Fill)
                    .on_press(Message::SelectEntry(entry_id))
                    .style(ButtonStyle::EntryListButton(entry_id == selected_entry_id))
            };
            if let Some(obj) = entries.as_array() {
                for value in obj {
                    col = col.push(
                        identity_entry(counter,
                            get_val(value, "title"),
                            get_val(value, "first_name"),
                            get_val(value, "last_name"),
                            get_val(value, "phone"),
                        )
                    );
                    counter += 1;
                }
            }
        },
        EntryType::Cards => {
            let card_entry = |entry_id: i32, title: String, name: String, card_last_four: String| {
                Button::new(
                    Column::new()
                        .push(Text::new(title))
                        .push(Text::new(name))
                        .push(Text::new(format!("****-****-****-{}", card_last_four)))
                )
                    .padding(25)
                    .width(Length::Fill)
                    .on_press(Message::SelectEntry(entry_id))
                    .style(ButtonStyle::EntryListButton(entry_id == selected_entry_id))
            };
            if let Some(obj) = entries.as_array() {
                for value in obj {
                    col = col.push(
                        card_entry(counter,
                            get_val(value, "title"),
                            get_val(value, "name"),
                            get_val(value, "card_last_four"),
                        ),
                    );
                    counter += 1;
                }
            }
        },
    }

    let scroll_area = Scrollable::new(col);

    let content = Column::new()
        .push(search_bar)
        .push(scroll_area);

    // Create a container to hold the column layout, set its dimensions and position, and return it as an Element
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .style(ContainerStyle::Secondary)
        .into()
}