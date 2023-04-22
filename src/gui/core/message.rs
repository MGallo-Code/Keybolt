use serde_json::Value;

use crate::gui::pages::details_page::{PageMode, EntryType};
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::core::app::Pages;

#[derive(Clone, Debug)]
/// Messages types that permit to react to application interactions/subscriptions
pub enum Message {
    ChangePage(Pages),
    ChangeStyle(StyleType),
    ChangeEntryMode(PageMode),
    UpdateEntry(EntryType, i32, Value),
    SelectEntry(i32),
    PasswordInputChanged(String),
    PasswordInputSubmit,

    // Messages for updating password entries
    UpdatePasswordTitle(String),
    UpdatePasswordUrl(String),
    UpdatePasswordUsername(String),
    UpdatePasswordPassword(String),
    UpdatePasswordOtpAuth(String),
    UpdatePasswordFavorite(String),
    UpdatePasswordTags(String),
    UpdatePasswordNotes(String),

    // Messages for updating identity entries
    UpdateIdentityTitle(String),
    UpdateIdentityFirstName(String),
    UpdateIdentityMiddleInitial(String),
    UpdateIdentityLastName(String),
    UpdateIdentityAddress(String),
    UpdateIdentityCity(String),
    UpdateIdentityCountry(String),
    UpdateIdentityState(String),
    UpdateIdentityZipcode(String),
    UpdateIdentityPhone(String),
    UpdateIdentityEmail(String),
    UpdateIdentityAptNumber(String),

    // Messages for updating card entries
    UpdateCardTitle(String),
    UpdateCardName(String),
    UpdateCardNumber(String),
    UpdateCardLastFour(String),
    UpdateCardExpirationDate(String),
    UpdateCardSecurityCode(String),
}