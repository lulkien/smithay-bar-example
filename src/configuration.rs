use std::sync::OnceLock;

use rusttype::Font;

use crate::components::ArgbColor;

pub struct SimbarConfig {
    pub width_fallback: u32,
    pub width: Option<u32>,
    pub height: u32,
    pub primary_output: Option<&'static str>,
    pub frame_rate: u64,
}

pub struct ThemeConfig {
    pub background_color: ArgbColor,
}

// -------------------------------------------------------------------------------------------------------

pub const SIMBAR_CONFIG: SimbarConfig = SimbarConfig {
    width_fallback: 1920,
    width: None,
    height: 40,
    primary_output: None,
    frame_rate: 60,
};

pub const THEME_CONFIG: ThemeConfig = ThemeConfig {
    background_color: ArgbColor::new(1.0, 0x11, 0x11, 0x18),
};

static GLOBAL_FONT: OnceLock<Font<'static>> = OnceLock::new();

pub fn global_font() -> &'static Font<'static> {
    GLOBAL_FONT.get_or_init(|| {
        let font_data = include_bytes!("../resources/fonts/CodeNewRomanNerdFontMono-Regular.otf");
        Font::try_from_bytes(font_data).expect("Failed to parse font")
    })
}
