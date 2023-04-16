use iced::{Application, Settings};

mod gui;
use crate::gui::keybolt::{KeyboltApp};


// Run application
fn main() -> Result<(), iced::Error> {
    // Start the application with default settings
    KeyboltApp::run(Settings::default())
}