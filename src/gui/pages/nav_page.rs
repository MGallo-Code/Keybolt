use iced::Length;
use iced::widget::{Column, Container, Text, Button};
use iced::Element;

use crate::gui::core::app::Pages;
// Import Message enum from the main application module
use crate::gui::core::message::Message;
use crate::gui::styles::style_constants::{
    FONT_SIZE_NAV,
    FONT_SIZE_NAV_TITLE,
    JOSEFIN_SANS_REG,
    RALEWAY_BOLD,
};
use crate::gui::styles::types::{
    style_type::StyleType,
    element_type::ElementType,
};

// Define the user interface layout for the ProfilePage
pub fn view_page(style: StyleType, current_page: Pages) -> Element<'static, Message> {
    let nav_btn = |label, page| {
        Button::new(
            Text::new(label)
                .font(RALEWAY_BOLD)
                .size(FONT_SIZE_NAV)
            )
            .width(Length::Fixed(200.0))
            .padding(10)
            .on_press(Message::ChangePage(page))
    };

    // Nav column
    let keybolt_title = Container::new(
        Text::new("Keybolt")
            .font(JOSEFIN_SANS_REG)
            .size(FONT_SIZE_NAV_TITLE)
        )
        .padding(15);
    let profile_page_btn = nav_btn("Profile", Pages::ProfilePage);
    let passwords_page_btn = nav_btn("Passwords", Pages::PasswordsPage);
    let identities_page_btn = nav_btn("Identities", Pages::IdentitiesPage);
    let cards_page_btn = nav_btn("Cards", Pages::CardsPage);

    // Create nav container
    Container::new(
        Column::new()
            .push(keybolt_title)
            .push(profile_page_btn)
            .push(passwords_page_btn)
            .push(identities_page_btn)
            .push(cards_page_btn)
    ).height(iced::Length::Fill)
    .into()
}