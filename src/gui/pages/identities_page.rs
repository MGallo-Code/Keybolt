use iced::Length;
use iced::widget::{Column, Container, Text};
use iced::Element;

// Import Message enum from the main application module
use crate::gui::core::message::Message;
use crate::gui::styles::types::{
    style_type::StyleType,
    style_tuple::StyleTuple,
    element_type::ElementType,
};

// Define the user interface layout for the IdentitiesPage
pub fn view_page(style: StyleType) -> Element<'static, Message> {
    // Create a text label for the IdentitiesPage
    let label = Text::new("Identities page");

    // Create a column layout, add the label and button to it
    let col = Column::new().push(label);

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