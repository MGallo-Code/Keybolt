use serde::{Deserialize, Serialize};

// Specify application style
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
pub enum StyleType {
    Dark,
    Light,
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Dark
    }
}
