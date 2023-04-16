use iced::Length;
use iced::alignment::Vertical;
use iced::widget::{Column, Container, Text};
use iced::Element;

// Import Message enum from the main application module
use crate::gui::keybolt::{Message};
use crate::gui::styles::types::{
    style_type::StyleType,
    style_tuple::StyleTuple,
    element_type::ElementType,
};

// Define the user interface layout for the ProfilePage
pub fn view_page(style: StyleType) -> Element<'static, Message> {
    // Create a text label for the ProfilePage
    let label = Text::new("Profile Page!");

    // Create a container to hold the column layout, set its dimensions and position, and return it as an Element
    Container::new(
        Column::new().push(label),
    )
    .height(Length::Fixed(95.0))
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Standard),
    )).into()
}