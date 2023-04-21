use crate::gui::pages::details_page::DetailsPageChange;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::core::app::Pages;

#[derive(Debug, Clone)]
/// Messages types that permit to react to application interactions/subscriptions
pub enum Message {
    ChangePage(Pages),
    ChangeStyle(StyleType),
    ChangeDetailsPage(DetailsPageChange),
    SelectEntry(i8),
    PasswordInputChanged(String),
    PasswordInputSubmit,
}
