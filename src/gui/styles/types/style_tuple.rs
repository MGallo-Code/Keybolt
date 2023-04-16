use crate::gui::styles::types::style_types::{
    ElementType,
    Theme,
};

/// This tuple permits to specify the correct style depending on the style type and on the element type
pub struct ThemeTuple(pub Theme, pub ElementType);

impl Clone for ThemeTuple {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
