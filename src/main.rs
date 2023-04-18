use iced::{Application, Settings};
use iced::window::{self, PlatformSpecific, Position};

mod secure;
mod gui;
use gui::core::app::KeyboltApp;
use gui::styles::style_constants::FONT_SIZE_BODY;

// Run application
fn main() -> Result<(), iced::Error> {
    // Start the application with default settings
    KeyboltApp::run(Settings {
        id: None,
        window: window::Settings {
            size: (1000, 670),
            position: Position::Centered,
            min_size: Some((800, 600)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: PlatformSpecific::default(),
        },
        flags: (),
        default_font: Some(include_bytes!(
            "../resources/fonts/Raleway-Regular.ttf"
        )),
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: true,
        antialiasing: false,

        exit_on_close_request: true,
        try_opengles_first: false,
    })
}