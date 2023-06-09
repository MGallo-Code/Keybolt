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

// Define the user interface layout for the ProfilePage
pub fn view_page(style: StyleType) -> Element<'static, Message> {
    // Create a text label for the ProfilePage
    let label = Text::new("Profile Page");
    let fjord_mode_btn = Button::new("Fjord Mode")
        .width(Length::Fixed(200.0))
        .on_press(Message::ChangeStyle(StyleType::Fjord))
        .style(<StyleTuple as Into<iced::theme::Button>>::into(
            StyleTuple(style, ElementType::Default),
        ));
    let vibrant_mode_btn = Button::new("Vibrant Mode")
        .width(Length::Fixed(200.0))
        .on_press(Message::ChangeStyle(StyleType::Vibrant))
        .style(<StyleTuple as Into<iced::theme::Button>>::into(
            StyleTuple(style, ElementType::Default),
        ));
    let dark_mode_btn = Button::new("Dark Mode")
        .width(Length::Fixed(200.0))
        .on_press(Message::ChangeStyle(StyleType::Dark))
        .style(<StyleTuple as Into<iced::theme::Button>>::into(
            StyleTuple(style, ElementType::Default),
        ));
    let default_mode_btn = Button::new("Default Mode")
        .width(Length::Fixed(200.0))
        .on_press(Message::ChangeStyle(StyleType::Default))
        .style(<StyleTuple as Into<iced::theme::Button>>::into(
            StyleTuple(style, ElementType::Default),
        ));

    let col = Column::new()
        .push(label)
        .push(fjord_mode_btn)
        .push(vibrant_mode_btn)
        .push(dark_mode_btn)
        .push(default_mode_btn);

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