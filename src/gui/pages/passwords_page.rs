use iced::Length;
use iced::widget::{Column, Container, Text, Button};
use iced::Element;

// Import Message enum from the main application module
use crate::gui::core::message::Message;
use crate::gui::styles::types::{
    style_type::StyleType,
    style_tuple::StyleTuple,
    element_type::ElementType,
};

// Define the user interface layout for the PasswordsPage
pub fn view_page(style: StyleType, selected_entry_id: i8) -> Element<'static, Message> {
    // Create a text label for the PasswordsPage
    let label = Text::new("Passwords page");
    let password_entry = |label, username, entry_id| {
        Button::new(
            Column::new()
                .push(Text::new(label))
                .push(Text::new(username))
        )
            .padding(25)
            .width(Length::Fill)
            .style(<StyleTuple as Into<iced::theme::Button>>::into(
                if entry_id == selected_entry_id {
                    StyleTuple(style, ElementType::SelectedItem)
                } else {
                    StyleTuple(style, ElementType::ItemListEntry)
                }
            ))
            .on_press(Message::SelectEntry(selected_entry_id))
    };
    let item1 = password_entry("Item 1", "1", 0);
    let item2 = password_entry("Item 2", "2", 1);
    let item3 = password_entry("Item 3", "3", 2);
    let item4 = password_entry("Item 4", "4", 3);

    // Create a column layout, add the label and button to it
    let col = Column::new()
        .push(label)
        .push(item1)
        .push(item2)
        .push(item3)
        .push(item4);

    // Create a container to hold the column layout, set its dimensions and position, and return it as an Element
    Container::new(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::ItemListColumn),
        ))
        .into()
}