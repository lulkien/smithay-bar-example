use rusttype::{Scale, point};

use crate::configuration::global_font;

use super::{
    BoundingBox, Component,
    colors::{ArgbColor, RgbColor},
};

#[allow(unused)]
pub struct Label {
    pub text: String,
    pub fg_color: RgbColor,
    pub bg_color: Option<RgbColor>,
    pub font_size: u32,
}

#[allow(unused)]
impl Label {
    fn update(&mut self, text: &str) {
        self.text = text.to_owned();
    }
}

impl Component for Label {
    fn render(&self) -> (BoundingBox, Vec<Option<u32>>) {
        let scale = Scale::uniform(self.font_size as f32);
        let v_metrics = global_font().v_metrics(scale);
        let glyphs: Vec<_> = global_font()
            .layout(&self.text, scale, point(0.0, v_metrics.ascent))
            .collect();

        let width = glyphs
            .iter()
            .map(|g| g.pixel_bounding_box().map(|bb| bb.max.x).unwrap_or(0))
            .max()
            .unwrap_or(0)
            .max(1) as usize;

        let height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap)
            .ceil()
            .max(1.0) as usize;

        let mut buffer: Vec<Option<u32>> = vec![None; width * height];

        let color_argb: u32 = ArgbColor::from(self.fg_color).into();

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                        let idx = (y as usize * width + x as usize) as usize;
                        if v > 0.0 {
                            let alpha = (v * 255.0) as u32;
                            let pixel = (alpha << 24) | (color_argb & 0x00FFFFFF);
                            buffer[idx] = Some(pixel);
                        }
                    }
                });
            }
        }

        // return
        (
            BoundingBox {
                width: width as u32,
                height: height as u32,
            },
            buffer,
        )
    }
}
