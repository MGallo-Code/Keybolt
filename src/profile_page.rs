use iced::Length;
use iced::pure::widget::{Button, Column, Container, Text};
use iced::pure::Element;

// Import the Views enum and the AppMsg enum from the main application module
use crate::Views;
use crate::AppMsg;

// Define the ProfilePage struct
pub struct ProfilePage;

// Implement methods for the ProfilePage struct
impl ProfilePage {
    // Create a new instance of the ProfilePage struct
    pub fn new() -> Self {
        ProfilePage
    }

    // Define the user interface layout for the ProfilePage
    pub fn view(&self) -> Element<AppMsg> {
        // Create a text label for the ProfilePage
        let label = Text::new("Page 2");

        // Create a button to return to the main menu
        let main_menu_btn = Button::new("Main Menu").on_press(AppMsg::ChangePage(Views::MainPage));

        // Create a column layout, add the label and button to it
        let col = Column::new().push(label).push(main_menu_btn);

        // Create a container to hold the column layout, set its dimensions and position, and return it as an Element
        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}