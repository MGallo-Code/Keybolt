use iced::Length;
use iced::pure::widget::{Button, Column, Container, Text};
use iced::pure::Element;

use crate::Views;
use crate::PageViewMsg;

pub struct ProfilePage;

impl ProfilePage {
    pub fn new() -> Self {
        ProfilePage
    }

    pub fn view(&self) -> Element<PageViewMsg> {
        let label = Text::new("Page 2");
        let main_menu_btn = Button::new("Main Menu").on_press(PageViewMsg::ChangePage(Views::MainPage));
        let col = Column::new().push(label).push(main_menu_btn);
        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}