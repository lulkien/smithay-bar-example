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
    fn render(&self) -> (BoundingBox, Vec<Option<ArgbColor>>) {
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

        let mut buffer: Vec<Option<ArgbColor>> = vec![None; width * height];

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, alpha| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                        let idx = (y as usize * width + x as usize) as usize;
                        if alpha > 0.0 {
                            let pixel =
                                ArgbColor::default().set_rgb(self.fg_color).set_alpha(alpha);

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
