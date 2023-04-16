use iced::{Application, Settings};

mod gui;
use crate::gui::types::keybolt_app::KeyboltApp;

use iced::window::{PlatformSpecific, Position};
use iced::window;
use gui::styles::style_constants::FONT_SIZE_BODY;


// Run application
fn main() -> Result<(), iced::Error> {
    // Start the application with default settings
    KeyboltApp::run(Settings {
        id: None,
        window: window::Settings {
            size: (1190, 670), // start size
            position: Position::Centered,
            min_size: Some((1190, 600)), // min size allowed
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: PlatformSpecific::default(),
        },
        flags: KeyboltApp::new(),
        default_font: Some(include_bytes!(
            "../resources/fonts/sarasa-mono-sc-regular.subset.ttf"
        )),
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: true,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}