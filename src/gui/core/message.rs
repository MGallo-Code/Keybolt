use crate::gui::pages::details_page::PageMode;
use crate::gui::core::app::Pages;
use crate::gui::styles::keybolt_theme::KeyboltTheme;

#[derive(Clone, Debug)]
/// Messages types that permit to react to application interactions/subscriptions
pub enum Message {
    ChangePage(Pages),
    ChangeStyle(KeyboltTheme),
    ChangeEntryMode(PageMode),
    SaveEntryEdits,
    SelectEntry(i32),
    PasswordInputChanged(String),
    PasswordInputSubmit,

    // Messages for updating password entries
    UpdatePasswordTitle(String),
    UpdatePasswordUrl(String),
    UpdatePasswordUsername(String),
    UpdatePasswordPassword(String),
    UpdatePasswordOtpAuth(String),
    UpdatePasswordFavorite(bool),
    UpdatePasswordTags(String),
    UpdatePasswordNotes(String),

    // Messages for updating identity entries
    UpdateIdentityTitle(String),
    UpdateIdentityFavorite(bool),
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
    UpdateIdentityWebsite(String),
    UpdateIdentityBirthMonth(String),
    UpdateIdentityBirthDay(String),
    UpdateIdentityBirthYear(String),

    // Messages for updating card entries
    UpdateCardTitle(String),
    UpdateCardFavorite(bool),
    UpdateCardName(String),
    UpdateCardNumber(String),
    UpdateCardLastFour(String),
    UpdateCardExpirationDate(String),
    UpdateCardSecurityCode(String),
}