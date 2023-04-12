use iced::Settings;
use iced::pure::widget::{Button, Column, Container, Text};
use iced::pure::Sandbox;
use profile_page::ProfilePage;

mod profile_page;

fn main() -> Result<(), iced::Error> {
    PageView::run(Settings::default())
}

struct PageView {
    current_view: Views,
    profile_page: ProfilePage
}

#[derive(Debug, Clone, Copy)]
pub enum Views {
    MainPage,
    ProfilePage
}

#[derive(Debug, Clone, Copy)]
pub enum PageViewMsg {
    ChangePage(Views),
}

impl Sandbox for PageView {
    type Message = PageViewMsg;

    fn new() -> Self {
        PageView {
            current_view: Views::MainPage,
            profile_page: ProfilePage::new()
        }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            PageViewMsg::ChangePage(view) => self.current_view = view,
        }
    }

    fn view(&self) -> iced::pure::Element<Self::Message> {
        let label = Text::new(format!("View: {}", "Main"));
        let profile_btn = Button::new("Profile").on_press(PageViewMsg::ChangePage(Views::ProfilePage));
        let col = Column::new().push(label).push(profile_btn);
        let main_page_layout = Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill);

        match self.current_view {
            Views::MainPage => main_page_layout.into(),
            Views::ProfilePage => self.profile_page.view()
        }
    }
}