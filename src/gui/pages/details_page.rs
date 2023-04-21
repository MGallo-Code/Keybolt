use iced::{widget::{Container, Column, Text, Space, Button}, Element, Length};

use crate::gui::{styles::types::{element_type::ElementType, style_tuple::StyleTuple, style_type::StyleType}, core::{message::Message, app::Pages}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetailsPageMode {
    View,
    Edit,
    Closed,
}

#[derive(Debug, Clone, Copy)]
pub enum DetailsPageChange {
    ChangeMode(DetailsPageMode),
    SavePage,
    ClosePage,
}

// Define the user interface layout for the ProfilePage
pub fn view_page(style: StyleType, current_details_page_mode: DetailsPageMode, selected_event_id: i8) -> Element<'static, Message> {
    match current_details_page_mode {
        DetailsPageMode::Closed => {
            Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
        },
        _ => {
            // Nav column
            let keybolt_title = Container::new(
                Text::new("Details Window")
                )
                .padding(15)
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::NavHeader),
                ));
            
            let close_btn = Button::new("Close")
                .on_press(Message::ChangeDetailsPage(DetailsPageChange::ClosePage));
            let edit_toggle_btn_str = if current_details_page_mode == DetailsPageMode::Edit { "Save" } else { "Edit" };
            let mut edit_toggle_btn = Button::new(edit_toggle_btn_str);
            if current_details_page_mode == DetailsPageMode::Edit {
                edit_toggle_btn = edit_toggle_btn.on_press(Message::ChangeDetailsPage(DetailsPageChange::SavePage));
            } else {
                edit_toggle_btn = edit_toggle_btn.on_press(Message::ChangeDetailsPage(DetailsPageChange::ChangeMode(DetailsPageMode::Edit)));
            };

            // Create nav container
            Container::new(
                Column::new()
                    .push(keybolt_title)
                    .push(close_btn)
                    .push(edit_toggle_btn)
            ).height(iced::Length::Fill)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(style, ElementType::NavColumn),
            )).into()
        },
    }
}