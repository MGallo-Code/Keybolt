use crate::gui::styles::types::style_type::StyleType;
use crate::gui::core::app::Pages;

#[derive(Debug, Clone)]
/// Messages types that permit to react to application interactions/subscriptions
pub enum Message {
    ChangePage(Pages),
    ChangeStyle(StyleType),
    PasswordInputChanged(String),
    PasswordInputSubmit,
}
