use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_type::StyleType;

/// This tuple permits to specify the correct style depending on the style type and on the element type
pub struct StyleTuple(pub StyleType, pub ElementType);

impl Clone for StyleTuple {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
