#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArgbColor {
    pub alpha: f32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbColor {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl Default for RgbColor {
    fn default() -> Self {
        Self {
            red: 0xFF,
            green: 0xFF,
            blue: 0xFF,
        }
    }
}

impl From<RgbColor> for u32 {
    fn from(color: RgbColor) -> u32 {
        (u32::from(color.red) << 16) | (u32::from(color.green) << 8) | u32::from(color.blue)
    }
}

impl ArgbColor {
    pub const fn new(alpha: f32, red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha: alpha.clamp(0.0, 1.0),
            red,
            green,
            blue,
        }
    }

    pub fn blend_with_background(self, background: ArgbColor) -> Self {
        let alpha = self.alpha + background.alpha * (1.0 - self.alpha);
        if alpha <= 0.0 {
            return Self::new(0.0, 0x00, 0x00, 0x00);
        }

        let red = ((self.red as f32 * self.alpha
            + background.red as f32 * background.alpha * (1.0 - self.alpha))
            / alpha)
            .round()
            .clamp(0.0, 255.0) as u8;

        let green = ((self.green as f32 * self.alpha
            + background.green as f32 * background.alpha * (1.0 - self.alpha))
            / alpha)
            .round()
            .clamp(0.0, 255.0) as u8;

        let blue = ((self.blue as f32 * self.alpha
            + background.blue as f32 * background.alpha * (1.0 - self.alpha))
            / alpha)
            .round()
            .clamp(0.0, 255.0) as u8;

        Self {
            alpha,
            red,
            green,
            blue,
        }
    }
}

impl Default for ArgbColor {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            red: 0xFF,
            green: 0xFF,
            blue: 0xFF,
        }
    }
}

impl From<ArgbColor> for u32 {
    fn from(color: ArgbColor) -> u32 {
        (u32::from((color.alpha * 255.0).round() as u8) << 24)
            | (u32::from(color.red) << 16)
            | (u32::from(color.green) << 8)
            | u32::from(color.blue)
    }
}

impl From<RgbColor> for ArgbColor {
    fn from(value: RgbColor) -> Self {
        Self {
            red: value.red,
            green: value.green,
            blue: value.blue,
            ..Default::default()
        }
    }
}
