use iced::Length;
use iced::pure::widget::{Button, Column, Container, Text};
use iced::pure::Element;

// Import the Views enum and the AppMsg enum from the main application module
use crate::Views;
use crate::AppMsg;

// Define the PasswordsPage struct
pub struct PasswordsPage;

// Implement methods for the PasswordsPage struct
impl PasswordsPage {
    // Create a new instance of the PasswordsPage struct
    pub fn new() -> Self {
        PasswordsPage
    }

    // Define the user interface layout for the PasswordsPage
    pub fn view(&self) -> Element<AppMsg> {
        // Create a text label for the PasswordsPage
        let label = Text::new("Passwords page!");

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
}