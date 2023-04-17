use iced::Length;
use iced::alignment::Vertical;
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
    let label = Text::new("Profile Page!");
    let light_mode_btn = Button::new("Light Mode")
        .width(Length::Fixed(200.0))
        .on_press(Message::ChangeStyle(StyleType::Light))
        .style(<StyleTuple as Into<iced::theme::Button>>::into(
            StyleTuple(style, ElementType::Default),
        ));
    let dark_mode_btn = Button::new("Dark Mode")
        .width(Length::Fixed(200.0))
        .on_press(Message::ChangeStyle(StyleType::Dark))
        .style(<StyleTuple as Into<iced::theme::Button>>::into(
            StyleTuple(style, ElementType::Default),
        ));
    let cust_mode_btn = Button::new("Custom Mode")
        .width(Length::Fixed(200.0))
        .on_press(Message::ChangeStyle(StyleType::Dark))
        .style(<StyleTuple as Into<iced::theme::Button>>::into(
            StyleTuple(style, ElementType::Default),
        ));

    let col = Column::new()
        .push(label)
        .push(light_mode_btn)
        .push(dark_mode_btn)
        .push(cust_mode_btn);

    // Create a container to hold the column layout, set its dimensions and position, and return it as an Element
    Container::new(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Default),
        ))
        .into()
}