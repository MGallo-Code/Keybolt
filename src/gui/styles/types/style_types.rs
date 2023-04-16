#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

pub enum ElementType {
    Default,
    NavActive,
    NavInactive,
}