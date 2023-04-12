use iced::Settings;
use iced::pure::widget::{Button, Column, Container, Text};
use iced::pure::Sandbox;
use profile_page::ProfilePage;

// Include each page
mod profile_page;

// Run application
fn main() -> Result<(), iced::Error> {
    // Start the application with default settings
    AppView::run(Settings::default())
}

// The main struct representing the application's state and its views
struct AppView {
    current_view: Views,
    profile_page: ProfilePage,
}

// An enumeration of the different views in the application
#[derive(Debug, Clone, Copy)]
pub enum Views {
    MainPage,
    ProfilePage,
}

// An enumeration of the messages the application can receive
#[derive(Debug, Clone, Copy)]
pub enum AppMsg {
    ChangePage(Views),
}

// Implement the Sandbox trait for the AppView struct
impl Sandbox for AppView {
    type Message = AppMsg;

    // Initialize a new AppView struct with default values
    fn new() -> Self {
        AppView {
            current_view: Views::MainPage,
            profile_page: ProfilePage::new(),
        }
    }

    // Return the title of the application
    fn title(&self) -> String {
        String::from("Counter app")
    }

    // Update the application's state based on received messages
    fn update(&mut self, message: Self::Message) {
        match message {
            AppMsg::ChangePage(view) => self.current_view = view,
        }
    }

    // Define the application's user interface layout based on its state
    fn view(&self) -> iced::pure::Element<Self::Message> {
        let label = Text::new(format!("View: {}", "Main"));
        let profile_btn = Button::new("Profile").on_press(AppMsg::ChangePage(Views::ProfilePage));
        let col = Column::new().push(label).push(profile_btn);
        let main_page_layout = Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill);

        // Display the appropriate view based on the current_view value
        match self.current_view {
            Views::MainPage => main_page_layout.into(),
            Views::ProfilePage => self.profile_page.view(),
        }
    }
}
