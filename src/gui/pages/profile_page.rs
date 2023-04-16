use iced::Length;
use iced::widget::{Column, Container, Text};
use iced::Element;

// Import AppMsg enum from the main application module
use crate::{AppMsg, KeyboltApp};

// Define the user interface layout for the ProfilePage
pub fn view_page(_keybolt: &KeyboltApp) -> Element<AppMsg> {
    // Create a text label for the ProfilePage
    let label = Text::new("Profile Page!");

    // Create a column layout, add the label and button to it
    let col = Column::new().push(label);

    // Create a container to hold the column layout, set its dimensions and position, and return it as an Element
    Container::new(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}