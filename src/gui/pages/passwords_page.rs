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
pub fn view_page(style: StyleType) -> Element<'static, Message> {
    let list_item = |label, selected| {
        Button::new(label)
            .padding(25)
            .width(Length::Fill)
            .style(<StyleTuple as Into<iced::theme::Button>>::into(
                if selected {
                    StyleTuple(style, ElementType::SelectedItem)
                } else {
                    StyleTuple(style, ElementType::ItemListEntry)
                }
            ))
            .on_press(Message::ChangeStyle(StyleType::Default))
    };

    // Create a text label for the PasswordsPage
    let label = Text::new("Passwords page");
    let item1 = list_item("Item 1", false);
    let item2 = list_item("Item 2", true);
    let item3 = list_item("Item 3", false);
    let item4 = list_item("Item 4", false);

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