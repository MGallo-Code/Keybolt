use iced::{Length, Renderer, Element};
use iced::widget::{Column, Container, Text, Button, Scrollable};
use serde_json::Value;

// Import Message enum from the main application module
use crate::gui::core::message::Message;
use crate::gui::styles::keybolt_theme::KeyboltTheme;

// Define the user interface layout for the IdentitiesPage
pub fn view_page(entries: &Value, selected_entry_id: i32) -> Element<'static, Message, Renderer<KeyboltTheme>> {
    // Create a text label for the IdentitiesPage
    let label = Text::new("Identities page");

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
    };

    // Create a column layout, add the label and button to it
    let mut col = Column::new().push(label);
    let get_val = |value: &Value, label: &str| {
        let s = serde_json::to_string(&value[label]).unwrap();
        s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).map(|s| s.to_owned()).unwrap_or_else(|| s.to_owned())
    };
    let mut counter: i32 = 0;
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

    let scroll_area = Scrollable::new(col);

    // Create a container to hold the column layout, set its dimensions and position, and return it as an Element
    Container::new(scroll_area)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}