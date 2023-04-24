use iced::{Font};

pub const JOSEFIN_SANS_REG: Font = Font::External {
    name: "josefin-sans-regular",
    bytes: include_bytes!("../../../resources/fonts/JosefinSans-Regular.ttf"),
};

// pub const RALEWAY_REG: Font = Font::External {
//     name: "raleway_reg",
//     bytes: include_bytes!("../../../resources/fonts/Raleway-Regular.ttf"),
// };

pub const RALEWAY_BOLD: Font = Font::External {
    name: "raleway_bold",
    bytes: include_bytes!("../../../resources/fonts/Raleway-Bold.ttf"),
};

// font sizes
pub const FONT_SIZE_NAV: f32 = 25.0;
pub const FONT_SIZE_NAV_TITLE: f32 = 27.0;
pub const FONT_SIZE_BODY: f32 = 21.0;